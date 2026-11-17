//! force-only
//!
//! This module builds one polygon per land directly from the layout's adjacencies. Graph vertices
//! mark where cell boundaries meet, and graph edges trace the boundaries between cells. The board
//! edges and corners provide anchors while the remaining vertices are relaxed by a spring
//! simulation.
//!
//! After relaxation, a deflation simulation moves shared boundaries according to the area
//! difference (simulating pressure) between neighboring lands, shrinking larger cells and
//! expanding smaller ones.

use std::collections::{BTreeMap, HashMap};

use psi_topology::layout::{Corner, Edge, LandNum, Layout, Terrain};
use psi_vec::{NormalizedVec2, Vec2};

use crate::{
    BoundingBox, PolyLands, Segment,
    poly::{Poly, PolyBuilder},
};

use key::{BoundaryKey, VertexKey};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CellKey {
    Land(LandNum),
    Edge(Edge),
}

impl CellKey {
    fn non_ocean_land(&self) -> Option<LandNum> {
        match self {
            CellKey::Edge(_) | CellKey::Land(LandNum(0)) => None,
            CellKey::Land(land) => Some(*land),
        }
    }
}

impl From<Option<Edge>> for CellKey {
    fn from(value: Option<Edge>) -> Self {
        value.map(Self::Edge).unwrap_or(Self::Land(LandNum(0)))
    }
}

mod key {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    pub(super) struct Key<T, const N: usize>([T; N]);

    impl<T: Ord, const N: usize> Key<T, N> {
        pub(super) fn new(mut subkeys: [T; N]) -> Self {
            subkeys.sort();
            Self(subkeys)
        }

        pub(super) fn iter(&self) -> impl Iterator<Item = &T> {
            self.0.iter()
        }

        pub(super) fn contains(&self, v: T) -> bool {
            self.iter().position(|v2| *v2 == v).is_some()
        }
    }

    pub(super) type VertexKey = Key<super::CellKey, 3>;
    pub(super) type BoundaryKey = Key<psi_topology::layout::LandNum, 2>;
}

impl VertexKey {
    fn is_corner(&self) -> bool {
        self.iter()
            .filter(|cell| cell.non_ocean_land().is_some())
            .count()
            == 1
    }
}

struct VertexData {
    anchor: VertexAnchor,
    links: Box<[VertexKey]>,
}

#[derive(Debug)]
struct VertexDataBuilder {
    anchor: VertexAnchor,
    links: Vec<VertexKey>,
}

impl VertexDataBuilder {
    fn data(&mut self) -> VertexData {
        VertexData {
            anchor: self.anchor,
            links: std::mem::take(&mut self.links).into_boxed_slice(),
        }
    }

    fn position(&self, bb: &BoundingBox) -> Vec2 {
        let midpoint = bb.side / 2.0;
        match self.anchor {
            VertexAnchor::None => Vec2 {
                x: midpoint,
                y: midpoint,
            },
            VertexAnchor::Ocean => Vec2 {
                x: 0.0,
                y: midpoint,
            },
            VertexAnchor::Fixed(vec2) => vec2,
        }
    }
}

#[derive(Debug)]
struct CellData {
    poly_vertices: Vec<VertexKey>,
    links: BTreeMap<CellKey, (VertexKey, VertexKey)>,
}

impl CellData {
    fn shoelace_area(&self, position: &HashMap<VertexKey, Vec2>) -> f64 {
        self.poly_vertices
            .iter()
            .zip(self.poly_vertices.iter().cycle().skip(1))
            .map(|(v1, v2)| {
                let p1 = position[v1];
                let p2 = position[v2];
                (p1.x * p2.y) - (p1.y * p2.x)
            })
            .sum::<f64>()
            / 2.0
    }
}

struct CellDataBuilderSide {
    vertices: (VertexKey, VertexKey),
    link: CellKey,
}

#[derive(Default)]
struct CellDataBuilder {
    sides: Vec<CellDataBuilderSide>,
}

impl CellDataBuilder {
    fn data(&mut self) -> CellData {
        let mut sides = std::mem::take(&mut self.sides);
        let CellDataBuilderSide {
            vertices: (first, mut next),
            link,
        } = sides.pop().unwrap();
        let mut links = BTreeMap::new();
        links.insert(link, (first, next));
        let mut poly_vertices = vec![first];
        while !sides.is_empty() {
            let (idx, new) = sides
                .iter()
                .enumerate()
                .find_map(|(idx, side)| {
                    if side.vertices.0 == next {
                        Some((idx, side.vertices.1))
                    } else if side.vertices.1 == next {
                        Some((idx, side.vertices.0))
                    } else {
                        None
                    }
                })
                .unwrap();
            let side = sides.swap_remove(idx);
            links.insert(side.link, (next, new));
            poly_vertices.push(next);
            next = new;
        }
        assert_eq!(first, next);
        CellData {
            poly_vertices,
            links,
        }
    }
}

pub(crate) struct ForceOnlyConfig {
    spring_force: f64,
    pressure: f64,
    spring_force2: f64,
    edge_push_force: f64,
    spring_iters: usize,
    deflate_iters: usize,
}

impl Default for ForceOnlyConfig {
    fn default() -> Self {
        fn from_env<T>(env: &str, default: T) -> T
        where
            T: std::str::FromStr,
        {
            std::env::var(env)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(default)
        }

        Self {
            spring_force: from_env("FO_SPRING_FORCE", 0.1),
            pressure: from_env("FO_PRESSURE", 3000.0),
            spring_force2: from_env("FO_SPRING_FORCE2", 0.01),
            edge_push_force: from_env("FO_EDGE_PUSH_FORCE", 0.5),
            spring_iters: from_env("FO_SPRING_ITERS", 100),
            deflate_iters: from_env("FO_DEFLATE_ITERS", 100),
        }
    }
}

struct Graph<'a> {
    layout: &'a Layout,
    bb: &'a BoundingBox,
    vertices: BTreeMap<VertexKey, VertexData>,
    cells: BTreeMap<LandNum, CellData>,
    config: ForceOnlyConfig,
    position: HashMap<VertexKey, Vec2>,
    force: HashMap<VertexKey, Vec2>,
}

impl<'a> Graph<'a> {
    fn build(layout: &'a Layout, bb: &'a BoundingBox) -> Result<Self, EnclaveUnsupported> {
        // TODO dedup candidate with `for_seg_vor`
        const RIGHT: NormalizedVec2 = NormalizedVec2::const_new(Vec2 { x: 1.0, y: 0.0 }).unwrap();
        const LEFT: NormalizedVec2 = NormalizedVec2::const_new(Vec2 { x: -1.0, y: 0.0 }).unwrap();
        const UP: NormalizedVec2 = NormalizedVec2::const_new(Vec2 { x: 0.0, y: 1.0 }).unwrap();
        const DOWN: NormalizedVec2 = NormalizedVec2::const_new(Vec2 { x: 0.0, y: -1.0 }).unwrap();

        GraphBuilder::new(layout, bb)
            .corner(
                Corner::Clock1,
                Vec2 { x: 0.0, y: bb.side },
                None,
                Some(RIGHT),
            )
            .corner(
                Corner::Clock5,
                Vec2 {
                    x: bb.side,
                    y: bb.side,
                },
                Some(LEFT),
                Some(DOWN),
            )
            .corner(
                Corner::Clock7,
                Vec2 { x: bb.side, y: 0.0 },
                Some(UP),
                Some(LEFT),
            )
            .corner(Corner::Clock11, Vec2 { x: 0.0, y: 0.0 }, Some(RIGHT), None)
            .edge(Edge::Clock3, Vec2 { x: 0.0, y: bb.side }, RIGHT)
            .edge(
                Edge::Clock6,
                Vec2 {
                    x: bb.side,
                    y: bb.side,
                },
                DOWN,
            )
            .edge(Edge::Clock9, Vec2 { x: bb.side, y: 0.0 }, LEFT)
            .finalize()
    }

    fn springs(&mut self) {
        for (k1, v1) in &self.vertices {
            if matches!(v1.anchor, VertexAnchor::Fixed(_)) {
                continue;
            }
            let mut force = Vec2::default();
            for v2 in &v1.links {
                let diff = self.position[k1] - self.position[v2];
                force += -self.config.spring_force * diff;
            }
            *self.force.get_mut(k1).unwrap() = force;
        }
    }

    fn deflate(&mut self) {
        let mut inv_areas = HashMap::new();
        for (c1, d1) in &self.cells {
            let ia1 = *inv_areas
                .entry(*c1)
                .or_insert_with(|| 1.0 / d1.shoelace_area(&self.position));
            for (c2, (v1, v2)) in d1
                .links
                .iter()
                .filter_map(|(k, v)| k.non_ocean_land().map(|c2| (c2, v)))
            {
                let d2 = &self.cells[&c2];
                let ia2 = *inv_areas
                    .entry(c2)
                    .or_insert_with(|| 1.0 / d2.shoelace_area(&self.position));
                // by `orient_cells`, we know `v1` -> `v2` goes counterclockwise.
                let diff = self.position[v2] - self.position[v1];
                // clock12 -> clock9 @@ (0,  1) -> (-1, 0)
                // clock9  -> clock6 @@ (-1, 0) -> (0, -1)
                let inwards = diff.quarter_counterclockwise();
                let force = (ia2 - ia1) * self.config.pressure / 2.0 * inwards;
                *self.force.get_mut(v1).unwrap() += force;
                *self.force.get_mut(v2).unwrap() += force;
            }
            for (v1, v2) in d1
                .poly_vertices
                .iter()
                .zip(d1.poly_vertices.iter().cycle().skip(1))
            {
                let diff = self.position[v1] - self.position[v2];
                let force = -self.config.spring_force2 * diff;
                *self.force.get_mut(v1).unwrap() += force;
                *self.force.get_mut(v2).unwrap() -= force;
            }
        }
        for v in self.vertices.keys() {
            let pos = self.position[v];
            let mut force = Vec2::default();
            if pos.x < self.bb.side * 0.1 {
                force += Vec2 {
                    x: (self.bb.side * 0.1 - pos.x) * self.config.edge_push_force,
                    y: 0.0,
                }
            } else if pos.x > self.bb.side * 0.9 {
                force += Vec2 {
                    x: (self.bb.side * 0.9 - pos.x) * self.config.edge_push_force,
                    y: 0.0,
                }
            }
            if pos.y < self.bb.side * 0.1 {
                force += Vec2 {
                    x: 0.0,
                    y: (self.bb.side * 0.1 - pos.y) * self.config.edge_push_force,
                }
            } else if pos.y > self.bb.side * 0.9 {
                force += Vec2 {
                    x: 0.0,
                    y: (self.bb.side * 0.9 - pos.y) * self.config.edge_push_force,
                }
            }
            *self.force.get_mut(v).unwrap() += force;
        }
    }

    fn apply_forces(&mut self) {
        for (key, force) in self.force.iter_mut() {
            let anchor = self.vertices[key].anchor;
            *self.position.get_mut(key).unwrap() += match anchor {
                VertexAnchor::None => *force,
                VertexAnchor::Ocean => Vec2 { x: 0.0, y: force.y },
                VertexAnchor::Fixed(_) => Vec2::default(),
            };
            *force = Vec2::default();
        }
    }

    fn orient_cells(&mut self) {
        fn idx_wraparound(len: usize, idx1: usize, idx2: usize) -> bool {
            idx1 == 0 && idx2 == len - 1
        }

        for data in self.cells.values_mut() {
            // `shoelace_area` is negative for a clockwise polygon.
            if data.shoelace_area(&self.position) < 0.0 {
                data.poly_vertices.reverse();
            }
            for (v1, v2) in data.links.values_mut() {
                let idx1 = data.poly_vertices.iter().position(|v| *v == *v1).unwrap();
                let idx2 = data.poly_vertices.iter().position(|v| *v == *v2).unwrap();
                let poly_len = data.poly_vertices.len();
                if (idx1 > idx2 && !idx_wraparound(poly_len, idx2, idx1))
                    || idx_wraparound(poly_len, idx1, idx2)
                {
                    std::mem::swap(v1, v2);
                }
            }
        }
    }

    fn evolve(&mut self) {
        for _ in 0..self.config.spring_iters {
            self.springs();
            self.apply_forces();
        }
        self.orient_cells();
        for _ in 0..self.config.deflate_iters {
            self.deflate();
            self.apply_forces();
        }
    }

    fn polys(&self) -> BTreeMap<LandNum, (Terrain, Poly)> {
        let mut polys = BTreeMap::new();
        for (v, p) in &self.position {
            assert!(
                p.x >= 0.0 && p.x <= self.bb.side && p.y >= 0.0 && p.y <= self.bb.side,
                "{v:?} {p:?}",
            )
        }
        for (land, data) in &self.cells {
            let terrain = self.layout.terrains[land];
            let mut poly = PolyBuilder::default();
            let first = self.position[data.poly_vertices.first().unwrap()];
            let mut prev = first;
            for vertex in data.poly_vertices.iter().skip(1) {
                let next = self.position[vertex];
                poly.push(Segment(prev, next));
                prev = next;
            }
            poly.push(Segment(prev, first));
            polys.insert(*land, (terrain, poly.poly()));
        }
        polys
    }
}

struct GraphBuilder<'a> {
    layout: &'a Layout,
    bb: &'a BoundingBox,
    vertices: BTreeMap<VertexKey, VertexDataBuilder>,
    cell_edges: BTreeMap<CellKey, CellDataBuilder>,
    boundaries: HashMap<BoundaryKey, Vec<VertexKey>>,
}

#[derive(Debug, thiserror::Error)]
#[error("EnclaveUnsupported")]
pub struct EnclaveUnsupported;

#[derive(Debug, Clone, Copy)]
enum VertexAnchor {
    None,
    Ocean,
    Fixed(Vec2),
}

// TODO dedup as much as possible with `for_seg_vor`
impl<'a> GraphBuilder<'a> {
    fn new(layout: &'a Layout, bb: &'a BoundingBox) -> Self {
        Self {
            layout,
            bb,
            vertices: Default::default(),
            boundaries: Default::default(),
            cell_edges: Default::default(),
        }
    }

    fn vertex(&mut self, key: VertexKey, anchor: VertexAnchor) {
        self.vertices.entry(key).or_insert(VertexDataBuilder {
            anchor,
            links: Vec::new(),
        });
    }

    fn link_cell_one_way(&mut self, c1: CellKey, c2: CellKey, v1: VertexKey, v2: VertexKey) {
        assert!(v1.contains(c1));
        assert!(v2.contains(c1));
        self.cell_edges
            .entry(c1)
            .or_default()
            .sides
            .push(CellDataBuilderSide {
                vertices: (v1, v2),
                link: c2,
            });
    }

    fn link_vertices_one_way(&mut self, v1: VertexKey, v2: VertexKey) {
        self.vertices.get_mut(&v1).unwrap().links.push(v2);
    }

    fn link(&mut self, v1: VertexKey, v2: VertexKey, c1: CellKey, c2: CellKey) {
        self.link_cell_one_way(c1, c2, v1, v2);
        self.link_cell_one_way(c2, c1, v1, v2);
        self.link_vertices_one_way(v1, v2);
        self.link_vertices_one_way(v2, v1);
    }

    fn corner(
        &mut self,
        corner: Corner,
        point: Vec2,
        backwards: Option<NormalizedVec2>,
        forwards: Option<NormalizedVec2>,
    ) -> &mut Self {
        // TODO dedup candidate with `for_seg_vor`
        let counter_clockwise = psi_topology::layout::COUNTER_CLOCKWISE.corner_to_edge(corner);
        let clockwise = psi_topology::layout::CLOCKWISE.corner_to_edge(corner);
        let land = self.layout.corner(corner);
        let key = VertexKey::new([
            CellKey::Land(land),
            counter_clockwise.into(),
            clockwise.into(),
        ]);
        if (backwards.zip(forwards)).is_none() {
            assert_eq!(backwards.is_none(), counter_clockwise.is_none());
            assert_eq!(forwards.is_none(), clockwise.is_none());
            self.boundaries
                .entry(BoundaryKey::new([land, LandNum(0)]))
                .or_default()
                .push(key);
        }
        self.vertex(key, VertexAnchor::Fixed(point));
        self
    }

    fn edge(&mut self, edge: Edge, offset: Vec2, advance: NormalizedVec2) -> &mut Self {
        let ledge = &self.layout.edges[edge];
        let mut prev_key = VertexKey::new([
            CellKey::Land(ledge.lands[0]),
            CellKey::Edge(edge),
            CellKey::from(psi_topology::layout::COUNTER_CLOCKWISE.edge_to_edge(edge)),
        ]);
        for (idx, boundary) in ledge.boundaries.iter().enumerate() {
            let land1 = ledge.lands[idx];
            let land2 = ledge.lands[idx + 1];
            let key = VertexKey::new([
                CellKey::Edge(edge),
                CellKey::Land(land1),
                CellKey::Land(land2),
            ]);
            self.boundaries
                .entry(BoundaryKey::new([land1, land2]))
                .or_default()
                .push(key);
            self.vertex(
                key,
                VertexAnchor::Fixed(boundary.0 as f64 * self.bb.per_boundary * *advance + offset),
            );
            self.link(prev_key, key, CellKey::Edge(edge), CellKey::Land(land1));
            prev_key = key;
        }
        let last = *ledge.lands.last().unwrap();
        self.link(
            prev_key,
            VertexKey::new([
                CellKey::Land(last),
                CellKey::Edge(edge),
                CellKey::from(psi_topology::layout::CLOCKWISE.edge_to_edge(edge)),
            ]),
            CellKey::Edge(edge),
            CellKey::Land(last),
        );
        self
    }

    fn finalize(&mut self) -> Result<Graph<'a>, EnclaveUnsupported> {
        // TODO dedup candidate with `for_seg_vor` (add an adjacencies iterator to `Layout`)
        for (land1, adj) in &self.layout.internal_adjacencies {
            for land2 in adj {
                if land1 >= land2 {
                    continue;
                }
                let mut vertices = self
                    .boundaries
                    .get(&BoundaryKey::new([*land1, *land2]))
                    .cloned()
                    .unwrap_or_default();
                for land3 in adj {
                    if self.layout.internal_adjacencies[land2].contains(land3) {
                        let key = VertexKey::new([
                            CellKey::Land(*land1),
                            CellKey::Land(*land2),
                            CellKey::Land(*land3),
                        ]);
                        let anchor = if key
                            .iter()
                            .any(|cell| matches!(cell, CellKey::Land(LandNum(0))))
                        {
                            VertexAnchor::Ocean
                        } else {
                            VertexAnchor::None
                        };
                        self.vertex(key, anchor);
                        vertices.push(key);
                    }
                }
                assert!(vertices.len() % 2 == 0);
                let Some([v1, v2]) = vertices.as_array() else {
                    // Consider a layout like
                    // ┏━━━┳━━━┓
                    // ┃ ┏━┻━┓ ┃
                    // ┃1┃ 2 ┃3┃
                    // ┃ ┗━┳━┛ ┃
                    // ┗━━━┻━━━┛
                    // Land 2 exists between two other lands with no other adjacencies.
                    // To support enclaves, we'll have to differentiate between the two segments
                    // shared by lands 1 and 3, as well as have some vertices for land 2.
                    // As it stands, the edge of lands 1 and 3 will run "through" land 2, and land 2
                    // will only have 2 vertices, making it a line.
                    // This configuration doesn't actually exist in official boards, so defer
                    // supporting it.
                    return Err(EnclaveUnsupported);
                };
                self.link(*v1, *v2, CellKey::Land(*land1), CellKey::Land(*land2));
            }
        }
        let lands = self
            .cell_edges
            .iter_mut()
            .filter_map(|(k, v)| k.non_ocean_land().map(|land| (land, v.data())))
            .collect::<BTreeMap<_, _>>();
        for (key, data) in &self.vertices {
            assert_eq!(
                data.links.len(),
                if key.is_corner() { 2 } else { 3 },
                "key: {key:?}, data: {data:?}",
            );
            for land in key.iter().filter_map(|cell| cell.non_ocean_land()) {
                assert!(
                    lands[&land].poly_vertices.contains(key),
                    "land: {land:?}, key: {key:?}",
                );
            }
        }
        Ok(Graph {
            layout: self.layout,
            bb: self.bb,
            vertices: self
                .vertices
                .iter_mut()
                .map(|(k, v)| (*k, v.data()))
                .collect(),
            cells: lands,
            config: ForceOnlyConfig::default(),
            position: self
                .vertices
                .iter()
                .map(|(k, v)| (*k, v.position(self.bb)))
                .collect(),
            force: self
                .vertices
                .keys()
                .map(|k| (*k, Vec2::default()))
                .collect(),
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ForceOnlyError {
    #[error("EnclaveUnsupported")]
    EnclaveUnsupported(#[from] EnclaveUnsupported),
}

pub(crate) fn force_only(layout: &Layout, bb: &BoundingBox) -> Result<PolyLands, ForceOnlyError> {
    let mut graph = Graph::build(layout, bb)?;
    graph.evolve();
    Ok(PolyLands(graph.polys()))
}
