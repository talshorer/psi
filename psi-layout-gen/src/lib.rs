use std::rc::Rc;

use psi_svg::{Segment, UnitSquarePoint};
use psi_topology::layout::{Boundary, Layout, LayoutData, ValidatedLayoutData};
use rand::{Rng as _, SeedableRng as _, seq::SliceRandom};
use rand_chacha::ChaCha20Rng;

mod scatter_debug;

struct Rng {
    inner: ChaCha20Rng,
}

impl Rng {
    const RESOLUTION: u64 = 0xffff_ffff_ffff_ffff;

    fn new(seed: u64) -> Self {
        Self {
            inner: ChaCha20Rng::seed_from_u64(seed),
        }
    }

    fn weighted<'a, T, I>(&mut self, mut choices: I) -> Option<T>
    where
        T: Clone + 'static,
        I: Clone + Iterator<Item = &'a WeightedOption<T>>,
    {
        let total_weight = choices.clone().map(|o| o.weight).sum::<u64>();
        let mut idx = self.next_u64().checked_rem(total_weight)?;
        choices
            .find(|o| {
                if idx < o.weight {
                    true
                } else {
                    idx -= o.weight;
                    false
                }
            })
            .map(|w| &w.value)
            .cloned()
    }

    fn next_u64(&mut self) -> u64 {
        self.inner.next_u64()
    }

    fn next_f64(&mut self) -> f64 {
        (self.next_u64() % Self::RESOLUTION) as f64 / Self::RESOLUTION as f64
    }
}

enum ScatterEdge {
    First,
    Mid,
    Last,
}

#[derive(Default)]
struct Scatter {
    lands: Vec<UnitSquarePoint>,
    links: Vec<(usize, usize)>,
    legal_links: Vec<WeightedOption<(usize, usize)>>,
}

impl Scatter {
    const WEIGHT_RESOLUTION: f64 = 0xffff_ffffu64 as f64;

    fn link_weight(magnitude: f64) -> u64 {
        (Self::WEIGHT_RESOLUTION / (magnitude * magnitude)) as u64
    }

    fn push(&mut self, land: UnitSquarePoint) -> usize {
        let idx = self.lands.len();
        self.lands.push(land);
        for i in 0..idx {
            self.legal_links.push(WeightedOption {
                value: (i, idx),
                weight: Self::link_weight((land - self.lands[i]).length()),
            });
        }
        idx
    }

    fn remove_legal_link(&mut self, link: (usize, usize)) {
        if let Some(idx) = self.legal_links.iter().position(|o| o.value == link) {
            self.legal_links.swap_remove(idx);
        }
    }

    fn push_edge(
        &mut self,
        edge: ScatterEdge,
        boundaries: &[usize],
        pos: fn(f64) -> UnitSquarePoint,
    ) {
        let last = self.lands.len().checked_sub(1);
        let lands = matches!(edge, ScatterEdge::First)
            .then_some(0.0)
            .into_iter()
            .chain(
                boundaries
                    .iter()
                    .take(boundaries.len() - 1)
                    .zip(boundaries.iter().cycle().skip(1))
                    .map(|(b1, b2)| (*b1 + *b2) as f64 / 2.0 / (Boundary::MAX + 1) as f64),
            )
            .chain((!matches!(edge, ScatterEdge::Last)).then_some(1.0))
            .map(pos)
            .collect::<Vec<_>>();
        let mut prev = Vec::from_iter(
            (!matches!(edge, ScatterEdge::First))
                .then_some(last)
                .flatten(),
        );
        for land in lands {
            let idx = self.push(land);
            if let Some(last) = prev.last().copied() {
                self.links.push((last, idx));
            }
            for prev in &prev {
                self.remove_legal_link((*prev, idx));
            }
            prev.push(idx);
        }
        if matches!(edge, ScatterEdge::Last)
            && let Some(last) = prev.last().copied()
        {
            self.links.push((0, last));
            for prev in &prev {
                self.remove_legal_link((0, *prev));
            }
        }
    }

    fn push_random(&mut self, rng: &mut Rng) {
        self.push(UnitSquarePoint::new(rng.next_f64(), rng.next_f64()));
    }

    fn link_random(&mut self, rng: &mut Rng, max_dist: f64) -> bool {
        let min_weight = Self::link_weight(max_dist);
        let Some((idx1, idx2)) =
            rng.weighted(self.legal_links.iter().filter(|w| w.weight > min_weight))
        else {
            return false;
        };
        self.remove_legal_link((idx1, idx2));
        let seg1 = Segment(self.lands[idx1], self.lands[idx2]);
        if !self
            .links
            .iter()
            .cloned()
            .any(|(idx3, idx4)| seg1.crosses(&Segment(self.lands[idx3], self.lands[idx4])))
        {
            self.links.push((idx1, idx2));
        };
        true
    }
}

#[derive(Debug)]
struct GeneratedLayout {
    envelope: Vec<usize>,
    boundaries: (Vec<usize>, Vec<usize>, Vec<usize>),
    internal_adjacencies: Vec<(usize, usize)>,
    terrains: Vec<char>,
    coastals: Vec<usize>,
}

impl GeneratedLayout {
    pub fn layout(&self) -> Rc<Layout> {
        ValidatedLayoutData::validate(&LayoutData {
            envelope: &self.envelope,
            boundaries: (&self.boundaries.0, &self.boundaries.1, &self.boundaries.2),
            internal_adjacencies: &self.internal_adjacencies,
            terrains: &self.terrains,
            coastals: &self.coastals,
        })
        .unwrap()
        .layout()
    }
}

pub struct WeightedOption<T> {
    pub value: T,
    pub weight: u64,
}

pub struct LayoutGeneratorBalancedWeights {
    pub envelope_count: Vec<WeightedOption<usize>>,
    pub edge_clock3_count: Vec<WeightedOption<usize>>,
    pub edge_clock6_count: Vec<WeightedOption<usize>>,
}

pub struct LayoutGeneratorUnbalancedWeights {
    pub coastal_count: Vec<WeightedOption<usize>>,
    pub land_count: Vec<WeightedOption<usize>>,
    pub edge_land_count: Vec<WeightedOption<usize>>,
    pub terrain: Vec<WeightedOption<char>>,
}

pub enum LayoutGeneratorBalance {
    Balanced(LayoutGeneratorBalancedWeights),
    Unbalanced(LayoutGeneratorUnbalancedWeights),
}

impl LayoutGeneratorBalance {
    fn weighted<T>(rng: &mut Rng, choices: &[WeightedOption<T>]) -> T
    where
        T: Clone + 'static,
    {
        rng.weighted(choices.iter()).unwrap()
    }

    fn coastal_count(&self, rng: &mut Rng) -> usize {
        match self {
            LayoutGeneratorBalance::Balanced(_) => 3,
            LayoutGeneratorBalance::Unbalanced(weights) => {
                Self::weighted(rng, &weights.coastal_count)
            }
        }
    }

    fn land_count(&self, rng: &mut Rng) -> usize {
        match self {
            LayoutGeneratorBalance::Balanced(_) => 8,
            LayoutGeneratorBalance::Unbalanced(weights) => Self::weighted(rng, &weights.land_count),
        }
    }

    fn envelope_counts(&self, rng: &mut Rng) -> [usize; 3] {
        match self {
            LayoutGeneratorBalance::Balanced(weights) => {
                let total = Self::weighted(rng, &weights.envelope_count);
                let clock3 = Self::weighted(rng, &weights.edge_clock3_count);
                let clock6 = Self::weighted(rng, &weights.edge_clock6_count);
                let clock9 = total + 2 - clock3 - clock6;
                [clock3, clock6, clock9]
            }
            LayoutGeneratorBalance::Unbalanced(weights) => [
                Self::weighted(rng, &weights.edge_land_count),
                Self::weighted(rng, &weights.edge_land_count),
                Self::weighted(rng, &weights.edge_land_count),
            ],
        }
    }

    fn terrains(&self, count: usize, rng: &mut Rng) -> Vec<char> {
        let mut terrains = match self {
            LayoutGeneratorBalance::Balanced(_) => {
                assert_eq!(count, 8);
                vec!['M', 'M', 'J', 'J', 'S', 'S', 'W', 'W']
            }
            LayoutGeneratorBalance::Unbalanced(weights) => (0..count)
                .map(|_| Self::weighted(rng, &weights.terrain))
                .collect(),
        };
        terrains.shuffle(&mut rng.inner);
        terrains
    }
}

pub struct LayoutGeneratorConfig {
    pub balance: LayoutGeneratorBalance,
    pub dist_step: f64,
}

pub struct LayoutGenerator {
    rng: Rng,
    config: LayoutGeneratorConfig,
    envelope: Vec<usize>,
    boundaries: [Vec<usize>; 3],
}

impl LayoutGenerator {
    pub fn new(seed: u64, config: LayoutGeneratorConfig) -> Self {
        Self {
            rng: Rng::new(seed),
            config,
            envelope: Vec::new(),
            boundaries: [Vec::new(), Vec::new(), Vec::new()],
        }
    }

    fn push_edge(
        &mut self,
        scatter: &mut Scatter,
        envelope_counts: &[usize],
        edge: usize,
        last: bool,
        pos: fn(f64) -> UnitSquarePoint,
    ) {
        let count = envelope_counts[edge];
        self.boundaries[edge] = self.boundaries(count);
        let new_first = scatter.lands.len();
        let new_last = new_first + count - 1;
        self.envelope.extend(new_first..new_last);
        let scatter_edge = if last {
            ScatterEdge::Last
        } else {
            ScatterEdge::Mid
        };
        scatter.push_edge(scatter_edge, &self.boundaries[edge], pos);
    }

    fn boundaries(&mut self, count: usize) -> Vec<usize> {
        let mut options = (1..=6).collect::<Vec<_>>();
        let mut result = (0..(count - 1))
            .map(|_| options.swap_remove(self.rng.next_u64() as usize % options.len()))
            .collect::<Vec<_>>();
        result.sort();
        result
    }

    pub fn generate_layout(&mut self) -> Rc<Layout> {
        let mut scatter = Scatter::default();
        let coastals = self.config.balance.coastal_count(&mut self.rng);
        scatter.push_edge(ScatterEdge::First, &self.boundaries(coastals), |v| {
            UnitSquarePoint::new(0.0, v)
        });
        let envelope_counts = self.config.balance.envelope_counts(&mut self.rng);
        self.push_edge(&mut scatter, &envelope_counts, 0, false, |v| {
            UnitSquarePoint::new(v, 1.0)
        });
        self.push_edge(&mut scatter, &envelope_counts, 1, false, |v| {
            UnitSquarePoint::new(1.0, 1.0 - v)
        });
        self.push_edge(&mut scatter, &envelope_counts, 2, true, |v| {
            UnitSquarePoint::new(1.0 - v, 0.0)
        });
        self.envelope.push(1);
        let nr_lands = self
            .config
            .balance
            .land_count(&mut self.rng)
            .max(scatter.lands.len());
        for _ in 0..(nr_lands - scatter.lands.len()) {
            scatter.push_random(&mut self.rng);
        }
        for max_dist in (0..((f64::sqrt(2.0) / self.config.dist_step).ceil() as u64))
            .map(|i| (i + 1) as f64 * self.config.dist_step)
            .chain(std::iter::once(2.0))
        {
            while scatter.link_random(&mut self.rng, max_dist) {}
        }
        if let Ok(scatter_debug) = std::env::var("PLG_SCATTER_DEBUG") {
            let _ = std::fs::write(scatter_debug, scatter_debug::generate_svg(&scatter));
        }

        let terrains = self.config.balance.terrains(nr_lands, &mut self.rng);

        GeneratedLayout {
            envelope: self.envelope.clone(),
            boundaries: (
                self.boundaries[0].clone(),
                self.boundaries[1].clone(),
                self.boundaries[2].clone(),
            ),
            internal_adjacencies: scatter
                .links
                .iter()
                .map(|(i, j)| (*i + 1, *j + 1))
                .collect(),
            terrains,
            coastals: (1..=coastals).collect(),
        }
        .layout()
    }
}
