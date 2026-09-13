use std::{
    collections::{HashMap, HashSet},
    iter::Peekable,
    rc::Rc,
    sync::LazyLock,
};

use either::Either;
use enum_map::{Enum, EnumMap, enum_map};

pub mod data;

struct LayoutData<'a> {
    envelope: &'a [usize],
    boundaries: (&'a [usize], &'a [usize], &'a [usize]),
    internal_adjacencies: &'a [(usize, usize)],
    terrains: &'a [char],
    coastals: &'a [usize],
}

pub struct ValidatedLayoutData<'a>(&'a LayoutData<'a>);

impl<'a> ValidatedLayoutData<'a> {
    const fn validate(data: &'a LayoutData) -> Option<Self> {
        const fn validate_terrains(terrains: &[char]) -> bool {
            let len = terrains.len();
            let mut i = 0;
            while i < len {
                if Terrain::from_char(terrains[i]).is_none() {
                    return false;
                }
                i += 1;
            }
            true
        }

        const fn validate_land(land: usize, num_lands: usize) -> bool {
            land > 0 && land <= num_lands
        }

        const fn validate_envelope(
            envelope: &[usize],
            boundaries: &(&[usize], &[usize], &[usize]),
            num_lands: usize,
        ) -> bool {
            let len = envelope.len();
            let mut i = 0;
            while i < len {
                if !validate_land(envelope[i], num_lands) {
                    return false;
                }
                i += 1;
            }
            len == boundaries.0.len() + boundaries.1.len() + boundaries.2.len() + 1
        }

        const fn validate_boundaries(boundaries: &[usize]) -> bool {
            let len = boundaries.len();
            let mut i = 0;
            while i < len {
                if boundaries[i] < Boundary::MIN || boundaries[i] > Boundary::MAX {
                    return false;
                }
                i += 1;
            }
            true
        }

        const fn validate_internal_adjacencies(
            adjacencies: &[(usize, usize)],
            num_lands: usize,
        ) -> bool {
            let len = adjacencies.len();
            let mut i = 0;
            while i < len {
                if !validate_land(adjacencies[i].0, num_lands)
                    || !validate_land(adjacencies[i].1, num_lands)
                {
                    return false;
                }
                i += 1;
            }
            true
        }

        const fn validate_coastals(coastals: &[usize], num_lands: usize) -> bool {
            let len = coastals.len();
            let mut i = 0;
            while i < len {
                if !validate_land(coastals[i], num_lands) {
                    return false;
                }
                i += 1;
            }
            true
        }

        let num_lands = data.terrains.len();

        if validate_terrains(data.terrains)
            && validate_envelope(data.envelope, &data.boundaries, num_lands)
            && validate_boundaries(data.boundaries.0)
            && validate_boundaries(data.boundaries.1)
            && validate_boundaries(data.boundaries.2)
            && validate_internal_adjacencies(data.internal_adjacencies, num_lands)
            && validate_coastals(data.coastals, num_lands)
        {
            Some(Self(data))
        } else {
            None
        }
    }

    pub fn layout(&self) -> Rc<Layout> {
        Rc::new(Layout::new(self))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Terrain {
    Jungle,
    Mountain,
    Sands,
    Wetlands,
    Ocean,
}

impl Terrain {
    const fn from_char(c: char) -> Option<Self> {
        match c {
            'J' => Some(Self::Jungle),
            'M' => Some(Self::Mountain),
            'S' => Some(Self::Sands),
            'W' => Some(Self::Wetlands),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Boundary(usize);

impl Boundary {
    const MIN: usize = 1;
    const MAX: usize = 6;

    pub(crate) fn add_would_overflow(&self, other: &Self) -> bool {
        self.0 + other.0 > Self::MAX + 1
    }
}

#[derive(Enum, Clone, Copy)]
pub enum Edge {
    // We treat the ocean as being Clock12, and work from that
    Clock3,
    Clock6,
    Clock9,
}

#[derive(Enum, Clone, Copy)]
pub(crate) enum Corner {
    Clock1,
    Clock5,
    Clock7,
    Clock11,
}

pub(crate) struct Rotate<'a> {
    to_edge: EnumMap<Edge, Option<Edge>>,
    to_corner: EnumMap<Edge, Corner>,
    reverse: &'a LazyLock<Self>,
}

impl<'a> Rotate<'a> {
    pub(crate) fn to_edge(&self, edge: Edge) -> Option<Edge> {
        self.to_edge[edge]
    }

    pub(crate) fn to_corner(&self, edge: Edge) -> Corner {
        self.to_corner[edge]
    }
    pub(crate) fn reverse(&self) -> &Self {
        self.reverse
    }
}

pub(crate) static CLOCKWISE: LazyLock<Rotate> = LazyLock::new(|| Rotate {
    to_edge: enum_map! {
        Edge::Clock3 => Some(Edge::Clock6),
        Edge::Clock6 => Some(Edge::Clock9),
        Edge::Clock9 => None,
    },
    to_corner: enum_map! {
        Edge::Clock3 => Corner::Clock5,
        Edge::Clock6 => Corner::Clock7,
        Edge::Clock9 => Corner::Clock11,
    },
    reverse: &COUNTER_CLOCKWISE,
});

pub(crate) static COUNTER_CLOCKWISE: LazyLock<Rotate> = LazyLock::new(|| Rotate {
    to_edge: enum_map! {
        Edge::Clock3 => None,
        Edge::Clock6 => Some(Edge::Clock3),
        Edge::Clock9 => Some(Edge::Clock6),
    },
    to_corner: enum_map! {
        Edge::Clock3 => Corner::Clock1,
        Edge::Clock6 => Corner::Clock5,
        Edge::Clock9 => Corner::Clock7,
    },
    reverse: &CLOCKWISE,
});

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct LandNum(pub(crate) usize);

pub(crate) struct LayoutEdge {
    pub(crate) lands: Vec<LandNum>,
    pub(crate) boundaries: Vec<Boundary>,
}

impl LayoutEdge {
    fn new(envelope: &[usize], boundaries: &[usize], start: usize, end: usize) -> Self {
        let lands = envelope[start..end + 1]
            .iter()
            .copied()
            .map(LandNum)
            .collect::<Vec<_>>();
        let boundaries = boundaries.iter().copied().map(Boundary).collect::<Vec<_>>();
        debug_assert!(lands.len() == boundaries.len() + 1, "Invalid layout edge");
        Self { lands, boundaries }
    }

    fn iter(&self, rev: bool) -> impl Iterator<Item = LayoutEdgeIterItem> {
        let lands = self.lands.iter().copied();
        let boundaries = self.boundaries.iter().copied();
        let (lands, boundaries, append) = if rev {
            (
                Either::Left(lands.rev()),
                Either::Left(boundaries.rev()),
                Boundary::MIN - 1,
            )
        } else {
            (
                Either::Right(lands),
                Either::Right(boundaries),
                Boundary::MAX + 1,
            )
        };
        boundaries
            .chain(Some(Boundary(append)))
            .zip(lands)
            .map(|(boundary, land)| LayoutEdgeIterItem { boundary, land })
    }

    pub(crate) fn link_iter<'a>(
        &'a self,
        other: &'a Self,
    ) -> impl Iterator<Item = (LandNum, LandNum)> + 'a {
        LayoutEdgeLinkIter {
            a: self.iter(false).peekable(),
            b: other.iter(true).peekable(),
        }
    }
}

#[derive(Clone, Copy)]
struct LayoutEdgeIterItem {
    boundary: Boundary,
    land: LandNum,
}

struct LayoutEdgeLinkIter<I: Iterator<Item = LayoutEdgeIterItem>> {
    // `a` goes up in `Boundary` values, `b` goes down
    a: Peekable<I>,
    b: Peekable<I>,
}

impl<I: Iterator<Item = LayoutEdgeIterItem>> Iterator for LayoutEdgeLinkIter<I> {
    type Item = (LandNum, LandNum);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(a) = self.a.peek().copied()
            && let Some(b) = self.b.peek().copied()
        {
            if a.boundary.add_would_overflow(&b.boundary) {
                self.b.next();
            } else {
                self.a.next();
            }
            Some((a.land, b.land))
        } else {
            debug_assert!({
                // One of the iterators is exhausted. The other should be one element away from
                // being exhausted.
                let it = if self.a.peek().is_none() {
                    &mut self.b
                } else {
                    &mut self.a
                };
                it.next();
                it.peek().is_none()
            });
            None
        }
    }
}

pub struct Layout {
    pub(crate) edges: EnumMap<Edge, LayoutEdge>,
    pub(crate) terrains: HashMap<LandNum, Terrain>,
    pub(crate) internal_adjacencies: HashMap<LandNum, HashSet<LandNum>>,
}

impl Layout {
    fn new(data: &ValidatedLayoutData) -> Self {
        fn link(
            internal_adjacencies: &mut HashMap<LandNum, HashSet<LandNum>>,
            i: LandNum,
            j: LandNum,
        ) {
            internal_adjacencies
                .get_mut(&i)
                .expect("Invalid internal adjacency in validated data")
                .insert(j);
        }

        let LayoutData {
            envelope,
            boundaries,
            internal_adjacencies: internal_adjacencies_data,
            terrains,
            coastals,
        } = data.0;

        let corner1 = 0;
        let corner5 = corner1 + boundaries.0.len();
        let corner7 = corner5 + boundaries.1.len();
        let corner11 = corner7 + boundaries.2.len();
        debug_assert_eq!(
            corner11 + 1,
            envelope.len(),
            "Invalid envelope length in validated data"
        );

        let mut internal_adjacencies =
            HashMap::from_iter((0..=terrains.len()).map(|i| (LandNum(i), HashSet::default())));
        for (i, j) in internal_adjacencies_data
            .iter()
            .copied()
            .chain(coastals.iter().map(|i| (0, *i)))
            .map(|(i, j)| (LandNum(i), LandNum(j)))
        {
            link(&mut internal_adjacencies, i, j);
            link(&mut internal_adjacencies, j, i);
        }

        Self {
            edges: enum_map! {
                Edge::Clock3 => LayoutEdge::new(envelope, boundaries.0, corner1, corner5),
                Edge::Clock6 => LayoutEdge::new(envelope, boundaries.1, corner5, corner7),
                Edge::Clock9 => LayoutEdge::new(envelope, boundaries.2, corner7, corner11),
            },
            terrains: terrains
                .iter()
                .copied()
                .enumerate()
                .map(|(i, c)| {
                    (
                        LandNum(i + 1),
                        Terrain::from_char(c).expect("Invalid terrain char in validated data"),
                    )
                })
                .chain(Some((LandNum(0), Terrain::Ocean)))
                .collect(),
            internal_adjacencies,
        }
    }

    pub(crate) fn corner(&self, corner: Corner) -> LandNum {
        match corner {
            Corner::Clock1 => LandNum(1),
            Corner::Clock5 => *self.edges[Edge::Clock3]
                .lands
                .last()
                .expect("Invalid layout"),
            Corner::Clock7 => *self.edges[Edge::Clock6]
                .lands
                .last()
                .expect("Invalid layout"),
            Corner::Clock11 => LandNum(3),
        }
    }
}
