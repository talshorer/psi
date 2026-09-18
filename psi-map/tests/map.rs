use std::error::Error;

use psi_map::{
    Map,
    board::{BoardEdge, BoardEdgeLinkError},
    layout::Edge::{self},
};

#[test]
fn edge_link_map_mismatch() -> Result<(), Box<dyn Error>> {
    let map_a = Map::new();
    let a = map_a.add_board("A".into(), psi_map::layout::data::A.layout());
    let map_b = Map::new();
    let b = map_b.add_board("B".into(), psi_map::layout::data::B.layout());
    let result = a.edge(Edge::Clock3).link(&b.edge(Edge::Clock6));
    let Err(BoardEdgeLinkError::MapMismatch) = result else {
        return Err(format!("Expected Err(MapMismatch), got {result:?}").into());
    };
    Ok(())
}

fn edge_link_occupied(
    expected: BoardEdgeLinkError,
    f: impl FnOnce(&BoardEdge, &BoardEdge) -> Result<(), BoardEdgeLinkError>,
) -> Result<(), Box<dyn Error>> {
    let map = Map::new();
    let a = &map
        .add_board("A".into(), psi_map::layout::data::A.layout())
        .edge(Edge::Clock3);
    let b = &map
        .add_board("B".into(), psi_map::layout::data::B.layout())
        .edge(Edge::Clock6);
    let c = &map
        .add_board("C".into(), psi_map::layout::data::C.layout())
        .edge(Edge::Clock9);
    a.link(b)?;
    match f(a, c) {
        Err(err) if err == expected => Ok(()),
        result => Err(format!("Expected Err({expected:?}), got {result:?}").into()),
    }
}

#[test]
fn edge_link_occupied_self() -> Result<(), Box<dyn Error>> {
    edge_link_occupied(BoardEdgeLinkError::OccupiedSelf, |a, c| a.link(c))
}

#[test]
fn edge_link_occupied_other() -> Result<(), Box<dyn Error>> {
    edge_link_occupied(BoardEdgeLinkError::OccupiedOther, |a, c| c.link(a))
}
