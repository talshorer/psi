//! force-segment-voronoi
//!
//! This module takes each land in a layout, and treats it as a point. The outer ring are used as
//! anchors based on their boundaries, forming the exterior of a square, with the corner lands at
//! the corners of the square.
//! Then, we apply `force_graph` to them, to let non-anchored points reach their position.
//! Once all points are fixed, we treat the boundary segments, and the edges between points, as
//! input segments for voronoi.
//! Since each input segment/point in voronoi produces a cell, we then get multiple cells per
//! land. Those are deduplicated by eliminating duplicate edges, and stitching together all the
//! surviving edges of each land into a polygon.

use std::collections::{HashMap, HashSet, hash_map::Entry};

use psi_topology::layout::{Corner, Edge, LandNum, Layout, LayoutEdge, Terrain};

use crate::{
    BoundingBox, PolyLands, Segment,
    poly::{Poly, PolyBuilder},
    vec::{NormalizedVec2, Vec2},
};

#[derive(Debug, Clone, Copy, PartialEq)]
struct LandData {
    land: LandNum,
    terrain: Terrain,
    anchor: Option<Segment>,
}

struct Graph<'a> {
    layout: &'a Layout,
    bb: &'a BoundingBox,
    inner: force_graph::ForceGraph<LandData, ()>,
}

impl<'a> Graph<'a> {
    fn evolve(&mut self, steps: usize) {
        for _ in 0..steps {
            self.inner.update(1.0 / 60.0);
        }
    }

    fn build(layout: &'a Layout, bb: &'a BoundingBox) -> Self {
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
            .edge(
                &layout.edges[Edge::Clock3],
                Vec2 { x: 0.0, y: bb.side },
                RIGHT,
            )
            .edge(
                &layout.edges[Edge::Clock6],
                Vec2 {
                    x: bb.side,
                    y: bb.side,
                },
                DOWN,
            )
            .edge(
                &layout.edges[Edge::Clock9],
                Vec2 { x: bb.side, y: 0.0 },
                LEFT,
            )
            .finalize(Vec2 {
                x: -bb.per_boundary,
                y: bb.side / 2.0,
            })
    }

    fn to_voronoi(
        &self,
    ) -> Result<(boostvoronoi::diagram::Diagram, VoronoiLands), boostvoronoi::BvError> {
        let mut lands = VoronoiLands::default();
        let diagram = boostvoronoi::builder::Builder::default()
            .with_segments({
                let mut nodes = HashMap::new();
                self.inner.visit_nodes(|node| {
                    assert!(node.x().abs() <= self.bb.side);
                    assert!(node.y().abs() <= self.bb.side);
                    if matches!(node.data.user_data.terrain, Terrain::Ocean) {
                        return;
                    }
                    nodes.insert(
                        node.data.user_data.land,
                        (
                            node.data.user_data,
                            Vec2 {
                                x: node.x(),
                                y: node.y(),
                            },
                        ),
                    );
                });
                let mut lines = Vec::new();
                for (data, point) in nodes.values() {
                    if let Some(anchor) = data.anchor {
                        if *point != anchor.0 {
                            lines.push(boostvoronoi::geometry::Line::new(
                                boostvoronoi::geometry::Point::new(point.x as i64, point.y as i64),
                                boostvoronoi::geometry::Point::new(
                                    anchor.0.x as i64,
                                    anchor.0.y as i64,
                                ),
                            ));
                            lands.push(*data);
                        }
                        if *point != anchor.1 {
                            lines.push(boostvoronoi::geometry::Line::new(
                                boostvoronoi::geometry::Point::new(point.x as i64, point.y as i64),
                                boostvoronoi::geometry::Point::new(
                                    anchor.1.x as i64,
                                    anchor.1.y as i64,
                                ),
                            ));
                            lands.push(*data);
                        }
                    }
                    for (data2, adj) in self
                        .layout
                        .internal_adjacencies
                        .get(&data.land)
                        .iter()
                        .flat_map(|m| m.iter())
                        .flat_map(|adj| nodes.get(adj))
                    {
                        if let Some(anchor1) = data.anchor
                            && let Some(anchor2) = data2.anchor
                            && (anchor1.0.x == anchor2.0.x
                                || anchor1.0.x == anchor2.1.x
                                || anchor1.1.x == anchor2.0.x
                                || anchor1.1.x == anchor2.1.x
                                || anchor1.0.y == anchor2.0.y
                                || anchor1.0.y == anchor2.1.y
                                || anchor1.1.y == anchor2.0.y
                                || anchor1.1.y == anchor2.1.y)
                        {
                            continue;
                        }
                        let mid = 0.5 * (*point + *adj);
                        lines.push(boostvoronoi::geometry::Line::new(
                            boostvoronoi::geometry::Point::new(point.x as i64, point.y as i64),
                            boostvoronoi::geometry::Point::new(mid.x as i64, mid.y as i64),
                        ));
                        lands.push(*data);
                    }
                }

                let lo = -self.bb.side;
                let hi = self.bb.side * 2.0;
                let infinity_points = [
                    Vec2 { x: lo, y: lo },
                    Vec2 { x: lo, y: hi },
                    Vec2 { x: hi, y: hi },
                    Vec2 { x: hi, y: lo },
                ];
                for (c1, c2) in infinity_points
                    .iter()
                    .zip(infinity_points.iter().cycle().skip(1))
                {
                    lines.push(boostvoronoi::geometry::Line::new(
                        boostvoronoi::geometry::Point::new(c1.x as i64, c1.y as i64),
                        boostvoronoi::geometry::Point::new(c2.x as i64, c2.y as i64),
                    ));
                }

                lines
            })?
            .build()?;
        Ok((diagram, lands))
    }
}

struct GraphBuilder<'a> {
    layout: &'a Layout,
    bb: &'a BoundingBox,
    open: HashSet<LandNum>,
    graph: force_graph::ForceGraph<LandData, ()>,
    indices: HashMap<LandNum, force_graph::DefaultNodeIdx>,
}

impl<'a> GraphBuilder<'a> {
    fn new(layout: &'a Layout, bb: &'a BoundingBox) -> Self {
        let open = layout.terrains.keys().copied().collect();
        Self {
            layout,
            bb,
            open,
            graph: force_graph::ForceGraph::new(Default::default()),
            indices: HashMap::new(),
        }
    }

    fn land(&mut self, land: LandNum, anchor: Option<(Vec2, Segment)>) {
        assert!(self.open.remove(&land));
        let (point, anchor) = anchor.unzip();
        self.indices.insert(
            land,
            self.graph.add_node(force_graph::NodeData {
                x: point.map(|p| p.x).unwrap_or(0.0),
                y: point.map(|p| p.y).unwrap_or(0.0),
                mass: 0.0,
                is_anchor: point.is_some(),
                user_data: LandData {
                    land,
                    terrain: self.layout.terrains[&land],
                    anchor,
                },
            }),
        );
    }

    fn corner(
        &mut self,
        corner: Corner,
        point: Vec2,
        backwards: Option<NormalizedVec2>,
        forwards: Option<NormalizedVec2>,
    ) -> &mut Self {
        let counter_clockwise = psi_topology::layout::COUNTER_CLOCKWISE.corner_to_edge(corner);
        let clockwise = psi_topology::layout::CLOCKWISE.corner_to_edge(corner);
        assert_eq!(backwards.is_none(), counter_clockwise.is_none());
        assert_eq!(forwards.is_none(), clockwise.is_none());
        let anchor = Segment(
            counter_clockwise
                .zip(backwards)
                .map_or_default(|(edge, advance)| {
                    (self.bb.side
                        - self.layout.edges[edge].boundaries.last().unwrap().0 as f32
                            * self.bb.per_boundary)
                        * *advance
                })
                + point,
            clockwise.zip(forwards).map_or_default(|(edge, advance)| {
                self.layout.edges[edge].boundaries.first().unwrap().0 as f32
                    * self.bb.per_boundary
                    * *advance
            }) + point,
        );
        self.land(self.layout.corner(corner), Some((point, anchor)));
        self
    }

    fn edge(&mut self, ledge: &LayoutEdge, offset: Vec2, advance: NormalizedVec2) -> &mut Self {
        for (idx, land) in ledge.lands[1..ledge.lands.len() - 1]
            .iter()
            .copied()
            .enumerate()
        {
            let anchor = Segment(
                ledge.boundaries[idx].0 as f32 * self.bb.per_boundary * *advance + offset,
                ledge.boundaries[idx + 1].0 as f32 * self.bb.per_boundary * *advance + offset,
            );
            let point = 0.5 * (anchor.0 + anchor.1);
            self.land(land, Some((point, anchor)));
        }
        self
    }

    fn finalize(&mut self, ocean: Vec2) -> Graph<'a> {
        self.land(LandNum(0), Some((ocean, Segment(ocean, ocean))));

        for land in self.open.clone() {
            self.land(land, None);
        }

        for (land1, adj) in &self.layout.internal_adjacencies {
            for land2 in adj {
                if land1.0 >= land2.0 {
                    continue;
                }
                self.graph
                    .add_edge(self.indices[land1], self.indices[land2], Default::default());
            }
        }

        Graph {
            layout: self.layout,
            bb: self.bb,
            inner: std::mem::replace(
                &mut self.graph,
                force_graph::ForceGraph::new(Default::default()),
            ),
        }
    }
}

#[derive(Default)]
struct VoronoiLands {
    by_src_idx: Vec<LandNum>,
    data: HashMap<LandNum, LandData>,
}

impl VoronoiLands {
    fn push(&mut self, data: LandData) {
        self.by_src_idx.push(data.land);
        match self.data.entry(data.land) {
            Entry::Occupied(occupied_entry) => assert_eq!(*occupied_entry.get(), data),
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(data);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct SegmentKey(u32, u32);

impl SegmentKey {
    fn reverse(&self) -> Self {
        Self(self.1, self.0)
    }
}

struct VoronoiPost<'a> {
    diagram: &'a boostvoronoi::diagram::Diagram,
    lands: &'a VoronoiLands,
    cells: HashMap<LandNum, HashMap<SegmentKey, Segment>>,
}

impl<'a> VoronoiPost<'a> {
    fn new(diagram: &'a boostvoronoi::diagram::Diagram, lands: &'a VoronoiLands) -> Self {
        Self {
            diagram,
            lands,
            cells: Default::default(),
        }
    }

    fn edge_to_segment(
        diagram: &boostvoronoi::diagram::Diagram,
        edge_id: boostvoronoi::diagram::EdgeIndex,
    ) -> Option<(SegmentKey, Segment)> {
        let v0 = diagram.edge_get_vertex0(edge_id).ok().flatten()?;
        let p0 = diagram.vertex(v0).ok()?;
        let v1 = diagram.edge_get_vertex1(edge_id).ok().flatten()?;
        let p1 = diagram.vertex(v1).ok()?;

        Some((
            SegmentKey(v0.u32(), v1.u32()),
            Segment(
                Vec2 {
                    x: p0.x() as f32,
                    y: p0.y() as f32,
                },
                Vec2 {
                    x: p1.x() as f32,
                    y: p1.y() as f32,
                },
            ),
        ))
    }

    fn cell(&mut self, cell: &boostvoronoi::diagram::Cell) {
        let Some(land) = self
            .lands
            .by_src_idx
            .get(cell.source_index().usize())
            .and_then(|land| self.lands.data.get(land))
        else {
            return;
        };

        let edges = self.cells.entry(land.land).or_default();
        for edge_id in self.diagram.cell_edge_iterator(cell.id()) {
            let Some((key, segment)) = Self::edge_to_segment(self.diagram, edge_id) else {
                continue;
            };
            if edges.remove(&key.reverse()).is_none() {
                edges.insert(key, segment);
            }
        }
    }

    fn polys(mut self) -> HashMap<LandNum, (Terrain, Poly)> {
        for cell in self.diagram.cells().iter() {
            self.cell(cell)
        }

        self.cells
            .iter()
            .map(|(land, edges)| {
                let (first_key, first_segment) = edges.iter().next().unwrap();
                let mut edge_by_vidx = edges
                    .iter()
                    .map(|(k, v)| (k.0, (k.1, *v)))
                    .collect::<HashMap<_, _>>();
                assert_eq!(edge_by_vidx.len(), edges.len());
                edge_by_vidx.remove(&first_key.0);

                let mut poly = PolyBuilder::default();
                let mut segment = *first_segment;
                poly.push(segment);
                let mut next = first_key.1;
                while !edge_by_vidx.is_empty() {
                    (next, segment) = edge_by_vidx.remove(&next).unwrap();
                    poly.push(segment);
                }
                assert_eq!(next, first_key.0);
                let poly = poly.poly();
                let terrain = self.lands.data[land].terrain;
                (*land, (terrain, poly))
            })
            .collect()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ForSegVorError {
    #[error("Voronoi({0})")]
    Voronoi(#[from] boostvoronoi::BvError),
}

pub(crate) fn for_seg_vor(layout: &Layout, bb: &BoundingBox) -> Result<PolyLands, ForSegVorError> {
    let mut graph = Graph::build(layout, bb);
    graph.evolve(1000);
    let (diagram, lands) = graph.to_voronoi()?;
    Ok(PolyLands(VoronoiPost::new(&diagram, &lands).polys()))
}
