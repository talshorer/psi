use std::{collections::HashSet, error::Error};

use psi_map::{Map, layout::Edge};

fn compare_adjacencies(map: Map, expected: &[(&str, &[&str])]) -> Result<(), Box<dyn Error>> {
    assert_eq!(
        map.boards().map(|b| b.lands().count()).sum::<usize>(),
        expected.len(),
    );
    for (key, neighbours) in expected {
        let expected_neighbours = neighbours
            .iter()
            .copied()
            .map(|s| s.parse().unwrap())
            .collect::<HashSet<_>>();
        let land = map.land(&key.parse().unwrap()).unwrap();
        let actual_neighbours = land
            .links()
            .map(|(land, _)| land.key())
            .collect::<HashSet<_>>();
        assert_eq!(
            actual_neighbours,
            expected_neighbours,
            "Neighbours mismatch for land {:?}",
            land.key()
        );
    }

    Ok(())
}

#[test]
fn adjacencies_3p_standard() -> Result<(), Box<dyn Error>> {
    let mut map = Map::new();
    let a = map.add_board("A".into(), psi_map::layout::data::A.layout());
    let b = map.add_board("B".into(), psi_map::layout::data::B.layout());
    let c = map.add_board("C".into(), psi_map::layout::data::C.layout());
    a.edge(Edge::Clock6).link(&b.edge(Edge::Clock9))?;
    b.edge(Edge::Clock6).link(&c.edge(Edge::Clock9))?;
    c.edge(Edge::Clock6).link(&a.edge(Edge::Clock9))?;

    compare_adjacencies(
        map,
        &[
            ("A0", &["A1", "A2", "A3"]),
            ("A1", &["A0", "A2", "A4", "A5", "A6"]),
            ("A2", &["A0", "A1", "A3", "A4"]),
            ("A3", &["A0", "A2", "A4", "C8"]),
            ("A4", &["A1", "A2", "A3", "A5", "C7", "C8"]),
            ("A5", &["A1", "A4", "A6", "A7", "A8", "C4", "C7"]),
            ("A6", &["A8", "A1", "A5"]),
            ("A7", &["A8", "A5", "B3", "B4", "B7", "C4"]),
            ("A8", &["A5", "A6", "A7", "B3"]),
            ("B0", &["B1", "B2", "B3"]),
            ("B1", &["B0", "B2", "B4", "B5", "B6"]),
            ("B2", &["B0", "B1", "B3", "B4"]),
            ("B3", &["B0", "B2", "B4", "A8", "A7"]),
            ("B4", &["B1", "B2", "B3", "B5", "B7", "A7"]),
            ("B5", &["B1", "B4", "B6", "B7"]),
            ("B6", &["B8", "B1", "B5", "B7"]),
            ("B7", &["B8", "B4", "B5", "B6", "A7", "C3", "C4"]),
            ("B8", &["B6", "B7", "C3"]),
            ("C0", &["C1", "C2", "C3"]),
            ("C1", &["C0", "C2", "C5", "C6"]),
            ("C2", &["C0", "C1", "C3", "C4", "C5"]),
            ("C3", &["C0", "C2", "C4", "B8", "B7"]),
            ("C4", &["C2", "C3", "C5", "C7", "B7", "A7", "A5"]),
            ("C5", &["C1", "C2", "C4", "C6", "C7"]),
            ("C6", &["C8", "C1", "C5", "C7"]),
            ("C7", &["C8", "C4", "C5", "C6", "A5", "A4"]),
            ("C8", &["C6", "C7", "A4", "A3"]),
        ],
    )
}

#[test]
fn adjacencies_6p_star() -> Result<(), Box<dyn Error>> {
    let mut map = Map::new();
    let a = map.add_board("A".into(), psi_map::layout::data::A.layout());
    let b = map.add_board("B".into(), psi_map::layout::data::B.layout());
    let c = map.add_board("C".into(), psi_map::layout::data::C.layout());
    let d = map.add_board("D".into(), psi_map::layout::data::D.layout());
    let e = map.add_board("E".into(), psi_map::layout::data::E.layout());
    let f = map.add_board("F".into(), psi_map::layout::data::F.layout());
    a.edge(Edge::Clock6).link(&b.edge(Edge::Clock3))?;
    b.edge(Edge::Clock6).link(&c.edge(Edge::Clock3))?;
    c.edge(Edge::Clock6).link(&d.edge(Edge::Clock3))?;
    d.edge(Edge::Clock6).link(&e.edge(Edge::Clock3))?;
    e.edge(Edge::Clock6).link(&f.edge(Edge::Clock3))?;
    f.edge(Edge::Clock6).link(&a.edge(Edge::Clock3))?;

    compare_adjacencies(
        map,
        &[
            ("A0", &["A1", "A2", "A3"]),
            ("A1", &["A0", "A2", "A4", "A5", "A6", "F7"]),
            ("A2", &["A0", "A1", "A3", "A4"]),
            ("A3", &["A0", "A2", "A4"]),
            ("A4", &["A1", "A2", "A3", "A5"]),
            ("A5", &["A1", "A4", "A6", "A7", "A8"]),
            ("A6", &["A8", "A1", "A5", "F7", "F8"]),
            ("A7", &["A8", "A5", "B8", "B6", "B1"]),
            ("A8", &["A5", "A6", "A7", "B8", "C8", "D8", "E8", "F8"]),
            ("B0", &["B1", "B2", "B3"]),
            ("B1", &["B0", "B2", "B4", "B5", "B6", "A7"]),
            ("B2", &["B0", "B1", "B3", "B4"]),
            ("B3", &["B0", "B2", "B4"]),
            ("B4", &["B1", "B2", "B3", "B5", "B7"]),
            ("B5", &["B1", "B4", "B6", "B7"]),
            ("B6", &["B8", "B1", "B5", "B7", "A7"]),
            ("B7", &["B8", "B4", "B5", "B6", "C8", "C6", "C1"]),
            ("B8", &["B6", "B7", "A8", "A7", "C8", "D8", "E8", "F8"]),
            ("C0", &["C1", "C2", "C3"]),
            ("C1", &["C0", "C2", "C5", "C6", "B7"]),
            ("C2", &["C0", "C1", "C3", "C4", "C5"]),
            ("C3", &["C0", "C2", "C4"]),
            ("C4", &["C2", "C3", "C5", "C7", "D1"]),
            ("C5", &["C1", "C2", "C4", "C6", "C7"]),
            ("C6", &["C8", "C1", "C5", "C7", "B7"]),
            ("C7", &["C8", "C4", "C5", "C6", "D1"]),
            (
                "C8",
                &["C6", "C7", "B8", "B7", "A8", "D8", "D1", "E8", "F8"],
            ),
            ("D0", &["D1", "D2", "D3"]),
            ("D1", &["D0", "D8", "D2", "D5", "D7", "C8", "C7", "C4"]),
            ("D2", &["D0", "D1", "D3", "D4", "D5"]),
            ("D3", &["D0", "D2", "D4"]),
            ("D4", &["D2", "D3", "D5", "D6"]),
            ("D5", &["D1", "D2", "D4", "D6", "D7"]),
            ("D6", &["D4", "D5", "D7", "E1"]),
            ("D7", &["D8", "D1", "D5", "D6", "E7", "E1"]),
            ("D8", &["D1", "D7", "C8", "B8", "A8", "E8", "E7", "F8"]),
            ("E0", &["E1", "E2", "E3"]),
            ("E1", &["E0", "E2", "E5", "E7", "D7", "D6"]),
            ("E2", &["E0", "E1", "E3", "E5"]),
            ("E3", &["E0", "E2", "E4", "E5"]),
            ("E4", &["E3", "E5", "E6", "E7"]),
            ("E5", &["E1", "E2", "E3", "E4", "E7"]),
            ("E6", &["E8", "E4", "E7", "F1"]),
            ("E7", &["E1", "E4", "E5", "E6", "E8", "D8", "D7"]),
            (
                "E8",
                &["E6", "E7", "D8", "C8", "B8", "A8", "F8", "F6", "F1"],
            ),
            ("F0", &["F1", "F2", "F3"]),
            ("F1", &["F0", "F2", "F5", "F6", "E8", "E6"]),
            ("F2", &["F0", "F1", "F3", "F4", "F5"]),
            ("F3", &["F0", "F2", "F4"]),
            ("F4", &["F2", "F3", "F5", "F7", "F8"]),
            ("F5", &["F1", "F2", "F4", "F6", "F8"]),
            ("F6", &["F8", "F1", "F5", "E8"]),
            ("F7", &["F8", "F4", "A1", "A6"]),
            (
                "F8",
                &["F4", "F5", "F6", "F7", "E8", "D8", "C8", "B8", "A8", "A6"],
            ),
        ],
    )
}
