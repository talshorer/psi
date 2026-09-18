use psi_map::LayoutCache;

#[derive(Debug, thiserror::Error)]
enum MainError {
    #[error("BoardEdgeLink({0})")]
    BoardEdgeLink(#[from] psi_map::board::BoardEdgeLinkError),
}

fn main() -> Result<(), MainError> {
    let lc = LayoutCache::new();
    let map = psi_map::Map::new();
    let a = map.add_board("A".into(), lc.layout(&psi_map::layout::data::A));
    let b = map.add_board("B".into(), lc.layout(&psi_map::layout::data::B));
    let c = map.add_board("C".into(), lc.layout(&psi_map::layout::data::C));
    a.edge(psi_map::layout::Edge::Clock6)
        .link(&b.edge(psi_map::layout::Edge::Clock9))?;
    b.edge(psi_map::layout::Edge::Clock6)
        .link(&c.edge(psi_map::layout::Edge::Clock9))?;
    c.edge(psi_map::layout::Edge::Clock6)
        .link(&a.edge(psi_map::layout::Edge::Clock9))?;

    for board in [a, b, c] {
        for land in board.lands() {
            for (link, distance) in land.links() {
                assert!(distance.0 == 1);
                println!(
                    "{:?} {:?} {:?} {:?} {:?}",
                    land.key(),
                    land.terrain(),
                    link.key(),
                    link.terrain(),
                    distance
                );
            }
        }
    }

    Ok(())
}
