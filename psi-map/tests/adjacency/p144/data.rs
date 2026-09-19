use std::rc::Rc;

use psi_map::{
    LayoutCache, Map,
    board::{BoardKey, LandKey},
    layout::{Corner, Edge, LandNum},
};

use super::{ContinentData, RegionData};

pub(super) static BLUE: ContinentData<'static> = ContinentData {
    rim: [
        RegionData {
            key: "🧀",
            p: &psi_map::layout::data::H,
            q: &psi_map::layout::data::E,
            r: &psi_map::layout::data::D,
            s: &psi_map::layout::data::G,
            t: &psi_map::layout::data::E,
            u: &psi_map::layout::data::H,
        },
        RegionData {
            key: "🌙",
            p: &psi_map::layout::data::C,
            q: &psi_map::layout::data::A,
            r: &psi_map::layout::data::C,
            s: &psi_map::layout::data::H,
            t: &psi_map::layout::data::A,
            u: &psi_map::layout::data::H,
        },
        RegionData {
            key: "🦋",
            p: &psi_map::layout::data::C,
            q: &psi_map::layout::data::G,
            r: &psi_map::layout::data::A,
            s: &psi_map::layout::data::F,
            t: &psi_map::layout::data::A,
            u: &psi_map::layout::data::D,
        },
        RegionData {
            key: "👻",
            p: &psi_map::layout::data::B,
            q: &psi_map::layout::data::A,
            r: &psi_map::layout::data::H,
            s: &psi_map::layout::data::C,
            t: &psi_map::layout::data::G,
            u: &psi_map::layout::data::G,
        },
        RegionData {
            key: "🧩",
            p: &psi_map::layout::data::B,
            q: &psi_map::layout::data::H,
            r: &psi_map::layout::data::H,
            s: &psi_map::layout::data::A,
            t: &psi_map::layout::data::E,
            u: &psi_map::layout::data::D,
        },
        RegionData {
            key: "🌵",
            p: &psi_map::layout::data::E,
            q: &psi_map::layout::data::H,
            r: &psi_map::layout::data::G,
            s: &psi_map::layout::data::H,
            t: &psi_map::layout::data::D,
            u: &psi_map::layout::data::F,
        },
    ],
    spokes: [
        RegionData {
            key: "🐍",
            p: &psi_map::layout::data::D,
            q: &psi_map::layout::data::E,
            r: &psi_map::layout::data::C,
            s: &psi_map::layout::data::A,
            t: &psi_map::layout::data::B,
            u: &psi_map::layout::data::B,
        },
        RegionData {
            key: "♾️",
            p: &psi_map::layout::data::H,
            q: &psi_map::layout::data::H,
            r: &psi_map::layout::data::B,
            s: &psi_map::layout::data::G,
            t: &psi_map::layout::data::F,
            u: &psi_map::layout::data::E,
        },
        RegionData {
            key: "🇲🇺",
            p: &psi_map::layout::data::A,
            q: &psi_map::layout::data::C,
            r: &psi_map::layout::data::A,
            s: &psi_map::layout::data::C,
            t: &psi_map::layout::data::C,
            u: &psi_map::layout::data::A,
        },
        RegionData {
            key: "⚽",
            p: &psi_map::layout::data::C,
            q: &psi_map::layout::data::B,
            r: &psi_map::layout::data::F,
            s: &psi_map::layout::data::C,
            t: &psi_map::layout::data::D,
            u: &psi_map::layout::data::E,
        },
        RegionData {
            key: "🌱",
            p: &psi_map::layout::data::B,
            q: &psi_map::layout::data::E,
            r: &psi_map::layout::data::G,
            s: &psi_map::layout::data::G,
            t: &psi_map::layout::data::H,
            u: &psi_map::layout::data::G,
        },
        RegionData {
            key: "🐀",
            p: &psi_map::layout::data::D,
            q: &psi_map::layout::data::E,
            r: &psi_map::layout::data::B,
            s: &psi_map::layout::data::A,
            t: &psi_map::layout::data::B,
            u: &psi_map::layout::data::F,
        },
    ],
    hub: [
        RegionData {
            key: "🏝️",
            p: &psi_map::layout::data::F,
            q: &psi_map::layout::data::D,
            r: &psi_map::layout::data::D,
            s: &psi_map::layout::data::G,
            t: &psi_map::layout::data::E,
            u: &psi_map::layout::data::H,
        },
        RegionData {
            key: "💖",
            p: &psi_map::layout::data::G,
            q: &psi_map::layout::data::A,
            r: &psi_map::layout::data::H,
            s: &psi_map::layout::data::H,
            t: &psi_map::layout::data::F,
            u: &psi_map::layout::data::B,
        },
        RegionData {
            key: "😎",
            p: &psi_map::layout::data::B,
            q: &psi_map::layout::data::F,
            r: &psi_map::layout::data::C,
            s: &psi_map::layout::data::G,
            t: &psi_map::layout::data::E,
            u: &psi_map::layout::data::C,
        },
    ],
};

pub(super) static ORANGE: ContinentData<'static> = ContinentData {
    rim: [
        RegionData {
            key: "🍌",
            p: &psi_map::layout::data::C,
            q: &psi_map::layout::data::B,
            r: &psi_map::layout::data::H,
            s: &psi_map::layout::data::B,
            t: &psi_map::layout::data::F,
            u: &psi_map::layout::data::D,
        },
        RegionData {
            key: "🐺",
            p: &psi_map::layout::data::G,
            q: &psi_map::layout::data::B,
            r: &psi_map::layout::data::E,
            s: &psi_map::layout::data::D,
            t: &psi_map::layout::data::F,
            u: &psi_map::layout::data::D,
        },
        RegionData {
            key: "🍕",
            p: &psi_map::layout::data::F,
            q: &psi_map::layout::data::G,
            r: &psi_map::layout::data::E,
            s: &psi_map::layout::data::F,
            t: &psi_map::layout::data::D,
            u: &psi_map::layout::data::G,
        },
        RegionData {
            key: "🍔",
            p: &psi_map::layout::data::A,
            q: &psi_map::layout::data::C,
            r: &psi_map::layout::data::E,
            s: &psi_map::layout::data::G,
            t: &psi_map::layout::data::G,
            u: &psi_map::layout::data::E,
        },
        RegionData {
            key: "🐦‍🔥",
            p: &psi_map::layout::data::C,
            q: &psi_map::layout::data::C,
            r: &psi_map::layout::data::G,
            s: &psi_map::layout::data::C,
            t: &psi_map::layout::data::E,
            u: &psi_map::layout::data::A,
        },
        RegionData {
            key: "📚",
            p: &psi_map::layout::data::G,
            q: &psi_map::layout::data::C,
            r: &psi_map::layout::data::F,
            s: &psi_map::layout::data::E,
            t: &psi_map::layout::data::E,
            u: &psi_map::layout::data::F,
        },
    ],
    spokes: [
        RegionData {
            key: "🥯",
            p: &psi_map::layout::data::A,
            q: &psi_map::layout::data::E,
            r: &psi_map::layout::data::G,
            s: &psi_map::layout::data::D,
            t: &psi_map::layout::data::F,
            u: &psi_map::layout::data::B,
        },
        RegionData {
            key: "🤠",
            p: &psi_map::layout::data::H,
            q: &psi_map::layout::data::A,
            r: &psi_map::layout::data::F,
            s: &psi_map::layout::data::B,
            t: &psi_map::layout::data::F,
            u: &psi_map::layout::data::F,
        },
        RegionData {
            key: "🍪",
            p: &psi_map::layout::data::A,
            q: &psi_map::layout::data::F,
            r: &psi_map::layout::data::B,
            s: &psi_map::layout::data::H,
            t: &psi_map::layout::data::A,
            u: &psi_map::layout::data::D,
        },
        RegionData {
            key: "🐉",
            p: &psi_map::layout::data::H,
            q: &psi_map::layout::data::D,
            r: &psi_map::layout::data::C,
            s: &psi_map::layout::data::H,
            t: &psi_map::layout::data::G,
            u: &psi_map::layout::data::B,
        },
        RegionData {
            key: "⛰️",
            p: &psi_map::layout::data::C,
            q: &psi_map::layout::data::G,
            r: &psi_map::layout::data::C,
            s: &psi_map::layout::data::H,
            t: &psi_map::layout::data::A,
            u: &psi_map::layout::data::D,
        },
        RegionData {
            key: "🎈",
            p: &psi_map::layout::data::B,
            q: &psi_map::layout::data::F,
            r: &psi_map::layout::data::D,
            s: &psi_map::layout::data::B,
            t: &psi_map::layout::data::E,
            u: &psi_map::layout::data::F,
        },
    ],
    hub: [
        RegionData {
            key: "⚡",
            p: &psi_map::layout::data::B,
            q: &psi_map::layout::data::A,
            r: &psi_map::layout::data::A,
            s: &psi_map::layout::data::F,
            t: &psi_map::layout::data::H,
            u: &psi_map::layout::data::C,
        },
        RegionData {
            key: "🤖",
            p: &psi_map::layout::data::B,
            q: &psi_map::layout::data::D,
            r: &psi_map::layout::data::E,
            s: &psi_map::layout::data::D,
            t: &psi_map::layout::data::E,
            u: &psi_map::layout::data::E,
        },
        RegionData {
            key: "🐑",
            p: &psi_map::layout::data::H,
            q: &psi_map::layout::data::A,
            r: &psi_map::layout::data::D,
            s: &psi_map::layout::data::B,
            t: &psi_map::layout::data::D,
            u: &psi_map::layout::data::A,
        },
    ],
};

pub(super) fn modifications(map: &Rc<Map>, lc: &Rc<LayoutCache>) {
    map.board(&BoardKey::from("🍪P")).unwrap().cast_down();
    map.land(&LandKey(BoardKey::from("🍪Q"), LandNum(8)))
        .unwrap()
        .cast_down();
    map.board(&BoardKey::from("♾️U")).unwrap().cast_down();
    {
        let mountain_r = map.board(&BoardKey::from("⛰️R")).unwrap();
        mountain_r.land(LandNum(1)).unwrap().sink();
        mountain_r.land(LandNum(2)).unwrap().sink();
        mountain_r.land(LandNum(3)).unwrap().sink();
    }
    map.board(&BoardKey::from("🍪U")).unwrap().cast_down();
    map.board(&BoardKey::from("🐀U")).unwrap().cast_down();
    map.land(&LandKey(BoardKey::from("🐀T"), LandNum(2)))
        .unwrap()
        .sink();
    map.board(&BoardKey::from("🇲🇺P")).unwrap().cast_down();
    {
        let board = map.add_board(BoardKey::from("👻X"), lc.layout(&psi_map::layout::data::F));
        board
            .edge(Edge::Clock3)
            .link(
                &map.board(&BoardKey::from("👻T"))
                    .unwrap()
                    .edge(Edge::Clock6),
            )
            .unwrap();
        board
            .edge(Edge::Clock6)
            .link(
                &map.board(&BoardKey::from("🇲🇺U"))
                    .unwrap()
                    .edge(Edge::Clock6),
            )
            .unwrap();
        let clock11 = board.corner(Corner::Clock11);
        clock11.link(
            &map.board(&BoardKey::from("🇲🇺Q"))
                .unwrap()
                .corner(Corner::Clock7),
        );
        clock11.link(
            &map.board(&BoardKey::from("🇲🇺R"))
                .unwrap()
                .corner(Corner::Clock5),
        );
    }
    {
        let board = map.add_board(BoardKey::from("👻Y"), lc.layout(&psi_map::layout::data::C));
        board
            .edge(Edge::Clock9)
            .link(
                &map.board(&BoardKey::from("👻P"))
                    .unwrap()
                    .edge(Edge::Clock3),
            )
            .unwrap();
    }
    map.board(&BoardKey::from("🤖U")).unwrap().cast_down();
    map.board(&BoardKey::from("🤖P")).unwrap().cast_down();
    map.land(&LandKey(BoardKey::from("🍪S"), LandNum(1)))
        .unwrap()
        .cast_down();
    map.board(&BoardKey::from("🤖T")).unwrap().cast_down();
    map.land(&LandKey(BoardKey::from("⛰️R"), LandNum(4)))
        .unwrap()
        .sink();
    map.land(&LandKey(BoardKey::from("⛰️S"), LandNum(7)))
        .unwrap()
        .sink();
    map.land(&LandKey(BoardKey::from("🐀T"), LandNum(3)))
        .unwrap()
        .sink();
    map.board(&BoardKey::from("🤠U")).unwrap().cast_down();
    {
        let board = map.add_board(BoardKey::from("👻Z"), lc.layout(&psi_map::layout::data::E));
        board
            .edge(Edge::Clock9)
            .link(
                &map.board(&BoardKey::from("👻Y"))
                    .unwrap()
                    .edge(Edge::Clock3),
            )
            .unwrap();
    }
    {
        let board = map.add_board(BoardKey::from("👻V"), lc.layout(&psi_map::layout::data::D));
        board
            .edge(Edge::Clock6)
            .link(
                &map.board(&BoardKey::from("👻Z"))
                    .unwrap()
                    .edge(Edge::Clock3),
            )
            .unwrap();
    }
    {
        let board = map.add_board(BoardKey::from("👻W"), lc.layout(&psi_map::layout::data::G));
        board
            .edge(Edge::Clock3)
            .link(
                &map.board(&BoardKey::from("👻V"))
                    .unwrap()
                    .edge(Edge::Clock3),
            )
            .unwrap();
        let ocean = board.ocean();
        ocean.link(&map.board(&BoardKey::from("🧩R")).unwrap().ocean());
        ocean.link(&map.board(&BoardKey::from("🧩S")).unwrap().ocean());
        ocean.link(&map.board(&BoardKey::from("🧩T")).unwrap().ocean());
        ocean.link(&map.board(&BoardKey::from("🧩U")).unwrap().ocean());
    }
    {
        let board = map.add_board(BoardKey::from("👻N"), lc.layout(&psi_map::layout::data::E));
        board
            .edge(Edge::Clock6)
            .link(
                &map.board(&BoardKey::from("👻W"))
                    .unwrap()
                    .edge(Edge::Clock6),
            )
            .unwrap();
    }
    {
        let board = map.add_board(BoardKey::from("👻K"), lc.layout(&psi_map::layout::data::B));
        board
            .edge(Edge::Clock6)
            .link(
                &map.board(&BoardKey::from("👻N"))
                    .unwrap()
                    .edge(Edge::Clock3),
            )
            .unwrap();
    }
    map.board(&BoardKey::from("🤖S")).unwrap().cast_down();
    map.land(&LandKey(BoardKey::from("🤖R"), LandNum(1)))
        .unwrap()
        .cast_down();
    map.land(&LandKey(BoardKey::from("🤖R"), LandNum(2)))
        .unwrap()
        .cast_down();
    map.land(&LandKey(BoardKey::from("🤖R"), LandNum(3)))
        .unwrap()
        .cast_down();
    map.land(&LandKey(BoardKey::from("🤖Q"), LandNum(1)))
        .unwrap()
        .cast_down();
    map.land(&LandKey(BoardKey::from("🤖Q"), LandNum(2)))
        .unwrap()
        .cast_down();
    map.land(&LandKey(BoardKey::from("🤖Q"), LandNum(3)))
        .unwrap()
        .cast_down();
    map.land(&LandKey(BoardKey::from("⚡U"), LandNum(1)))
        .unwrap()
        .cast_down();
    map.land(&LandKey(BoardKey::from("⚡U"), LandNum(2)))
        .unwrap()
        .cast_down();
    map.land(&LandKey(BoardKey::from("⚡U"), LandNum(3)))
        .unwrap()
        .cast_down();
    map.land(&LandKey(BoardKey::from("🏝️S"), LandNum(8)))
        .unwrap()
        .cast_down();
    map.land(&LandKey(BoardKey::from("🦋P"), LandNum(8)))
        .unwrap()
        .cast_down();
    map.land(&LandKey(BoardKey::from("👻Y"), LandNum(3)))
        .unwrap()
        .cast_down();
    map.land(&LandKey(BoardKey::from("👻Y"), LandNum(4)))
        .unwrap()
        .cast_down();
    map.board(&BoardKey::from("🦋U")).unwrap().cast_down();
}

pub(super) static EXPECTED: &[super::super::CompareAdjaceciesExpectedSingleLand<'static>] = &[
    (
        "🧀P0",
        (&["🧀P1", "🧀P2", "🧀P3"], &["🍌P0", "🍌P1", "🍌P2", "🍌P3"]),
    ),
    (
        "🧀P1",
        (
            &["🧀P8", "🧀P2", "🧀P6", "🧀P0"],
            &["🍌P0", "🍌P1", "🍌P2", "🍌P3"],
        ),
    ),
    (
        "🧀P2",
        (
            &["🧀P1", "🧀P3", "🧀P5", "🧀P6", "🧀P0"],
            &["🍌P0", "🍌P1", "🍌P2", "🍌P3"],
        ),
    ),
    (
        "🧀P3",
        (
            &["🧀P2", "🧀P4", "🧀P5", "🧀P0", "🧀Q6", "🧀Q4", "🧀R8"],
            &["🍌P0", "🍌P1", "🍌P2", "🍌P3"],
        ),
    ),
    (
        "🧀P4",
        (&["🧀P3", "🧀P5", "🧀P7", "🧀Q4", "🧀Q3", "🐍P1"], &[]),
    ),
    ("🧀P5", (&["🧀P2", "🧀P3", "🧀P4", "🧀P6", "🧀P7"], &[])),
    ("🧀P6", (&["🧀P1", "🧀P2", "🧀P5", "🧀P7", "🧀P8"], &[])),
    ("🧀P7", (&["🧀P8", "🧀P4", "🧀P5", "🧀P6", "🐍P1"], &[])),
    (
        "🧀P8",
        (&["🧀P1", "🧀P6", "🧀P7", "🐍P1", "🐍P8", "🌙U3"], &[]),
    ),
    (
        "🧀Q0",
        (&["🧀Q1", "🧀Q2", "🧀Q3"], &["😎U0", "😎U1", "😎U2", "😎U3"]),
    ),
    (
        "🧀Q1",
        (
            &["🧀Q2", "🧀Q5", "🧀Q7", "🧀Q0"],
            &["😎U0", "😎U1", "😎U2", "😎U3"],
        ),
    ),
    (
        "🧀Q2",
        (
            &["🧀Q1", "🧀Q3", "🧀Q5", "🧀Q0"],
            &["😎U0", "😎U1", "😎U2", "😎U3"],
        ),
    ),
    (
        "🧀Q3",
        (
            &["🧀Q2", "🧀Q4", "🧀Q5", "🧀Q0", "🧀P4", "🐍P1"],
            &["😎U0", "😎U1", "😎U2", "😎U3"],
        ),
    ),
    (
        "🧀Q4",
        (&["🧀Q3", "🧀Q5", "🧀Q6", "🧀Q7", "🧀P3", "🧀P4"], &[]),
    ),
    ("🧀Q5", (&["🧀Q1", "🧀Q2", "🧀Q3", "🧀Q4", "🧀Q7"], &[])),
    ("🧀Q6", (&["🧀Q8", "🧀Q4", "🧀Q7", "🧀P3", "🧀R8"], &[])),
    ("🧀Q7", (&["🧀Q1", "🧀Q4", "🧀Q5", "🧀Q6", "🧀Q8"], &[])),
    (
        "🧀Q8",
        (&["🧀Q6", "🧀Q7", "🧀R8", "🧀R7", "🧀R6", "🧀S8"], &[]),
    ),
    (
        "🧀R0",
        (&["🧀R1", "🧀R2", "🧀R3"], &["🍌R0", "🍌R1", "🍌R2", "🍌R3"]),
    ),
    (
        "🧀R1",
        (
            &["🧀R8", "🧀R2", "🧀R5", "🧀R7", "🧀R0"],
            &["🍌R0", "🍌R1", "🍌R2", "🍌R3"],
        ),
    ),
    (
        "🧀R2",
        (
            &["🧀R1", "🧀R3", "🧀R4", "🧀R5", "🧀R0"],
            &["🍌R0", "🍌R1", "🍌R2", "🍌R3"],
        ),
    ),
    (
        "🧀R3",
        (
            &["🧀R2", "🧀R4", "🧀R0", "🧀S1"],
            &["🍌R0", "🍌R1", "🍌R2", "🍌R3"],
        ),
    ),
    (
        "🧀R4",
        (
            &["🧀R2", "🧀R3", "🧀R5", "🧀R6", "🧀S1", "🧀S6", "🧀S8"],
            &[],
        ),
    ),
    ("🧀R5", (&["🧀R1", "🧀R2", "🧀R4", "🧀R6", "🧀R7"], &[])),
    ("🧀R6", (&["🧀R4", "🧀R5", "🧀R7", "🧀Q8", "🧀S8"], &[])),
    ("🧀R7", (&["🧀R8", "🧀R1", "🧀R5", "🧀R6", "🧀Q8"], &[])),
    ("🧀R8", (&["🧀R1", "🧀R7", "🧀Q6", "🧀Q8", "🧀P3"], &[])),
    (
        "🧀S0",
        (&["🧀S1", "🧀S2", "🧀S3"], &["🍌S0", "🍌S1", "🍌S2", "🍌S3"]),
    ),
    (
        "🧀S1",
        (
            &["🧀S2", "🧀S6", "🧀S0", "🧀R3", "🧀R4"],
            &["🍌S0", "🍌S1", "🍌S2", "🍌S3"],
        ),
    ),
    (
        "🧀S2",
        (
            &["🧀S1", "🧀S3", "🧀S4", "🧀S5", "🧀S6", "🧀S0"],
            &["🍌S0", "🍌S1", "🍌S2", "🍌S3"],
        ),
    ),
    (
        "🧀S3",
        (
            &["🧀S2", "🧀S4", "🧀S0", "🧀T1", "🧀T7"],
            &["🍌S0", "🍌S1", "🍌S2", "🍌S3"],
        ),
    ),
    (
        "🧀S4",
        (&["🧀S2", "🧀S3", "🧀S5", "🧀S7", "🧀T7", "🧀T8"], &[]),
    ),
    ("🧀S5", (&["🧀S2", "🧀S4", "🧀S6", "🧀S7", "🧀S8"], &[])),
    ("🧀S6", (&["🧀S8", "🧀S1", "🧀S2", "🧀S5", "🧀R4"], &[])),
    ("🧀S7", (&["🧀S8", "🧀S4", "🧀S5", "🧀T8"], &[])),
    (
        "🧀S8",
        (&["🧀S5", "🧀S6", "🧀S7", "🧀R4", "🧀R6", "🧀Q8"], &[]),
    ),
    (
        "🧀T0",
        (&["🧀T1", "🧀T2", "🧀T3"], &["🍌T0", "🍌T1", "🍌T2", "🍌T3"]),
    ),
    (
        "🧀T1",
        (
            &["🧀T2", "🧀T5", "🧀T7", "🧀T0", "🧀S3"],
            &["🍌T0", "🍌T1", "🍌T2", "🍌T3"],
        ),
    ),
    (
        "🧀T2",
        (
            &["🧀T1", "🧀T3", "🧀T5", "🧀T0"],
            &["🍌T0", "🍌T1", "🍌T2", "🍌T3"],
        ),
    ),
    (
        "🧀T3",
        (
            &["🧀T2", "🧀T4", "🧀T5", "🧀T0", "🧀U1"],
            &["🍌T0", "🍌T1", "🍌T2", "🍌T3"],
        ),
    ),
    (
        "🧀T4",
        (&["🧀T3", "🧀T5", "🧀T6", "🧀T7", "🧀U1", "🧀U8"], &[]),
    ),
    ("🧀T5", (&["🧀T1", "🧀T2", "🧀T3", "🧀T4", "🧀T7"], &[])),
    ("🧀T6", (&["🧀T8", "🧀T4", "🧀T7", "🧀U8"], &[])),
    (
        "🧀T7",
        (
            &["🧀T1", "🧀T4", "🧀T5", "🧀T6", "🧀T8", "🧀S3", "🧀S4"],
            &[],
        ),
    ),
    ("🧀T8", (&["🧀T6", "🧀T7", "🧀S4", "🧀S7"], &[])),
    (
        "🧀U0",
        (&["🧀U1", "🧀U2", "🧀U3"], &["🍌U0", "🍌U1", "🍌U2", "🍌U3"]),
    ),
    (
        "🧀U1",
        (
            &["🧀U8", "🧀U2", "🧀U6", "🧀U0", "🧀T3", "🧀T4"],
            &["🍌U0", "🍌U1", "🍌U2", "🍌U3"],
        ),
    ),
    (
        "🧀U2",
        (
            &["🧀U1", "🧀U3", "🧀U5", "🧀U6", "🧀U0"],
            &["🍌U0", "🍌U1", "🍌U2", "🍌U3"],
        ),
    ),
    (
        "🧀U3",
        (
            &["🧀U2", "🧀U4", "🧀U5", "🧀U0", "🐀P7", "🐀P8", "🌵P8"],
            &["🍌U0", "🍌U1", "🍌U2", "🍌U3"],
        ),
    ),
    (
        "🧀U4",
        (&["🧀U3", "🧀U5", "🧀U7", "🐀P6", "🐀P7", "🐀Q8"], &[]),
    ),
    ("🧀U5", (&["🧀U2", "🧀U3", "🧀U4", "🧀U6", "🧀U7"], &[])),
    ("🧀U6", (&["🧀U1", "🧀U2", "🧀U5", "🧀U7", "🧀U8"], &[])),
    ("🧀U7", (&["🧀U8", "🧀U4", "🧀U5", "🧀U6"], &[])),
    ("🧀U8", (&["🧀U1", "🧀U6", "🧀U7", "🧀T4", "🧀T6"], &[])),
    (
        "🌙P0",
        (&["🌙P1", "🌙P2", "🌙P3"], &["🐺P0", "🐺P1", "🐺P2", "🐺P3"]),
    ),
    (
        "🌙P1",
        (
            &["🌙P2", "🌙P5", "🌙P6", "🌙P0"],
            &["🐺P0", "🐺P1", "🐺P2", "🐺P3"],
        ),
    ),
    (
        "🌙P2",
        (
            &["🌙P1", "🌙P3", "🌙P4", "🌙P5", "🌙P0"],
            &["🐺P0", "🐺P1", "🐺P2", "🐺P3"],
        ),
    ),
    (
        "🌙P3",
        (
            &["🌙P2", "🌙P4", "🌙P0", "🌙Q7", "🌙Q5", "🌙Q4", "🌙R8"],
            &["🐺P0", "🐺P1", "🐺P2", "🐺P3"],
        ),
    ),
    (
        "🌙P4",
        (
            &["🌙P2", "🌙P3", "🌙P5", "🌙P7", "🌙Q4", "🌙Q3", "♾️P1"],
            &[],
        ),
    ),
    ("🌙P5", (&["🌙P1", "🌙P2", "🌙P4", "🌙P6", "🌙P7"], &[])),
    ("🌙P6", (&["🌙P8", "🌙P1", "🌙P5", "🌙P7"], &[])),
    ("🌙P7", (&["🌙P8", "🌙P4", "🌙P5", "🌙P6", "♾️P1"], &[])),
    ("🌙P8", (&["🌙P6", "🌙P7", "♾️P1", "♾️P8"], &[])),
    (
        "🌙Q0",
        (&["🌙Q1", "🌙Q2", "🌙Q3"], &["🏝️Q0", "🏝️Q1", "🏝️Q2", "🏝️Q3"]),
    ),
    (
        "🌙Q1",
        (
            &["🌙Q2", "🌙Q4", "🌙Q5", "🌙Q6", "🌙Q0"],
            &["🏝️Q0", "🏝️Q1", "🏝️Q2", "🏝️Q3"],
        ),
    ),
    (
        "🌙Q2",
        (
            &["🌙Q1", "🌙Q3", "🌙Q4", "🌙Q0"],
            &["🏝️Q0", "🏝️Q1", "🏝️Q2", "🏝️Q3"],
        ),
    ),
    (
        "🌙Q3",
        (
            &["🌙Q2", "🌙Q4", "🌙Q0", "🌙P4", "♾️P1"],
            &["🏝️Q0", "🏝️Q1", "🏝️Q2", "🏝️Q3"],
        ),
    ),
    (
        "🌙Q4",
        (&["🌙Q1", "🌙Q2", "🌙Q3", "🌙Q5", "🌙P3", "🌙P4"], &[]),
    ),
    (
        "🌙Q5",
        (&["🌙Q1", "🌙Q4", "🌙Q6", "🌙Q7", "🌙Q8", "🌙P3"], &[]),
    ),
    ("🌙Q6", (&["🌙Q8", "🌙Q1", "🌙Q5"], &[])),
    ("🌙Q7", (&["🌙Q8", "🌙Q5", "🌙P3", "🌙R8", "🌙R7"], &[])),
    (
        "🌙Q8",
        (
            &["🌙Q5", "🌙Q6", "🌙Q7", "🌙R7", "🌙R4", "🌙S8", "🐍U3"],
            &[],
        ),
    ),
    (
        "🌙R0",
        (&["🌙R1", "🌙R2", "🌙R3"], &["🐺R0", "🐺R1", "🐺R2", "🐺R3"]),
    ),
    (
        "🌙R1",
        (
            &["🌙R2", "🌙R5", "🌙R6", "🌙R0"],
            &["🐺R0", "🐺R1", "🐺R2", "🐺R3"],
        ),
    ),
    (
        "🌙R2",
        (
            &["🌙R1", "🌙R3", "🌙R4", "🌙R5", "🌙R0"],
            &["🐺R0", "🐺R1", "🐺R2", "🐺R3"],
        ),
    ),
    (
        "🌙R3",
        (
            &["🌙R2", "🌙R4", "🌙R0", "🌙S1"],
            &["🐺R0", "🐺R1", "🐺R2", "🐺R3"],
        ),
    ),
    (
        "🌙R4",
        (
            &[
                "🌙R2", "🌙R3", "🌙R5", "🌙R7", "🌙Q8", "🌙S1", "🌙S8", "🐍U3",
            ],
            &[],
        ),
    ),
    ("🌙R5", (&["🌙R1", "🌙R2", "🌙R4", "🌙R6", "🌙R7"], &[])),
    ("🌙R6", (&["🌙R8", "🌙R1", "🌙R5", "🌙R7"], &[])),
    (
        "🌙R7",
        (&["🌙R8", "🌙R4", "🌙R5", "🌙R6", "🌙Q7", "🌙Q8"], &[]),
    ),
    ("🌙R8", (&["🌙R6", "🌙R7", "🌙Q7", "🌙P3"], &[])),
    (
        "🌙S0",
        (&["🌙S1", "🌙S2", "🌙S3"], &["🐺S0", "🐺S1", "🐺S2", "🐺S3"]),
    ),
    (
        "🌙S1",
        (
            &["🌙S8", "🌙S2", "🌙S6", "🌙S0", "🌙R3", "🌙R4"],
            &["🐺S0", "🐺S1", "🐺S2", "🐺S3"],
        ),
    ),
    (
        "🌙S2",
        (
            &["🌙S1", "🌙S3", "🌙S5", "🌙S6", "🌙S0"],
            &["🐺S0", "🐺S1", "🐺S2", "🐺S3"],
        ),
    ),
    (
        "🌙S3",
        (
            &["🌙S2", "🌙S4", "🌙S5", "🌙S0", "🌙T1", "🌙T6"],
            &["🐺S0", "🐺S1", "🐺S2", "🐺S3"],
        ),
    ),
    (
        "🌙S4",
        (
            &["🌙S3", "🌙S5", "🌙S7", "🌙T6", "🌙T8", "🐍U4", "🐍U7"],
            &[],
        ),
    ),
    ("🌙S5", (&["🌙S2", "🌙S3", "🌙S4", "🌙S6", "🌙S7"], &[])),
    ("🌙S6", (&["🌙S1", "🌙S2", "🌙S5", "🌙S7", "🌙S8"], &[])),
    (
        "🌙S7",
        (&["🌙S8", "🌙S4", "🌙S5", "🌙S6", "🐍U3", "🐍U4"], &[]),
    ),
    (
        "🌙S8",
        (&["🌙S1", "🌙S6", "🌙S7", "🌙R4", "🌙Q8", "🐍U3"], &[]),
    ),
    (
        "🌙T0",
        (&["🌙T1", "🌙T2", "🌙T3"], &["🐺T0", "🐺T1", "🐺T2", "🐺T3"]),
    ),
    (
        "🌙T1",
        (
            &["🌙T2", "🌙T4", "🌙T5", "🌙T6", "🌙T0", "🌙S3"],
            &["🐺T0", "🐺T1", "🐺T2", "🐺T3"],
        ),
    ),
    (
        "🌙T2",
        (
            &["🌙T1", "🌙T3", "🌙T4", "🌙T0"],
            &["🐺T0", "🐺T1", "🐺T2", "🐺T3"],
        ),
    ),
    (
        "🌙T3",
        (
            &["🌙T2", "🌙T4", "🌙T0", "🌙U1"],
            &["🐺T0", "🐺T1", "🐺T2", "🐺T3"],
        ),
    ),
    (
        "🌙T4",
        (&["🌙T1", "🌙T2", "🌙T3", "🌙T5", "🌙U1", "🌙U8"], &[]),
    ),
    (
        "🌙T5",
        (&["🌙T1", "🌙T4", "🌙T6", "🌙T7", "🌙T8", "🌙U8"], &[]),
    ),
    ("🌙T6", (&["🌙T8", "🌙T1", "🌙T5", "🌙S3", "🌙S4"], &[])),
    ("🌙T7", (&["🌙T8", "🌙T5", "🌙U8"], &[])),
    ("🌙T8", (&["🌙T5", "🌙T6", "🌙T7", "🌙S4", "🐍U7"], &[])),
    (
        "🌙U0",
        (&["🌙U1", "🌙U2", "🌙U3"], &["🐺U0", "🐺U1", "🐺U2", "🐺U3"]),
    ),
    (
        "🌙U1",
        (
            &["🌙U8", "🌙U2", "🌙U6", "🌙U0", "🌙T3", "🌙T4"],
            &["🐺U0", "🐺U1", "🐺U2", "🐺U3"],
        ),
    ),
    (
        "🌙U2",
        (
            &["🌙U1", "🌙U3", "🌙U5", "🌙U6", "🌙U0"],
            &["🐺U0", "🐺U1", "🐺U2", "🐺U3"],
        ),
    ),
    (
        "🌙U3",
        (
            &["🌙U2", "🌙U4", "🌙U5", "🌙U0", "🐍P7", "🐍P8", "🧀P8"],
            &["🐺U0", "🐺U1", "🐺U2", "🐺U3"],
        ),
    ),
    (
        "🌙U4",
        (&["🌙U3", "🌙U5", "🌙U7", "🐍P6", "🐍P7", "🐍Q8"], &[]),
    ),
    ("🌙U5", (&["🌙U2", "🌙U3", "🌙U4", "🌙U6", "🌙U7"], &[])),
    ("🌙U6", (&["🌙U1", "🌙U2", "🌙U5", "🌙U7", "🌙U8"], &[])),
    ("🌙U7", (&["🌙U8", "🌙U4", "🌙U5", "🌙U6"], &[])),
    (
        "🌙U8",
        (&["🌙U1", "🌙U6", "🌙U7", "🌙T4", "🌙T5", "🌙T7"], &[]),
    ),
    (
        "🦋P0",
        (&["🦋P1", "🦋P2", "🦋P3"], &["🍕P0", "🍕P1", "🍕P2", "🍕P3"]),
    ),
    (
        "🦋P1",
        (
            &["🦋P2", "🦋P5", "🦋P6", "🦋P0"],
            &["🍕P0", "🍕P1", "🍕P2", "🍕P3"],
        ),
    ),
    (
        "🦋P2",
        (
            &["🦋P1", "🦋P3", "🦋P4", "🦋P5", "🦋P0"],
            &["🍕P0", "🍕P1", "🍕P2", "🍕P3"],
        ),
    ),
    (
        "🦋P3",
        (
            &["🦋P2", "🦋P4", "🦋P0", "🦋Q7", "🦋Q4", "🦋Q3", "🦋R8"],
            &["🍕P0", "🍕P1", "🍕P2", "🍕P3"],
        ),
    ),
    ("🦋P4", (&["🦋P2", "🦋P3", "🦋P5", "🦋P7", "🦋Q3"], &[])),
    ("🦋P5", (&["🦋P1", "🦋P2", "🦋P4", "🦋P6", "🦋P7"], &[])),
    ("🦋P6", (&["🦋P1", "🦋P5", "🦋P7"], &[])),
    ("🦋P7", (&["🦋P4", "🦋P5", "🦋P6"], &[])),
    (
        "🦋Q0",
        (&["🦋Q1", "🦋Q2", "🦋Q3"], &["🏝️U0", "🏝️U1", "🏝️U2", "🏝️U3"]),
    ),
    (
        "🦋Q1",
        (&["🦋Q2", "🦋Q6", "🦋Q0"], &["🏝️U0", "🏝️U1", "🏝️U2", "🏝️U3"]),
    ),
    (
        "🦋Q2",
        (
            &["🦋Q1", "🦋Q3", "🦋Q4", "🦋Q5", "🦋Q6", "🦋Q0"],
            &["🏝️U0", "🏝️U1", "🏝️U2", "🏝️U3"],
        ),
    ),
    (
        "🦋Q3",
        (
            &["🦋Q2", "🦋Q4", "🦋Q0", "🦋P3", "🦋P4"],
            &["🏝️U0", "🏝️U1", "🏝️U2", "🏝️U3"],
        ),
    ),
    ("🦋Q4", (&["🦋Q2", "🦋Q3", "🦋Q5", "🦋Q7", "🦋P3"], &[])),
    ("🦋Q5", (&["🦋Q2", "🦋Q4", "🦋Q6", "🦋Q7", "🦋Q8"], &[])),
    ("🦋Q6", (&["🦋Q8", "🦋Q1", "🦋Q2", "🦋Q5"], &[])),
    (
        "🦋Q7",
        (&["🦋Q8", "🦋Q4", "🦋Q5", "🦋P3", "🦋R8", "🦋R7"], &[]),
    ),
    ("🦋Q8", (&["🦋Q5", "🦋Q6", "🦋Q7", "🦋R7", "🦋S8"], &[])),
    (
        "🦋R0",
        (&["🦋R1", "🦋R2", "🦋R3"], &["🍕R0", "🍕R1", "🍕R2", "🍕R3"]),
    ),
    (
        "🦋R1",
        (
            &["🦋R2", "🦋R4", "🦋R5", "🦋R6", "🦋R0"],
            &["🍕R0", "🍕R1", "🍕R2", "🍕R3"],
        ),
    ),
    (
        "🦋R2",
        (
            &["🦋R1", "🦋R3", "🦋R4", "🦋R0"],
            &["🍕R0", "🍕R1", "🍕R2", "🍕R3"],
        ),
    ),
    (
        "🦋R3",
        (
            &["🦋R2", "🦋R4", "🦋R0", "🦋S1", "🦋S6"],
            &["🍕R0", "🍕R1", "🍕R2", "🍕R3"],
        ),
    ),
    ("🦋R4", (&["🦋R1", "🦋R2", "🦋R3", "🦋R5", "🦋S6"], &[])),
    (
        "🦋R5",
        (
            &["🦋R1", "🦋R4", "🦋R6", "🦋R7", "🦋R8", "🦋S6", "🦋S8"],
            &[],
        ),
    ),
    ("🦋R6", (&["🦋R8", "🦋R1", "🦋R5"], &[])),
    ("🦋R7", (&["🦋R8", "🦋R5", "🦋Q7", "🦋Q8", "🦋S8"], &[])),
    ("🦋R8", (&["🦋R5", "🦋R6", "🦋R7", "🦋Q7", "🦋P3"], &[])),
    (
        "🦋S0",
        (&["🦋S1", "🦋S2", "🦋S3"], &["🍕S0", "🍕S1", "🍕S2", "🍕S3"]),
    ),
    (
        "🦋S1",
        (
            &["🦋S2", "🦋S5", "🦋S6", "🦋S0", "🦋R3"],
            &["🍕S0", "🍕S1", "🍕S2", "🍕S3"],
        ),
    ),
    (
        "🦋S2",
        (
            &["🦋S1", "🦋S3", "🦋S4", "🦋S5", "🦋S0"],
            &["🍕S0", "🍕S1", "🍕S2", "🍕S3"],
        ),
    ),
    (
        "🦋S3",
        (
            &["🦋S2", "🦋S4", "🦋S0", "🦋T1", "🦋T6"],
            &["🍕S0", "🍕S1", "🍕S2", "🍕S3"],
        ),
    ),
    (
        "🦋S4",
        (
            &["🦋S2", "🦋S3", "🦋S5", "🦋S7", "🦋S8", "🦋T6", "🦋T8"],
            &[],
        ),
    ),
    ("🦋S5", (&["🦋S1", "🦋S2", "🦋S4", "🦋S6", "🦋S8"], &[])),
    (
        "🦋S6",
        (&["🦋S8", "🦋S1", "🦋S5", "🦋R3", "🦋R4", "🦋R5"], &[]),
    ),
    ("🦋S7", (&["🦋S8", "🦋S4", "🦋T8"], &[])),
    (
        "🦋S8",
        (
            &["🦋S4", "🦋S5", "🦋S6", "🦋S7", "🦋R5", "🦋R7", "🦋Q8"],
            &[],
        ),
    ),
    (
        "🦋T0",
        (&["🦋T1", "🦋T2", "🦋T3"], &["🍕T0", "🍕T1", "🍕T2", "🍕T3"]),
    ),
    (
        "🦋T1",
        (
            &["🦋T2", "🦋T4", "🦋T5", "🦋T6", "🦋T0", "🦋S3"],
            &["🍕T0", "🍕T1", "🍕T2", "🍕T3"],
        ),
    ),
    (
        "🦋T2",
        (
            &["🦋T1", "🦋T3", "🦋T4", "🦋T0"],
            &["🍕T0", "🍕T1", "🍕T2", "🍕T3"],
        ),
    ),
    (
        "🦋T3",
        (&["🦋T2", "🦋T4", "🦋T0"], &["🍕T0", "🍕T1", "🍕T2", "🍕T3"]),
    ),
    ("🦋T4", (&["🦋T1", "🦋T2", "🦋T3", "🦋T5"], &[])),
    ("🦋T5", (&["🦋T1", "🦋T4", "🦋T6", "🦋T7", "🦋T8"], &[])),
    ("🦋T6", (&["🦋T8", "🦋T1", "🦋T5", "🦋S3", "🦋S4"], &[])),
    ("🦋T7", (&["🦋T8", "🦋T5"], &[])),
    ("🦋T8", (&["🦋T5", "🦋T6", "🦋T7", "🦋S4", "🦋S7"], &[])),
    (
        "👻P0",
        (&["👻P1", "👻P2", "👻P3"], &["🍔P0", "🍔P1", "🍔P2", "🍔P3"]),
    ),
    (
        "👻P1",
        (
            &["👻P2", "👻P4", "👻P5", "👻P6", "👻P0"],
            &["🍔P0", "🍔P1", "🍔P2", "🍔P3"],
        ),
    ),
    (
        "👻P2",
        (
            &["👻P1", "👻P3", "👻P4", "👻P0"],
            &["🍔P0", "🍔P1", "🍔P2", "🍔P3"],
        ),
    ),
    (
        "👻P3",
        (
            &[
                "👻P2", "👻P4", "👻P0", "👻Q7", "👻Q5", "👻Q4", "👻Q3", "👻R8",
            ],
            &["🍔P0", "🍔P1", "🍔P2", "🍔P3"],
        ),
    ),
    (
        "👻P4",
        (&["👻P1", "👻P2", "👻P3", "👻P5", "👻P7", "👻Q3"], &[]),
    ),
    ("👻P5", (&["👻P1", "👻P4", "👻P6", "👻P7"], &[])),
    ("👻P6", (&["👻P8", "👻P1", "👻P5", "👻P7"], &[])),
    (
        "👻P7",
        (
            &[
                "👻P8", "👻P4", "👻P5", "👻P6", "👻Q3", "⚽P1", "⚽P6", "⚽P8",
            ],
            &[],
        ),
    ),
    ("👻P8", (&["👻P6", "👻P7", "⚽P8", "🧩U3"], &[])),
    (
        "👻Q0",
        (&["👻Q1", "👻Q2", "👻Q3"], &["💖Q0", "💖Q1", "💖Q2", "💖Q3"]),
    ),
    (
        "👻Q1",
        (
            &["👻Q2", "👻Q4", "👻Q5", "👻Q6", "👻Q0"],
            &["💖Q0", "💖Q1", "💖Q2", "💖Q3"],
        ),
    ),
    (
        "👻Q2",
        (
            &["👻Q1", "👻Q3", "👻Q4", "👻Q0"],
            &["💖Q0", "💖Q1", "💖Q2", "💖Q3"],
        ),
    ),
    (
        "👻Q3",
        (
            &["👻Q2", "👻Q4", "👻Q0", "👻P3", "👻P4", "👻P7", "⚽P1"],
            &["💖Q0", "💖Q1", "💖Q2", "💖Q3"],
        ),
    ),
    ("👻Q4", (&["👻Q1", "👻Q2", "👻Q3", "👻Q5", "👻P3"], &[])),
    (
        "👻Q5",
        (&["👻Q1", "👻Q4", "👻Q6", "👻Q7", "👻Q8", "👻P3"], &[]),
    ),
    ("👻Q6", (&["👻Q8", "👻Q1", "👻Q5"], &[])),
    ("👻Q7", (&["👻Q8", "👻Q5", "👻P3", "👻R8", "👻R7"], &[])),
    (
        "👻Q8",
        (
            &["👻Q5", "👻Q6", "👻Q7", "👻R7", "👻R4", "👻S8", "🇲🇺U3"],
            &[],
        ),
    ),
    (
        "👻R0",
        (&["👻R1", "👻R2", "👻R3"], &["🍔R0", "🍔R1", "🍔R2", "🍔R3"]),
    ),
    (
        "👻R1",
        (
            &["👻R8", "👻R2", "👻R6", "👻R0"],
            &["🍔R0", "🍔R1", "🍔R2", "🍔R3"],
        ),
    ),
    (
        "👻R2",
        (
            &["👻R1", "👻R3", "👻R5", "👻R6", "👻R0"],
            &["🍔R0", "🍔R1", "🍔R2", "🍔R3"],
        ),
    ),
    (
        "👻R3",
        (
            &["👻R2", "👻R4", "👻R5", "👻R0", "👻S1", "👻S6"],
            &["🍔R0", "🍔R1", "🍔R2", "🍔R3"],
        ),
    ),
    (
        "👻R4",
        (
            &["👻R3", "👻R5", "👻R7", "👻Q8", "👻S6", "👻S8", "🇲🇺U3"],
            &[],
        ),
    ),
    ("👻R5", (&["👻R2", "👻R3", "👻R4", "👻R6", "👻R7"], &[])),
    ("👻R6", (&["👻R1", "👻R2", "👻R5", "👻R7", "👻R8"], &[])),
    (
        "👻R7",
        (&["👻R8", "👻R4", "👻R5", "👻R6", "👻Q7", "👻Q8"], &[]),
    ),
    ("👻R8", (&["👻R1", "👻R6", "👻R7", "👻Q7", "👻P3"], &[])),
    (
        "👻S0",
        (&["👻S1", "👻S2", "👻S3"], &["🍔S0", "🍔S1", "🍔S2", "🍔S3"]),
    ),
    (
        "👻S1",
        (
            &["👻S2", "👻S5", "👻S6", "👻S0", "👻R3"],
            &["🍔S0", "🍔S1", "🍔S2", "🍔S3"],
        ),
    ),
    (
        "👻S2",
        (
            &["👻S1", "👻S3", "👻S4", "👻S5", "👻S0"],
            &["🍔S0", "🍔S1", "🍔S2", "🍔S3"],
        ),
    ),
    (
        "👻S3",
        (
            &["👻S2", "👻S4", "👻S0", "👻T1", "👻T6"],
            &["🍔S0", "🍔S1", "🍔S2", "🍔S3"],
        ),
    ),
    (
        "👻S4",
        (
            &[
                "👻S2",
                "👻S3",
                "👻S5",
                "👻S7",
                "👻T6",
                "👻T8",
                "🇲🇺U5",
                "🇲🇺U7",
                "👻X8",
            ],
            &[],
        ),
    ),
    ("👻S5", (&["👻S1", "👻S2", "👻S4", "👻S6", "👻S7"], &[])),
    (
        "👻S6",
        (&["👻S8", "👻S1", "👻S5", "👻S7", "👻R3", "👻R4"], &[]),
    ),
    (
        "👻S7",
        (&["👻S8", "👻S4", "👻S5", "👻S6", "🇲🇺U4", "🇲🇺U5"], &[]),
    ),
    (
        "👻S8",
        (&["👻S6", "👻S7", "👻R4", "👻Q8", "🇲🇺U3", "🇲🇺U4"], &[]),
    ),
    (
        "👻T0",
        (&["👻T1", "👻T2", "👻T3"], &["🍔T0", "🍔T1", "🍔T2", "🍔T3"]),
    ),
    (
        "👻T1",
        (
            &["👻T2", "👻T6", "👻T0", "👻S3"],
            &["🍔T0", "🍔T1", "🍔T2", "🍔T3"],
        ),
    ),
    (
        "👻T2",
        (
            &["👻T1", "👻T3", "👻T4", "👻T5", "👻T6", "👻T0"],
            &["🍔T0", "🍔T1", "🍔T2", "🍔T3"],
        ),
    ),
    (
        "👻T3",
        (
            &["👻T2", "👻T4", "👻T0", "👻U1", "👻U6"],
            &["🍔T0", "🍔T1", "🍔T2", "🍔T3"],
        ),
    ),
    (
        "👻T4",
        (&["👻T2", "👻T3", "👻T5", "👻T7", "👻U6", "👻U8"], &[]),
    ),
    ("👻T5", (&["👻T2", "👻T4", "👻T6", "👻T7", "👻T8"], &[])),
    (
        "👻T6",
        (&["👻T8", "👻T1", "👻T2", "👻T5", "👻S3", "👻S4"], &[]),
    ),
    (
        "👻T7",
        (&["👻T8", "👻T4", "👻T5", "👻U8", "👻X6", "👻X1"], &[]),
    ),
    (
        "👻T8",
        (
            &["👻T5", "👻T6", "👻T7", "👻S4", "🇲🇺U7", "👻X8", "👻X6"],
            &[],
        ),
    ),
    (
        "👻U0",
        (&["👻U1", "👻U2", "👻U3"], &["🍔U0", "🍔U1", "🍔U2", "🍔U3"]),
    ),
    (
        "👻U1",
        (
            &["👻U2", "👻U6", "👻U0", "👻T3"],
            &["🍔U0", "🍔U1", "🍔U2", "🍔U3"],
        ),
    ),
    (
        "👻U2",
        (
            &["👻U1", "👻U3", "👻U4", "👻U5", "👻U6", "👻U0"],
            &["🍔U0", "🍔U1", "🍔U2", "🍔U3"],
        ),
    ),
    (
        "👻U3",
        (&["👻U2", "👻U4", "👻U0"], &["🍔U0", "🍔U1", "🍔U2", "🍔U3"]),
    ),
    ("👻U4", (&["👻U2", "👻U3", "👻U5", "👻U7"], &[])),
    ("👻U5", (&["👻U2", "👻U4", "👻U6", "👻U7", "👻U8"], &[])),
    (
        "👻U6",
        (&["👻U8", "👻U1", "👻U2", "👻U5", "👻T3", "👻T4"], &[]),
    ),
    ("👻U7", (&["👻U8", "👻U4", "👻U5", "🇲🇺Q8"], &[])),
    (
        "👻U8",
        (&["👻U5", "👻U6", "👻U7", "👻T4", "👻T7", "👻X1"], &[]),
    ),
    (
        "🧩P0",
        (&["🧩P1", "🧩P2", "🧩P3"], &["🐦‍🔥P0", "🐦‍🔥P1", "🐦‍🔥P2", "🐦‍🔥P3"]),
    ),
    (
        "🧩P1",
        (
            &["🧩P2", "🧩P4", "🧩P5", "🧩P6", "🧩P0"],
            &["🐦‍🔥P0", "🐦‍🔥P1", "🐦‍🔥P2", "🐦‍🔥P3"],
        ),
    ),
    (
        "🧩P2",
        (
            &["🧩P1", "🧩P3", "🧩P4", "🧩P0"],
            &["🐦‍🔥P0", "🐦‍🔥P1", "🐦‍🔥P2", "🐦‍🔥P3"],
        ),
    ),
    (
        "🧩P3",
        (
            &["🧩P2", "🧩P4", "🧩P0", "🧩Q4", "🧩Q3", "🧩R8"],
            &["🐦‍🔥P0", "🐦‍🔥P1", "🐦‍🔥P2", "🐦‍🔥P3"],
        ),
    ),
    (
        "🧩P4",
        (&["🧩P1", "🧩P2", "🧩P3", "🧩P5", "🧩P7", "🧩Q3"], &[]),
    ),
    ("🧩P5", (&["🧩P1", "🧩P4", "🧩P6", "🧩P7"], &[])),
    ("🧩P6", (&["🧩P8", "🧩P1", "🧩P5", "🧩P7"], &[])),
    (
        "🧩P7",
        (
            &[
                "🧩P8", "🧩P4", "🧩P5", "🧩P6", "🧩Q3", "🌱P1", "🌱P6", "🌱P8",
            ],
            &[],
        ),
    ),
    ("🧩P8", (&["🧩P6", "🧩P7", "🌱P8", "🌵U3"], &[])),
    (
        "🧩Q0",
        (&["🧩Q1", "🧩Q2", "🧩Q3"], &["💖U0", "💖U1", "💖U2", "💖U3"]),
    ),
    (
        "🧩Q1",
        (
            &["🧩Q8", "🧩Q2", "🧩Q6", "🧩Q0"],
            &["💖U0", "💖U1", "💖U2", "💖U3"],
        ),
    ),
    (
        "🧩Q2",
        (
            &["🧩Q1", "🧩Q3", "🧩Q5", "🧩Q6", "🧩Q0"],
            &["💖U0", "💖U1", "💖U2", "💖U3"],
        ),
    ),
    (
        "🧩Q3",
        (
            &[
                "🧩Q2", "🧩Q4", "🧩Q5", "🧩Q0", "🧩P3", "🧩P4", "🧩P7", "🌱P1",
            ],
            &["💖U0", "💖U1", "💖U2", "💖U3"],
        ),
    ),
    ("🧩Q4", (&["🧩Q3", "🧩Q5", "🧩Q7", "🧩P3", "🧩R8"], &[])),
    ("🧩Q5", (&["🧩Q2", "🧩Q3", "🧩Q4", "🧩Q6", "🧩Q7"], &[])),
    ("🧩Q6", (&["🧩Q1", "🧩Q2", "🧩Q5", "🧩Q7", "🧩Q8"], &[])),
    (
        "🧩Q7",
        (&["🧩Q8", "🧩Q4", "🧩Q5", "🧩Q6", "🧩R8", "🧩R7"], &[]),
    ),
    (
        "🧩Q8",
        (
            &["🧩Q1", "🧩Q6", "🧩Q7", "🧩R7", "🧩R4", "🧩S8", "⚽U3"],
            &[],
        ),
    ),
    (
        "🧩R0",
        (
            &["🧩R1", "🧩R2", "🧩R3"],
            &[
                "🐦‍🔥R0",
                "🐦‍🔥R1",
                "🐦‍🔥R2",
                "🐦‍🔥R3",
                "👻W0",
                "👻W1",
                "👻W2",
                "👻W3",
            ],
        ),
    ),
    (
        "🧩R1",
        (
            &["🧩R8", "🧩R2", "🧩R6", "🧩R0"],
            &[
                "🐦‍🔥R0",
                "🐦‍🔥R1",
                "🐦‍🔥R2",
                "🐦‍🔥R3",
                "👻W0",
                "👻W1",
                "👻W2",
                "👻W3",
            ],
        ),
    ),
    (
        "🧩R2",
        (
            &["🧩R1", "🧩R3", "🧩R5", "🧩R6", "🧩R0"],
            &[
                "🐦‍🔥R0",
                "🐦‍🔥R1",
                "🐦‍🔥R2",
                "🐦‍🔥R3",
                "👻W0",
                "👻W1",
                "👻W2",
                "👻W3",
            ],
        ),
    ),
    (
        "🧩R3",
        (
            &["🧩R2", "🧩R4", "🧩R5", "🧩R0", "🧩S1", "🧩S6"],
            &[
                "🐦‍🔥R0",
                "🐦‍🔥R1",
                "🐦‍🔥R2",
                "🐦‍🔥R3",
                "👻W0",
                "👻W1",
                "👻W2",
                "👻W3",
            ],
        ),
    ),
    (
        "🧩R4",
        (
            &["🧩R3", "🧩R5", "🧩R7", "🧩Q8", "🧩S6", "🧩S8", "⚽U3"],
            &[],
        ),
    ),
    ("🧩R5", (&["🧩R2", "🧩R3", "🧩R4", "🧩R6", "🧩R7"], &[])),
    ("🧩R6", (&["🧩R1", "🧩R2", "🧩R5", "🧩R7", "🧩R8"], &[])),
    (
        "🧩R7",
        (&["🧩R8", "🧩R4", "🧩R5", "🧩R6", "🧩Q7", "🧩Q8"], &[]),
    ),
    (
        "🧩R8",
        (&["🧩R1", "🧩R6", "🧩R7", "🧩Q4", "🧩Q7", "🧩P3"], &[]),
    ),
    (
        "🧩S0",
        (
            &["🧩S1", "🧩S2", "🧩S3"],
            &[
                "🐦‍🔥S0",
                "🐦‍🔥S1",
                "🐦‍🔥S2",
                "🐦‍🔥S3",
                "👻W0",
                "👻W1",
                "👻W2",
                "👻W3",
            ],
        ),
    ),
    (
        "🧩S1",
        (
            &["🧩S2", "🧩S4", "🧩S5", "🧩S6", "🧩S0", "🧩R3"],
            &[
                "🐦‍🔥S0",
                "🐦‍🔥S1",
                "🐦‍🔥S2",
                "🐦‍🔥S3",
                "👻W0",
                "👻W1",
                "👻W2",
                "👻W3",
            ],
        ),
    ),
    (
        "🧩S2",
        (
            &["🧩S1", "🧩S3", "🧩S4", "🧩S0"],
            &[
                "🐦‍🔥S0",
                "🐦‍🔥S1",
                "🐦‍🔥S2",
                "🐦‍🔥S3",
                "👻W0",
                "👻W1",
                "👻W2",
                "👻W3",
            ],
        ),
    ),
    (
        "🧩S3",
        (
            &["🧩S2", "🧩S4", "🧩S0", "🧩T1", "🧩T7"],
            &[
                "🐦‍🔥S0",
                "🐦‍🔥S1",
                "🐦‍🔥S2",
                "🐦‍🔥S3",
                "👻W0",
                "👻W1",
                "👻W2",
                "👻W3",
            ],
        ),
    ),
    (
        "🧩S4",
        (&["🧩S1", "🧩S2", "🧩S3", "🧩S5", "🧩T7", "🧩T8"], &[]),
    ),
    (
        "🧩S5",
        (&["🧩S1", "🧩S4", "🧩S6", "🧩S7", "🧩S8", "🧩T8"], &[]),
    ),
    ("🧩S6", (&["🧩S8", "🧩S1", "🧩S5", "🧩R3", "🧩R4"], &[])),
    (
        "🧩S7",
        (&["🧩S8", "🧩S5", "🧩T8", "⚽U3", "⚽U4", "⚽U6"], &[]),
    ),
    (
        "🧩S8",
        (&["🧩S5", "🧩S6", "🧩S7", "🧩R4", "🧩Q8", "⚽U3"], &[]),
    ),
    (
        "🧩T0",
        (
            &["🧩T1", "🧩T2", "🧩T3"],
            &[
                "🐦‍🔥T0",
                "🐦‍🔥T1",
                "🐦‍🔥T2",
                "🐦‍🔥T3",
                "👻W0",
                "👻W1",
                "👻W2",
                "👻W3",
            ],
        ),
    ),
    (
        "🧩T1",
        (
            &["🧩T2", "🧩T5", "🧩T7", "🧩T0", "🧩S3"],
            &[
                "🐦‍🔥T0",
                "🐦‍🔥T1",
                "🐦‍🔥T2",
                "🐦‍🔥T3",
                "👻W0",
                "👻W1",
                "👻W2",
                "👻W3",
            ],
        ),
    ),
    (
        "🧩T2",
        (
            &["🧩T1", "🧩T3", "🧩T5", "🧩T0"],
            &[
                "🐦‍🔥T0",
                "🐦‍🔥T1",
                "🐦‍🔥T2",
                "🐦‍🔥T3",
                "👻W0",
                "👻W1",
                "👻W2",
                "👻W3",
            ],
        ),
    ),
    (
        "🧩T3",
        (
            &["🧩T2", "🧩T4", "🧩T5", "🧩T0", "🧩U1"],
            &[
                "🐦‍🔥T0",
                "🐦‍🔥T1",
                "🐦‍🔥T2",
                "🐦‍🔥T3",
                "👻W0",
                "👻W1",
                "👻W2",
                "👻W3",
            ],
        ),
    ),
    (
        "🧩T4",
        (&["🧩T3", "🧩T5", "🧩T6", "🧩T7", "🧩U1", "🧩U8"], &[]),
    ),
    ("🧩T5", (&["🧩T1", "🧩T2", "🧩T3", "🧩T4", "🧩T7"], &[])),
    ("🧩T6", (&["🧩T8", "🧩T4", "🧩T7", "🧩U8"], &[])),
    (
        "🧩T7",
        (
            &["🧩T1", "🧩T4", "🧩T5", "🧩T6", "🧩T8", "🧩S3", "🧩S4"],
            &[],
        ),
    ),
    (
        "🧩T8",
        (&["🧩T6", "🧩T7", "🧩S4", "🧩S5", "🧩S7", "⚽U6"], &[]),
    ),
    (
        "🧩U0",
        (
            &["🧩U1", "🧩U2", "🧩U3"],
            &[
                "🐦‍🔥U0",
                "🐦‍🔥U1",
                "🐦‍🔥U2",
                "🐦‍🔥U3",
                "👻W0",
                "👻W1",
                "👻W2",
                "👻W3",
            ],
        ),
    ),
    (
        "🧩U1",
        (
            &["🧩U8", "🧩U2", "🧩U5", "🧩U7", "🧩U0", "🧩T3", "🧩T4"],
            &[
                "🐦‍🔥U0",
                "🐦‍🔥U1",
                "🐦‍🔥U2",
                "🐦‍🔥U3",
                "👻W0",
                "👻W1",
                "👻W2",
                "👻W3",
            ],
        ),
    ),
    (
        "🧩U2",
        (
            &["🧩U1", "🧩U3", "🧩U4", "🧩U5", "🧩U0"],
            &[
                "🐦‍🔥U0",
                "🐦‍🔥U1",
                "🐦‍🔥U2",
                "🐦‍🔥U3",
                "👻W0",
                "👻W1",
                "👻W2",
                "👻W3",
            ],
        ),
    ),
    (
        "🧩U3",
        (
            &["🧩U2", "🧩U4", "🧩U0", "⚽P8", "👻P8"],
            &[
                "🐦‍🔥U0",
                "🐦‍🔥U1",
                "🐦‍🔥U2",
                "🐦‍🔥U3",
                "👻W0",
                "👻W1",
                "👻W2",
                "👻W3",
            ],
        ),
    ),
    (
        "🧩U4",
        (&["🧩U2", "🧩U3", "🧩U5", "🧩U6", "⚽P7", "⚽P8"], &[]),
    ),
    ("🧩U5", (&["🧩U1", "🧩U2", "🧩U4", "🧩U6", "🧩U7"], &[])),
    (
        "🧩U6",
        (&["🧩U4", "🧩U5", "🧩U7", "⚽P4", "⚽P7", "⚽Q8"], &[]),
    ),
    ("🧩U7", (&["🧩U8", "🧩U1", "🧩U5", "🧩U6"], &[])),
    ("🧩U8", (&["🧩U1", "🧩U7", "🧩T4", "🧩T6"], &[])),
    (
        "🌵P0",
        (&["🌵P1", "🌵P2", "🌵P3"], &["📚P0", "📚P1", "📚P2", "📚P3"]),
    ),
    (
        "🌵P1",
        (
            &["🌵P2", "🌵P5", "🌵P7", "🌵P0"],
            &["📚P0", "📚P1", "📚P2", "📚P3"],
        ),
    ),
    (
        "🌵P2",
        (
            &["🌵P1", "🌵P3", "🌵P5", "🌵P0"],
            &["📚P0", "📚P1", "📚P2", "📚P3"],
        ),
    ),
    (
        "🌵P3",
        (
            &["🌵P2", "🌵P4", "🌵P5", "🌵P0", "🌵Q4", "🌵R8"],
            &["📚P0", "📚P1", "📚P2", "📚P3"],
        ),
    ),
    (
        "🌵P4",
        (&["🌵P3", "🌵P5", "🌵P6", "🌵P7", "🌵Q4", "🌵Q3"], &[]),
    ),
    ("🌵P5", (&["🌵P1", "🌵P2", "🌵P3", "🌵P4", "🌵P7"], &[])),
    ("🌵P6", (&["🌵P8", "🌵P4", "🌵P7", "🌵Q3", "🐀P1"], &[])),
    ("🌵P7", (&["🌵P1", "🌵P4", "🌵P5", "🌵P6", "🌵P8"], &[])),
    ("🌵P8", (&["🌵P6", "🌵P7", "🐀P1", "🐀P8", "🧀U3"], &[])),
    (
        "🌵Q0",
        (&["🌵Q1", "🌵Q2", "🌵Q3"], &["😎Q0", "😎Q1", "😎Q2", "😎Q3"]),
    ),
    (
        "🌵Q1",
        (
            &["🌵Q8", "🌵Q2", "🌵Q6", "🌵Q0"],
            &["😎Q0", "😎Q1", "😎Q2", "😎Q3"],
        ),
    ),
    (
        "🌵Q2",
        (
            &["🌵Q1", "🌵Q3", "🌵Q5", "🌵Q6", "🌵Q0"],
            &["😎Q0", "😎Q1", "😎Q2", "😎Q3"],
        ),
    ),
    (
        "🌵Q3",
        (
            &["🌵Q2", "🌵Q4", "🌵Q5", "🌵Q0", "🌵P4", "🌵P6", "🐀P1"],
            &["😎Q0", "😎Q1", "😎Q2", "😎Q3"],
        ),
    ),
    (
        "🌵Q4",
        (&["🌵Q3", "🌵Q5", "🌵Q7", "🌵P3", "🌵P4", "🌵R8"], &[]),
    ),
    ("🌵Q5", (&["🌵Q2", "🌵Q3", "🌵Q4", "🌵Q6", "🌵Q7"], &[])),
    ("🌵Q6", (&["🌵Q1", "🌵Q2", "🌵Q5", "🌵Q7", "🌵Q8"], &[])),
    (
        "🌵Q7",
        (&["🌵Q8", "🌵Q4", "🌵Q5", "🌵Q6", "🌵R8", "🌵R7"], &[]),
    ),
    (
        "🌵Q8",
        (&["🌵Q1", "🌵Q6", "🌵Q7", "🌵R7", "🌵S8", "🌱U3"], &[]),
    ),
    (
        "🌵R0",
        (&["🌵R1", "🌵R2", "🌵R3"], &["📚R0", "📚R1", "📚R2", "📚R3"]),
    ),
    (
        "🌵R1",
        (&["🌵R2", "🌵R6", "🌵R0"], &["📚R0", "📚R1", "📚R2", "📚R3"]),
    ),
    (
        "🌵R2",
        (
            &["🌵R1", "🌵R3", "🌵R4", "🌵R5", "🌵R6", "🌵R0"],
            &["📚R0", "📚R1", "📚R2", "📚R3"],
        ),
    ),
    (
        "🌵R3",
        (
            &["🌵R2", "🌵R4", "🌵R0", "🌵S1", "🌵S8"],
            &["📚R0", "📚R1", "📚R2", "📚R3"],
        ),
    ),
    ("🌵R4", (&["🌵R2", "🌵R3", "🌵R5", "🌵R7", "🌵S8"], &[])),
    ("🌵R5", (&["🌵R2", "🌵R4", "🌵R6", "🌵R7", "🌵R8"], &[])),
    ("🌵R6", (&["🌵R8", "🌵R1", "🌵R2", "🌵R5"], &[])),
    (
        "🌵R7",
        (
            &["🌵R8", "🌵R4", "🌵R5", "🌵Q7", "🌵Q8", "🌵S8", "🌱U3"],
            &[],
        ),
    ),
    (
        "🌵R8",
        (&["🌵R5", "🌵R6", "🌵R7", "🌵Q4", "🌵Q7", "🌵P3"], &[]),
    ),
    (
        "🌵S0",
        (&["🌵S1", "🌵S2", "🌵S3"], &["📚S0", "📚S1", "📚S2", "📚S3"]),
    ),
    (
        "🌵S1",
        (
            &["🌵S8", "🌵S2", "🌵S6", "🌵S0", "🌵R3"],
            &["📚S0", "📚S1", "📚S2", "📚S3"],
        ),
    ),
    (
        "🌵S2",
        (
            &["🌵S1", "🌵S3", "🌵S5", "🌵S6", "🌵S0"],
            &["📚S0", "📚S1", "📚S2", "📚S3"],
        ),
    ),
    (
        "🌵S3",
        (
            &["🌵S2", "🌵S4", "🌵S5", "🌵S0", "🌵T1"],
            &["📚S0", "📚S1", "📚S2", "📚S3"],
        ),
    ),
    (
        "🌵S4",
        (
            &["🌵S3", "🌵S5", "🌵S7", "🌵T1", "🌵T8", "🌱U4", "🌱U7"],
            &[],
        ),
    ),
    ("🌵S5", (&["🌵S2", "🌵S3", "🌵S4", "🌵S6", "🌵S7"], &[])),
    ("🌵S6", (&["🌵S1", "🌵S2", "🌵S5", "🌵S7", "🌵S8"], &[])),
    (
        "🌵S7",
        (&["🌵S8", "🌵S4", "🌵S5", "🌵S6", "🌱U3", "🌱U4"], &[]),
    ),
    (
        "🌵S8",
        (
            &[
                "🌵S1", "🌵S6", "🌵S7", "🌵R3", "🌵R4", "🌵R7", "🌵Q8", "🌱U3",
            ],
            &[],
        ),
    ),
    (
        "🌵T0",
        (&["🌵T1", "🌵T2", "🌵T3"], &["📚T0", "📚T1", "📚T2", "📚T3"]),
    ),
    (
        "🌵T1",
        (
            &["🌵T8", "🌵T2", "🌵T5", "🌵T7", "🌵T0", "🌵S3", "🌵S4"],
            &["📚T0", "📚T1", "📚T2", "📚T3"],
        ),
    ),
    (
        "🌵T2",
        (
            &["🌵T1", "🌵T3", "🌵T4", "🌵T5", "🌵T0"],
            &["📚T0", "📚T1", "📚T2", "📚T3"],
        ),
    ),
    (
        "🌵T3",
        (
            &["🌵T2", "🌵T4", "🌵T0", "🌵U1"],
            &["📚T0", "📚T1", "📚T2", "📚T3"],
        ),
    ),
    (
        "🌵T4",
        (&["🌵T2", "🌵T3", "🌵T5", "🌵T6", "🌵U1", "🌵U6"], &[]),
    ),
    ("🌵T5", (&["🌵T1", "🌵T2", "🌵T4", "🌵T6", "🌵T7"], &[])),
    ("🌵T6", (&["🌵T4", "🌵T5", "🌵T7", "🌵U6", "🌵U8"], &[])),
    ("🌵T7", (&["🌵T8", "🌵T1", "🌵T5", "🌵T6"], &[])),
    ("🌵T8", (&["🌵T1", "🌵T7", "🌵S4", "🌱U7"], &[])),
    (
        "🌵U0",
        (&["🌵U1", "🌵U2", "🌵U3"], &["📚U0", "📚U1", "📚U2", "📚U3"]),
    ),
    (
        "🌵U1",
        (
            &["🌵U2", "🌵U5", "🌵U6", "🌵U0", "🌵T3", "🌵T4"],
            &["📚U0", "📚U1", "📚U2", "📚U3"],
        ),
    ),
    (
        "🌵U2",
        (
            &["🌵U1", "🌵U3", "🌵U4", "🌵U5", "🌵U0"],
            &["📚U0", "📚U1", "📚U2", "📚U3"],
        ),
    ),
    (
        "🌵U3",
        (
            &["🌵U2", "🌵U4", "🌵U0", "🌱P8", "🧩P8"],
            &["📚U0", "📚U1", "📚U2", "📚U3"],
        ),
    ),
    (
        "🌵U4",
        (
            &["🌵U2", "🌵U3", "🌵U5", "🌵U7", "🌵U8", "🌱P7", "🌱P8"],
            &[],
        ),
    ),
    ("🌵U5", (&["🌵U1", "🌵U2", "🌵U4", "🌵U6", "🌵U8"], &[])),
    ("🌵U6", (&["🌵U8", "🌵U1", "🌵U5", "🌵T4", "🌵T6"], &[])),
    ("🌵U7", (&["🌵U8", "🌵U4", "🌱P7", "🌱Q8"], &[])),
    ("🌵U8", (&["🌵U4", "🌵U5", "🌵U6", "🌵U7", "🌵T6"], &[])),
    (
        "🐍P0",
        (&["🐍P1", "🐍P2", "🐍P3"], &["😎U0", "😎U1", "😎U2", "😎U3"]),
    ),
    (
        "🐍P1",
        (
            &[
                "🐍P8", "🐍P2", "🐍P5", "🐍P7", "🐍P0", "🧀P4", "🧀P7", "🧀P8", "🧀Q3",
            ],
            &["😎U0", "😎U1", "😎U2", "😎U3"],
        ),
    ),
    (
        "🐍P2",
        (
            &["🐍P1", "🐍P3", "🐍P4", "🐍P5", "🐍P0"],
            &["😎U0", "😎U1", "😎U2", "😎U3"],
        ),
    ),
    (
        "🐍P3",
        (
            &["🐍P2", "🐍P4", "🐍P0", "🐍Q1"],
            &["😎U0", "😎U1", "😎U2", "😎U3"],
        ),
    ),
    (
        "🐍P4",
        (
            &["🐍P2", "🐍P3", "🐍P5", "🐍P6", "🐍Q1", "🐍Q7", "🐍Q8"],
            &[],
        ),
    ),
    ("🐍P5", (&["🐍P1", "🐍P2", "🐍P4", "🐍P6", "🐍P7"], &[])),
    ("🐍P6", (&["🐍P4", "🐍P5", "🐍P7", "🐍Q8", "🌙U4"], &[])),
    (
        "🐍P7",
        (&["🐍P8", "🐍P1", "🐍P5", "🐍P6", "🌙U4", "🌙U3"], &[]),
    ),
    ("🐍P8", (&["🐍P1", "🐍P7", "🧀P8", "🌙U3"], &[])),
    (
        "🐍Q0",
        (&["🐍Q1", "🐍Q2", "🐍Q3"], &["😎U0", "😎U1", "😎U2", "😎U3"]),
    ),
    (
        "🐍Q1",
        (
            &["🐍Q2", "🐍Q5", "🐍Q7", "🐍Q0", "🐍P3", "🐍P4"],
            &["😎U0", "😎U1", "😎U2", "😎U3"],
        ),
    ),
    (
        "🐍Q2",
        (
            &["🐍Q1", "🐍Q3", "🐍Q5", "🐍Q0"],
            &["😎U0", "😎U1", "😎U2", "😎U3"],
        ),
    ),
    (
        "🐍Q3",
        (
            &["🐍Q2", "🐍Q4", "🐍Q5", "🐍Q0", "🐍R1", "🐍R6"],
            &["😎U0", "😎U1", "😎U2", "😎U3"],
        ),
    ),
    (
        "🐍Q4",
        (&["🐍Q3", "🐍Q5", "🐍Q6", "🐍Q7", "🐍R6", "🐍R8"], &[]),
    ),
    ("🐍Q5", (&["🐍Q1", "🐍Q2", "🐍Q3", "🐍Q4", "🐍Q7"], &[])),
    ("🐍Q6", (&["🐍Q8", "🐍Q4", "🐍Q7", "🐍R8"], &[])),
    (
        "🐍Q7",
        (&["🐍Q1", "🐍Q4", "🐍Q5", "🐍Q6", "🐍Q8", "🐍P4"], &[]),
    ),
    ("🐍Q8", (&["🐍Q6", "🐍Q7", "🐍P4", "🐍P6", "🌙U4"], &[])),
    (
        "🐍R0",
        (&["🐍R1", "🐍R2", "🐍R3"], &["😎U0", "😎U1", "😎U2", "😎U3"]),
    ),
    (
        "🐍R1",
        (
            &["🐍R2", "🐍R5", "🐍R6", "🐍R0", "🐍Q3"],
            &["😎U0", "😎U1", "😎U2", "😎U3"],
        ),
    ),
    (
        "🐍R2",
        (
            &["🐍R1", "🐍R3", "🐍R4", "🐍R5", "🐍R0"],
            &["😎U0", "😎U1", "😎U2", "😎U3"],
        ),
    ),
    (
        "🐍R3",
        (
            &["🐍R2", "🐍R4", "🐍R0", "🐍S8", "🐍S7", "🏝️P1"],
            &["😎U0", "😎U1", "😎U2", "😎U3"],
        ),
    ),
    (
        "🐍R4",
        (&["🐍R2", "🐍R3", "🐍R5", "🐍R7", "🐍S7", "🐍T8"], &[]),
    ),
    ("🐍R5", (&["🐍R1", "🐍R2", "🐍R4", "🐍R6", "🐍R7"], &[])),
    (
        "🐍R6",
        (&["🐍R8", "🐍R1", "🐍R5", "🐍R7", "🐍Q3", "🐍Q4"], &[]),
    ),
    ("🐍R7", (&["🐍R8", "🐍R4", "🐍R5", "🐍R6"], &[])),
    ("🐍R8", (&["🐍R6", "🐍R7", "🐍Q4", "🐍Q6"], &[])),
    (
        "🐍S0",
        (&["🐍S1", "🐍S2", "🐍S3"], &["🏝️T0", "🏝️T1", "🏝️T2", "🏝️T3"]),
    ),
    (
        "🐍S1",
        (
            &[
                "🐍S2",
                "🐍S4",
                "🐍S5",
                "🐍S6",
                "🐍S0",
                "🏝️P6",
                "🏝️P8",
                "🏝️Q3",
            ],
            &["🏝️T0", "🏝️T1", "🏝️T2", "🏝️T3"],
        ),
    ),
    (
        "🐍S2",
        (
            &["🐍S1", "🐍S3", "🐍S4", "🐍S0"],
            &["🏝️T0", "🏝️T1", "🏝️T2", "🏝️T3"],
        ),
    ),
    (
        "🐍S3",
        (
            &["🐍S2", "🐍S4", "🐍S0", "🐍T1", "🐍T6"],
            &["🏝️T0", "🏝️T1", "🏝️T2", "🏝️T3"],
        ),
    ),
    (
        "🐍S4",
        (&["🐍S1", "🐍S2", "🐍S3", "🐍S5", "🐍T6", "🐍T8"], &[]),
    ),
    (
        "🐍S5",
        (&["🐍S1", "🐍S4", "🐍S6", "🐍S7", "🐍S8", "🐍T8"], &[]),
    ),
    ("🐍S6", (&["🐍S8", "🐍S1", "🐍S5", "🏝️P6"], &[])),
    ("🐍S7", (&["🐍S8", "🐍S5", "🐍T8", "🐍R3", "🐍R4"], &[])),
    (
        "🐍S8",
        (&["🐍S5", "🐍S6", "🐍S7", "🐍R3", "🏝️P1", "🏝️P6"], &[]),
    ),
    (
        "🐍T0",
        (&["🐍T1", "🐍T2", "🐍T3"], &["🏝️T0", "🏝️T1", "🏝️T2", "🏝️T3"]),
    ),
    (
        "🐍T1",
        (
            &["🐍T2", "🐍T4", "🐍T5", "🐍T6", "🐍T0", "🐍S3"],
            &["🏝️T0", "🏝️T1", "🏝️T2", "🏝️T3"],
        ),
    ),
    (
        "🐍T2",
        (
            &["🐍T1", "🐍T3", "🐍T4", "🐍T0"],
            &["🏝️T0", "🏝️T1", "🏝️T2", "🏝️T3"],
        ),
    ),
    (
        "🐍T3",
        (
            &["🐍T2", "🐍T4", "🐍T0", "🐍U1", "🐍U6", "🐍U8"],
            &["🏝️T0", "🏝️T1", "🏝️T2", "🏝️T3"],
        ),
    ),
    (
        "🐍T4",
        (&["🐍T1", "🐍T2", "🐍T3", "🐍T5", "🐍T7", "🐍U8"], &[]),
    ),
    ("🐍T5", (&["🐍T1", "🐍T4", "🐍T6", "🐍T7"], &[])),
    (
        "🐍T6",
        (&["🐍T8", "🐍T1", "🐍T5", "🐍T7", "🐍S3", "🐍S4"], &[]),
    ),
    ("🐍T7", (&["🐍T8", "🐍T4", "🐍T5", "🐍T6", "🐍U8"], &[])),
    (
        "🐍T8",
        (&["🐍T6", "🐍T7", "🐍S4", "🐍S5", "🐍S7", "🐍R4"], &[]),
    ),
    (
        "🐍U0",
        (&["🐍U1", "🐍U2", "🐍U3"], &["🏝️T0", "🏝️T1", "🏝️T2", "🏝️T3"]),
    ),
    (
        "🐍U1",
        (
            &["🐍U2", "🐍U4", "🐍U5", "🐍U6", "🐍U0", "🐍T3"],
            &["🏝️T0", "🏝️T1", "🏝️T2", "🏝️T3"],
        ),
    ),
    (
        "🐍U2",
        (
            &["🐍U1", "🐍U3", "🐍U4", "🐍U0"],
            &["🏝️T0", "🏝️T1", "🏝️T2", "🏝️T3"],
        ),
    ),
    (
        "🐍U3",
        (
            &["🐍U2", "🐍U4", "🐍U0", "🌙S8", "🌙S7", "🌙R4", "🌙Q8"],
            &["🏝️T0", "🏝️T1", "🏝️T2", "🏝️T3"],
        ),
    ),
    (
        "🐍U4",
        (
            &["🐍U1", "🐍U2", "🐍U3", "🐍U5", "🐍U7", "🌙S7", "🌙S4"],
            &[],
        ),
    ),
    ("🐍U5", (&["🐍U1", "🐍U4", "🐍U6", "🐍U7"], &[])),
    ("🐍U6", (&["🐍U8", "🐍U1", "🐍U5", "🐍U7", "🐍T3"], &[])),
    (
        "🐍U7",
        (&["🐍U8", "🐍U4", "🐍U5", "🐍U6", "🌙S4", "🌙T8"], &[]),
    ),
    ("🐍U8", (&["🐍U6", "🐍U7", "🐍T3", "🐍T4", "🐍T7"], &[])),
    (
        "♾️P0",
        (&["♾️P1", "♾️P2", "♾️P3"], &["🏝️Q0", "🏝️Q1", "🏝️Q2", "🏝️Q3"]),
    ),
    (
        "♾️P1",
        (
            &[
                "♾️P8", "♾️P2", "♾️P6", "♾️P0", "🌙P4", "🌙P7", "🌙P8", "🌙Q3",
            ],
            &["🏝️Q0", "🏝️Q1", "🏝️Q2", "🏝️Q3"],
        ),
    ),
    (
        "♾️P2",
        (
            &["♾️P1", "♾️P3", "♾️P5", "♾️P6", "♾️P0"],
            &["🏝️Q0", "🏝️Q1", "🏝️Q2", "🏝️Q3"],
        ),
    ),
    (
        "♾️P3",
        (
            &["♾️P2", "♾️P4", "♾️P5", "♾️P0", "♾️Q1"],
            &["🏝️Q0", "🏝️Q1", "🏝️Q2", "🏝️Q3"],
        ),
    ),
    ("♾️P4", (&["♾️P3", "♾️P5", "♾️P7", "♾️Q1", "♾️Q8"], &[])),
    ("♾️P5", (&["♾️P2", "♾️P3", "♾️P4", "♾️P6", "♾️P7"], &[])),
    ("♾️P6", (&["♾️P1", "♾️P2", "♾️P5", "♾️P7", "♾️P8"], &[])),
    ("♾️P7", (&["♾️P8", "♾️P4", "♾️P5", "♾️P6"], &[])),
    ("♾️P8", (&["♾️P1", "♾️P6", "♾️P7", "🌙P8"], &[])),
    (
        "♾️Q0",
        (&["♾️Q1", "♾️Q2", "♾️Q3"], &["🏝️Q0", "🏝️Q1", "🏝️Q2", "🏝️Q3"]),
    ),
    (
        "♾️Q1",
        (
            &["♾️Q8", "♾️Q2", "♾️Q6", "♾️Q0", "♾️P3", "♾️P4"],
            &["🏝️Q0", "🏝️Q1", "🏝️Q2", "🏝️Q3"],
        ),
    ),
    (
        "♾️Q2",
        (
            &["♾️Q1", "♾️Q3", "♾️Q5", "♾️Q6", "♾️Q0"],
            &["🏝️Q0", "🏝️Q1", "🏝️Q2", "🏝️Q3"],
        ),
    ),
    (
        "♾️Q3",
        (
            &["♾️Q2", "♾️Q4", "♾️Q5", "♾️Q0", "♾️R1", "♾️R6"],
            &["🏝️Q0", "🏝️Q1", "🏝️Q2", "🏝️Q3"],
        ),
    ),
    ("♾️Q4", (&["♾️Q3", "♾️Q5", "♾️Q7", "♾️R6", "♾️R8"], &[])),
    ("♾️Q5", (&["♾️Q2", "♾️Q3", "♾️Q4", "♾️Q6", "♾️Q7"], &[])),
    ("♾️Q6", (&["♾️Q1", "♾️Q2", "♾️Q5", "♾️Q7", "♾️Q8"], &[])),
    ("♾️Q7", (&["♾️Q8", "♾️Q4", "♾️Q5", "♾️Q6"], &[])),
    ("♾️Q8", (&["♾️Q1", "♾️Q6", "♾️Q7", "♾️P4"], &[])),
    (
        "♾️R0",
        (&["♾️R1", "♾️R2", "♾️R3"], &["🏝️Q0", "🏝️Q1", "🏝️Q2", "🏝️Q3"]),
    ),
    (
        "♾️R1",
        (
            &["♾️R2", "♾️R4", "♾️R5", "♾️R6", "♾️R0", "♾️Q3"],
            &["🏝️Q0", "🏝️Q1", "🏝️Q2", "🏝️Q3"],
        ),
    ),
    (
        "♾️R2",
        (
            &["♾️R1", "♾️R3", "♾️R4", "♾️R0"],
            &["🏝️Q0", "🏝️Q1", "🏝️Q2", "🏝️Q3"],
        ),
    ),
    (
        "♾️R3",
        (
            &["♾️R2", "♾️R4", "♾️R0", "♾️S8", "♾️S7", "🏝️T1"],
            &["🏝️Q0", "🏝️Q1", "🏝️Q2", "🏝️Q3"],
        ),
    ),
    (
        "♾️R4",
        (&["♾️R1", "♾️R2", "♾️R3", "♾️R5", "♾️R7", "♾️S7"], &[]),
    ),
    ("♾️R5", (&["♾️R1", "♾️R4", "♾️R6", "♾️R7"], &[])),
    (
        "♾️R6",
        (&["♾️R8", "♾️R1", "♾️R5", "♾️R7", "♾️Q3", "♾️Q4"], &[]),
    ),
    (
        "♾️R7",
        (&["♾️R8", "♾️R4", "♾️R5", "♾️R6", "♾️S7", "♾️T8"], &[]),
    ),
    ("♾️R8", (&["♾️R6", "♾️R7", "♾️Q4"], &[])),
    (
        "♾️S0",
        (&["♾️S1", "♾️S2", "♾️S3"], &["💖P0", "💖P1", "💖P2", "💖P3"]),
    ),
    (
        "♾️S1",
        (
            &["♾️S2", "♾️S6", "♾️S0", "🏝️T7", "🏝️T8", "🏝️U3"],
            &["💖P0", "💖P1", "💖P2", "💖P3"],
        ),
    ),
    (
        "♾️S2",
        (
            &["♾️S1", "♾️S3", "♾️S4", "♾️S5", "♾️S6", "♾️S0"],
            &["💖P0", "💖P1", "💖P2", "💖P3"],
        ),
    ),
    (
        "♾️S3",
        (
            &["♾️S2", "♾️S4", "♾️S0", "♾️T1", "♾️T6"],
            &["💖P0", "💖P1", "💖P2", "💖P3"],
        ),
    ),
    (
        "♾️S4",
        (&["♾️S2", "♾️S3", "♾️S5", "♾️S7", "♾️T6", "♾️T8"], &[]),
    ),
    ("♾️S5", (&["♾️S2", "♾️S4", "♾️S6", "♾️S7", "♾️S8"], &[])),
    (
        "♾️S6",
        (&["♾️S8", "♾️S1", "♾️S2", "♾️S5", "🏝️T1", "🏝️T7"], &[]),
    ),
    (
        "♾️S7",
        (
            &["♾️S8", "♾️S4", "♾️S5", "♾️T8", "♾️R3", "♾️R4", "♾️R7"],
            &[],
        ),
    ),
    ("♾️S8", (&["♾️S5", "♾️S6", "♾️S7", "♾️R3", "🏝️T1"], &[])),
    (
        "♾️T0",
        (&["♾️T1", "♾️T2", "♾️T3"], &["💖P0", "💖P1", "💖P2", "💖P3"]),
    ),
    (
        "♾️T1",
        (
            &["♾️T2", "♾️T5", "♾️T6", "♾️T0", "♾️S3"],
            &["💖P0", "💖P1", "💖P2", "💖P3"],
        ),
    ),
    (
        "♾️T2",
        (
            &["♾️T1", "♾️T3", "♾️T4", "♾️T5", "♾️T0"],
            &["💖P0", "💖P1", "💖P2", "💖P3"],
        ),
    ),
    (
        "♾️T3",
        (&["♾️T2", "♾️T4", "♾️T0"], &["💖P0", "💖P1", "💖P2", "💖P3"]),
    ),
    ("♾️T4", (&["♾️T2", "♾️T3", "♾️T5", "♾️T7", "♾️T8"], &[])),
    ("♾️T5", (&["♾️T1", "♾️T2", "♾️T4", "♾️T6", "♾️T8"], &[])),
    ("♾️T6", (&["♾️T8", "♾️T1", "♾️T5", "♾️S3", "♾️S4"], &[])),
    ("♾️T7", (&["♾️T8", "♾️T4"], &[])),
    (
        "♾️T8",
        (
            &["♾️T4", "♾️T5", "♾️T6", "♾️T7", "♾️S4", "♾️S7", "♾️R7"],
            &[],
        ),
    ),
    (
        "🇲🇺Q0",
        (&["🇲🇺Q1", "🇲🇺Q2", "🇲🇺Q3"], &["🏝️U0", "🏝️U1", "🏝️U2", "🏝️U3"]),
    ),
    (
        "🇲🇺Q1",
        (
            &["🇲🇺Q2", "🇲🇺Q5", "🇲🇺Q6", "🇲🇺Q0"],
            &["🏝️U0", "🏝️U1", "🏝️U2", "🏝️U3"],
        ),
    ),
    (
        "🇲🇺Q2",
        (
            &["🇲🇺Q1", "🇲🇺Q3", "🇲🇺Q4", "🇲🇺Q5", "🇲🇺Q0"],
            &["🏝️U0", "🏝️U1", "🏝️U2", "🏝️U3"],
        ),
    ),
    (
        "🇲🇺Q3",
        (
            &["🇲🇺Q2", "🇲🇺Q4", "🇲🇺Q0", "🇲🇺R1", "🇲🇺R6"],
            &["🏝️U0", "🏝️U1", "🏝️U2", "🏝️U3"],
        ),
    ),
    (
        "🇲🇺Q4",
        (
            &["🇲🇺Q2", "🇲🇺Q3", "🇲🇺Q5", "🇲🇺Q7", "🇲🇺R6", "🇲🇺R8", "👻X3"],
            &[],
        ),
    ),
    ("🇲🇺Q5", (&["🇲🇺Q1", "🇲🇺Q2", "🇲🇺Q4", "🇲🇺Q6", "🇲🇺Q7"], &[])),
    ("🇲🇺Q6", (&["🇲🇺Q8", "🇲🇺Q1", "🇲🇺Q5", "🇲🇺Q7"], &[])),
    ("🇲🇺Q7", (&["🇲🇺Q8", "🇲🇺Q4", "🇲🇺Q5", "🇲🇺Q6"], &[])),
    ("🇲🇺Q8", (&["🇲🇺Q6", "🇲🇺Q7", "👻U7"], &[])),
    (
        "🇲🇺R0",
        (&["🇲🇺R1", "🇲🇺R2", "🇲🇺R3"], &["🏝️U0", "🏝️U1", "🏝️U2", "🏝️U3"]),
    ),
    (
        "🇲🇺R1",
        (
            &["🇲🇺R2", "🇲🇺R4", "🇲🇺R5", "🇲🇺R6", "🇲🇺R0", "🇲🇺Q3"],
            &["🏝️U0", "🏝️U1", "🏝️U2", "🏝️U3"],
        ),
    ),
    (
        "🇲🇺R2",
        (
            &["🇲🇺R1", "🇲🇺R3", "🇲🇺R4", "🇲🇺R0"],
            &["🏝️U0", "🏝️U1", "🏝️U2", "🏝️U3"],
        ),
    ),
    (
        "🇲🇺R3",
        (
            &["🇲🇺R2", "🇲🇺R4", "🇲🇺R0", "🇲🇺S8", "💖P1"],
            &["🏝️U0", "🏝️U1", "🏝️U2", "🏝️U3"],
        ),
    ),
    (
        "🇲🇺R4",
        (&["🇲🇺R1", "🇲🇺R2", "🇲🇺R3", "🇲🇺R5", "🇲🇺S8", "🇲🇺S7"], &[]),
    ),
    (
        "🇲🇺R5",
        (
            &["🇲🇺R1", "🇲🇺R4", "🇲🇺R6", "🇲🇺R7", "🇲🇺R8", "🇲🇺S7", "🇲🇺S4"],
            &[],
        ),
    ),
    ("🇲🇺R6", (&["🇲🇺R8", "🇲🇺R1", "🇲🇺R5", "🇲🇺Q3", "🇲🇺Q4"], &[])),
    ("🇲🇺R7", (&["🇲🇺R8", "🇲🇺R5", "🇲🇺S4", "🇲🇺T8"], &[])),
    ("🇲🇺R8", (&["🇲🇺R5", "🇲🇺R6", "🇲🇺R7", "🇲🇺Q4", "👻X3"], &[])),
    (
        "🇲🇺S0",
        (&["🇲🇺S1", "🇲🇺S2", "🇲🇺S3"], &["💖T0", "💖T1", "💖T2", "💖T3"]),
    ),
    (
        "🇲🇺S1",
        (
            &["🇲🇺S2", "🇲🇺S5", "🇲🇺S6", "🇲🇺S0", "💖P8", "💖Q3"],
            &["💖T0", "💖T1", "💖T2", "💖T3"],
        ),
    ),
    (
        "🇲🇺S2",
        (
            &["🇲🇺S1", "🇲🇺S3", "🇲🇺S4", "🇲🇺S5", "🇲🇺S0"],
            &["💖T0", "💖T1", "💖T2", "💖T3"],
        ),
    ),
    (
        "🇲🇺S3",
        (
            &["🇲🇺S2", "🇲🇺S4", "🇲🇺S0", "🇲🇺T1", "🇲🇺T6"],
            &["💖T0", "💖T1", "💖T2", "💖T3"],
        ),
    ),
    (
        "🇲🇺S4",
        (
            &[
                "🇲🇺S2",
                "🇲🇺S3",
                "🇲🇺S5",
                "🇲🇺S7",
                "🇲🇺T6",
                "🇲🇺T8",
                "🇲🇺R5",
                "🇲🇺R7",
            ],
            &[],
        ),
    ),
    ("🇲🇺S5", (&["🇲🇺S1", "🇲🇺S2", "🇲🇺S4", "🇲🇺S6", "🇲🇺S7"], &[])),
    (
        "🇲🇺S6",
        (&["🇲🇺S8", "🇲🇺S1", "🇲🇺S5", "🇲🇺S7", "💖P6", "💖P8"], &[]),
    ),
    (
        "🇲🇺S7",
        (&["🇲🇺S8", "🇲🇺S4", "🇲🇺S5", "🇲🇺S6", "🇲🇺R4", "🇲🇺R5"], &[]),
    ),
    (
        "🇲🇺S8",
        (&["🇲🇺S6", "🇲🇺S7", "🇲🇺R3", "🇲🇺R4", "💖P1", "💖P6"], &[]),
    ),
    (
        "🇲🇺T0",
        (&["🇲🇺T1", "🇲🇺T2", "🇲🇺T3"], &["💖T0", "💖T1", "💖T2", "💖T3"]),
    ),
    (
        "🇲🇺T1",
        (
            &["🇲🇺T2", "🇲🇺T5", "🇲🇺T6", "🇲🇺T0", "🇲🇺S3"],
            &["💖T0", "💖T1", "💖T2", "💖T3"],
        ),
    ),
    (
        "🇲🇺T2",
        (
            &["🇲🇺T1", "🇲🇺T3", "🇲🇺T4", "🇲🇺T5", "🇲🇺T0"],
            &["💖T0", "💖T1", "💖T2", "💖T3"],
        ),
    ),
    (
        "🇲🇺T3",
        (
            &["🇲🇺T2", "🇲🇺T4", "🇲🇺T0", "🇲🇺U1", "🇲🇺U6"],
            &["💖T0", "💖T1", "💖T2", "💖T3"],
        ),
    ),
    (
        "🇲🇺T4",
        (
            &["🇲🇺T2", "🇲🇺T3", "🇲🇺T5", "🇲🇺T7", "🇲🇺U6", "🇲🇺U8", "👻X7"],
            &[],
        ),
    ),
    ("🇲🇺T5", (&["🇲🇺T1", "🇲🇺T2", "🇲🇺T4", "🇲🇺T6", "🇲🇺T7"], &[])),
    (
        "🇲🇺T6",
        (&["🇲🇺T8", "🇲🇺T1", "🇲🇺T5", "🇲🇺T7", "🇲🇺S3", "🇲🇺S4"], &[]),
    ),
    ("🇲🇺T7", (&["🇲🇺T8", "🇲🇺T4", "🇲🇺T5", "🇲🇺T6"], &[])),
    ("🇲🇺T8", (&["🇲🇺T6", "🇲🇺T7", "🇲🇺S4", "🇲🇺R7"], &[])),
    (
        "🇲🇺U0",
        (&["🇲🇺U1", "🇲🇺U2", "🇲🇺U3"], &["💖T0", "💖T1", "💖T2", "💖T3"]),
    ),
    (
        "🇲🇺U1",
        (
            &["🇲🇺U2", "🇲🇺U4", "🇲🇺U5", "🇲🇺U6", "🇲🇺U0", "🇲🇺T3"],
            &["💖T0", "💖T1", "💖T2", "💖T3"],
        ),
    ),
    (
        "🇲🇺U2",
        (
            &["🇲🇺U1", "🇲🇺U3", "🇲🇺U4", "🇲🇺U0"],
            &["💖T0", "💖T1", "💖T2", "💖T3"],
        ),
    ),
    (
        "🇲🇺U3",
        (
            &["🇲🇺U2", "🇲🇺U4", "🇲🇺U0", "👻S8", "👻R4", "👻Q8"],
            &["💖T0", "💖T1", "💖T2", "💖T3"],
        ),
    ),
    (
        "🇲🇺U4",
        (&["🇲🇺U1", "🇲🇺U2", "🇲🇺U3", "🇲🇺U5", "👻S8", "👻S7"], &[]),
    ),
    (
        "🇲🇺U5",
        (
            &["🇲🇺U1", "🇲🇺U4", "🇲🇺U6", "🇲🇺U7", "🇲🇺U8", "👻S7", "👻S4"],
            &[],
        ),
    ),
    ("🇲🇺U6", (&["🇲🇺U8", "🇲🇺U1", "🇲🇺U5", "🇲🇺T3", "🇲🇺T4"], &[])),
    (
        "🇲🇺U7",
        (&["🇲🇺U8", "🇲🇺U5", "👻S4", "👻T8", "👻X8", "👻X7"], &[]),
    ),
    ("🇲🇺U8", (&["🇲🇺U5", "🇲🇺U6", "🇲🇺U7", "🇲🇺T4", "👻X7"], &[])),
    (
        "⚽P0",
        (&["⚽P1", "⚽P2", "⚽P3"], &["💖Q0", "💖Q1", "💖Q2", "💖Q3"]),
    ),
    (
        "⚽P1",
        (
            &["⚽P2", "⚽P5", "⚽P6", "⚽P0", "👻P7", "👻Q3"],
            &["💖Q0", "💖Q1", "💖Q2", "💖Q3"],
        ),
    ),
    (
        "⚽P2",
        (
            &["⚽P1", "⚽P3", "⚽P4", "⚽P5", "⚽P0"],
            &["💖Q0", "💖Q1", "💖Q2", "💖Q3"],
        ),
    ),
    (
        "⚽P3",
        (
            &["⚽P2", "⚽P4", "⚽P0", "⚽Q1", "⚽Q6"],
            &["💖Q0", "💖Q1", "💖Q2", "💖Q3"],
        ),
    ),
    (
        "⚽P4",
        (
            &["⚽P2", "⚽P3", "⚽P5", "⚽P7", "⚽Q6", "⚽Q8", "🧩U6"],
            &[],
        ),
    ),
    ("⚽P5", (&["⚽P1", "⚽P2", "⚽P4", "⚽P6", "⚽P7"], &[])),
    ("⚽P6", (&["⚽P8", "⚽P1", "⚽P5", "⚽P7", "👻P7"], &[])),
    (
        "⚽P7",
        (&["⚽P8", "⚽P4", "⚽P5", "⚽P6", "🧩U6", "🧩U4"], &[]),
    ),
    (
        "⚽P8",
        (&["⚽P6", "⚽P7", "👻P7", "👻P8", "🧩U4", "🧩U3"], &[]),
    ),
    (
        "⚽Q0",
        (&["⚽Q1", "⚽Q2", "⚽Q3"], &["💖Q0", "💖Q1", "💖Q2", "💖Q3"]),
    ),
    (
        "⚽Q1",
        (
            &["⚽Q2", "⚽Q4", "⚽Q5", "⚽Q6", "⚽Q0", "⚽P3"],
            &["💖Q0", "💖Q1", "💖Q2", "💖Q3"],
        ),
    ),
    (
        "⚽Q2",
        (
            &["⚽Q1", "⚽Q3", "⚽Q4", "⚽Q0"],
            &["💖Q0", "💖Q1", "💖Q2", "💖Q3"],
        ),
    ),
    (
        "⚽Q3",
        (
            &["⚽Q2", "⚽Q4", "⚽Q0", "⚽R1", "⚽R6"],
            &["💖Q0", "💖Q1", "💖Q2", "💖Q3"],
        ),
    ),
    (
        "⚽Q4",
        (
            &["⚽Q1", "⚽Q2", "⚽Q3", "⚽Q5", "⚽Q7", "⚽R6", "⚽R8"],
            &[],
        ),
    ),
    ("⚽Q5", (&["⚽Q1", "⚽Q4", "⚽Q6", "⚽Q7"], &[])),
    (
        "⚽Q6",
        (&["⚽Q8", "⚽Q1", "⚽Q5", "⚽Q7", "⚽P3", "⚽P4"], &[]),
    ),
    ("⚽Q7", (&["⚽Q8", "⚽Q4", "⚽Q5", "⚽Q6", "⚽R8"], &[])),
    ("⚽Q8", (&["⚽Q6", "⚽Q7", "⚽P4", "🧩U6"], &[])),
    (
        "⚽R0",
        (&["⚽R1", "⚽R2", "⚽R3"], &["💖Q0", "💖Q1", "💖Q2", "💖Q3"]),
    ),
    (
        "⚽R1",
        (
            &["⚽R2", "⚽R5", "⚽R6", "⚽R0", "⚽Q3"],
            &["💖Q0", "💖Q1", "💖Q2", "💖Q3"],
        ),
    ),
    (
        "⚽R2",
        (
            &["⚽R1", "⚽R3", "⚽R4", "⚽R5", "⚽R0"],
            &["💖Q0", "💖Q1", "💖Q2", "💖Q3"],
        ),
    ),
    (
        "⚽R3",
        (
            &["⚽R2", "⚽R4", "⚽R0", "⚽S8", "💖T1"],
            &["💖Q0", "💖Q1", "💖Q2", "💖Q3"],
        ),
    ),
    (
        "⚽R4",
        (
            &["⚽R2", "⚽R3", "⚽R5", "⚽R7", "⚽R8", "⚽S8", "⚽S7"],
            &[],
        ),
    ),
    ("⚽R5", (&["⚽R1", "⚽R2", "⚽R4", "⚽R6", "⚽R8"], &[])),
    ("⚽R6", (&["⚽R8", "⚽R1", "⚽R5", "⚽Q3", "⚽Q4"], &[])),
    ("⚽R7", (&["⚽R8", "⚽R4", "⚽S7", "⚽S4", "⚽T8"], &[])),
    (
        "⚽R8",
        (&["⚽R4", "⚽R5", "⚽R6", "⚽R7", "⚽Q4", "⚽Q7"], &[]),
    ),
    (
        "⚽S0",
        (&["⚽S1", "⚽S2", "⚽S3"], &["😎P0", "😎P1", "😎P2", "😎P3"]),
    ),
    (
        "⚽S1",
        (
            &["⚽S2", "⚽S5", "⚽S6", "⚽S0", "💖T6", "💖T8", "💖U3"],
            &["😎P0", "😎P1", "😎P2", "😎P3"],
        ),
    ),
    (
        "⚽S2",
        (
            &["⚽S1", "⚽S3", "⚽S4", "⚽S5", "⚽S0"],
            &["😎P0", "😎P1", "😎P2", "😎P3"],
        ),
    ),
    (
        "⚽S3",
        (
            &["⚽S2", "⚽S4", "⚽S0", "⚽T1"],
            &["😎P0", "😎P1", "😎P2", "😎P3"],
        ),
    ),
    (
        "⚽S4",
        (
            &["⚽S2", "⚽S3", "⚽S5", "⚽S7", "⚽T1", "⚽T8", "⚽R7"],
            &[],
        ),
    ),
    ("⚽S5", (&["⚽S1", "⚽S2", "⚽S4", "⚽S6", "⚽S7"], &[])),
    ("⚽S6", (&["⚽S8", "⚽S1", "⚽S5", "⚽S7", "💖T6"], &[])),
    (
        "⚽S7",
        (&["⚽S8", "⚽S4", "⚽S5", "⚽S6", "⚽R4", "⚽R7"], &[]),
    ),
    (
        "⚽S8",
        (&["⚽S6", "⚽S7", "⚽R3", "⚽R4", "💖T1", "💖T6"], &[]),
    ),
    (
        "⚽T0",
        (&["⚽T1", "⚽T2", "⚽T3"], &["😎P0", "😎P1", "😎P2", "😎P3"]),
    ),
    (
        "⚽T1",
        (
            &["⚽T8", "⚽T2", "⚽T5", "⚽T7", "⚽T0", "⚽S3", "⚽S4"],
            &["😎P0", "😎P1", "😎P2", "😎P3"],
        ),
    ),
    (
        "⚽T2",
        (
            &["⚽T1", "⚽T3", "⚽T4", "⚽T5", "⚽T0"],
            &["😎P0", "😎P1", "😎P2", "😎P3"],
        ),
    ),
    (
        "⚽T3",
        (
            &["⚽T2", "⚽T4", "⚽T0", "⚽U1"],
            &["😎P0", "😎P1", "😎P2", "😎P3"],
        ),
    ),
    (
        "⚽T4",
        (
            &["⚽T2", "⚽T3", "⚽T5", "⚽T6", "⚽U1", "⚽U7", "⚽U8"],
            &[],
        ),
    ),
    ("⚽T5", (&["⚽T1", "⚽T2", "⚽T4", "⚽T6", "⚽T7"], &[])),
    ("⚽T6", (&["⚽T4", "⚽T5", "⚽T7", "⚽U8"], &[])),
    ("⚽T7", (&["⚽T8", "⚽T1", "⚽T5", "⚽T6"], &[])),
    ("⚽T8", (&["⚽T1", "⚽T7", "⚽S4", "⚽R7"], &[])),
    (
        "⚽U0",
        (&["⚽U1", "⚽U2", "⚽U3"], &["😎P0", "😎P1", "😎P2", "😎P3"]),
    ),
    (
        "⚽U1",
        (
            &["⚽U2", "⚽U5", "⚽U7", "⚽U0", "⚽T3", "⚽T4"],
            &["😎P0", "😎P1", "😎P2", "😎P3"],
        ),
    ),
    (
        "⚽U2",
        (
            &["⚽U1", "⚽U3", "⚽U5", "⚽U0"],
            &["😎P0", "😎P1", "😎P2", "😎P3"],
        ),
    ),
    (
        "⚽U3",
        (
            &[
                "⚽U2", "⚽U4", "⚽U5", "⚽U0", "🧩S8", "🧩S7", "🧩R4", "🧩Q8",
            ],
            &["😎P0", "😎P1", "😎P2", "😎P3"],
        ),
    ),
    ("⚽U4", (&["⚽U3", "⚽U5", "⚽U6", "⚽U7", "🧩S7"], &[])),
    ("⚽U5", (&["⚽U1", "⚽U2", "⚽U3", "⚽U4", "⚽U7"], &[])),
    ("⚽U6", (&["⚽U8", "⚽U4", "⚽U7", "🧩S7", "🧩T8"], &[])),
    (
        "⚽U7",
        (&["⚽U1", "⚽U4", "⚽U5", "⚽U6", "⚽U8", "⚽T4"], &[]),
    ),
    ("⚽U8", (&["⚽U6", "⚽U7", "⚽T4", "⚽T6"], &[])),
    (
        "🌱P0",
        (&["🌱P1", "🌱P2", "🌱P3"], &["💖U0", "💖U1", "💖U2", "💖U3"]),
    ),
    (
        "🌱P1",
        (
            &["🌱P2", "🌱P4", "🌱P5", "🌱P6", "🌱P0", "🧩P7", "🧩Q3"],
            &["💖U0", "💖U1", "💖U2", "💖U3"],
        ),
    ),
    (
        "🌱P2",
        (
            &["🌱P1", "🌱P3", "🌱P4", "🌱P0"],
            &["💖U0", "💖U1", "💖U2", "💖U3"],
        ),
    ),
    (
        "🌱P3",
        (
            &["🌱P2", "🌱P4", "🌱P0", "🌱Q1", "🌱Q7"],
            &["💖U0", "💖U1", "💖U2", "💖U3"],
        ),
    ),
    (
        "🌱P4",
        (
            &["🌱P1", "🌱P2", "🌱P3", "🌱P5", "🌱P7", "🌱Q7", "🌱Q8"],
            &[],
        ),
    ),
    ("🌱P5", (&["🌱P1", "🌱P4", "🌱P6", "🌱P7"], &[])),
    ("🌱P6", (&["🌱P8", "🌱P1", "🌱P5", "🌱P7", "🧩P7"], &[])),
    (
        "🌱P7",
        (
            &["🌱P8", "🌱P4", "🌱P5", "🌱P6", "🌱Q8", "🌵U7", "🌵U4"],
            &[],
        ),
    ),
    (
        "🌱P8",
        (&["🌱P6", "🌱P7", "🧩P7", "🧩P8", "🌵U4", "🌵U3"], &[]),
    ),
    (
        "🌱Q0",
        (&["🌱Q1", "🌱Q2", "🌱Q3"], &["💖U0", "💖U1", "💖U2", "💖U3"]),
    ),
    (
        "🌱Q1",
        (
            &["🌱Q2", "🌱Q5", "🌱Q7", "🌱Q0", "🌱P3"],
            &["💖U0", "💖U1", "💖U2", "💖U3"],
        ),
    ),
    (
        "🌱Q2",
        (
            &["🌱Q1", "🌱Q3", "🌱Q5", "🌱Q0"],
            &["💖U0", "💖U1", "💖U2", "💖U3"],
        ),
    ),
    (
        "🌱Q3",
        (
            &["🌱Q2", "🌱Q4", "🌱Q5", "🌱Q0", "🌱R1", "🌱R6"],
            &["💖U0", "💖U1", "💖U2", "💖U3"],
        ),
    ),
    (
        "🌱Q4",
        (&["🌱Q3", "🌱Q5", "🌱Q6", "🌱Q7", "🌱R6", "🌱R8"], &[]),
    ),
    ("🌱Q5", (&["🌱Q1", "🌱Q2", "🌱Q3", "🌱Q4", "🌱Q7"], &[])),
    ("🌱Q6", (&["🌱Q8", "🌱Q4", "🌱Q7", "🌱R8"], &[])),
    (
        "🌱Q7",
        (
            &["🌱Q1", "🌱Q4", "🌱Q5", "🌱Q6", "🌱Q8", "🌱P3", "🌱P4"],
            &[],
        ),
    ),
    ("🌱Q8", (&["🌱Q6", "🌱Q7", "🌱P4", "🌱P7", "🌵U7"], &[])),
    (
        "🌱R0",
        (&["🌱R1", "🌱R2", "🌱R3"], &["💖U0", "💖U1", "💖U2", "💖U3"]),
    ),
    (
        "🌱R1",
        (
            &["🌱R2", "🌱R6", "🌱R0", "🌱Q3"],
            &["💖U0", "💖U1", "💖U2", "💖U3"],
        ),
    ),
    (
        "🌱R2",
        (
            &["🌱R1", "🌱R3", "🌱R4", "🌱R5", "🌱R6", "🌱R0"],
            &["💖U0", "💖U1", "💖U2", "💖U3"],
        ),
    ),
    (
        "🌱R3",
        (
            &["🌱R2", "🌱R4", "🌱R0", "🌱S8", "🌱S7", "😎P1"],
            &["💖U0", "💖U1", "💖U2", "💖U3"],
        ),
    ),
    ("🌱R4", (&["🌱R2", "🌱R3", "🌱R5", "🌱R7", "🌱S7"], &[])),
    ("🌱R5", (&["🌱R2", "🌱R4", "🌱R6", "🌱R7", "🌱R8"], &[])),
    (
        "🌱R6",
        (&["🌱R8", "🌱R1", "🌱R2", "🌱R5", "🌱Q3", "🌱Q4"], &[]),
    ),
    ("🌱R7", (&["🌱R8", "🌱R4", "🌱R5", "🌱S7", "🌱T8"], &[])),
    ("🌱R8", (&["🌱R5", "🌱R6", "🌱R7", "🌱Q4", "🌱Q6"], &[])),
    (
        "🌱S0",
        (&["🌱S1", "🌱S2", "🌱S3"], &["😎T0", "😎T1", "😎T2", "😎T3"]),
    ),
    (
        "🌱S1",
        (
            &["🌱S2", "🌱S6", "🌱S0", "😎P8", "😎Q3"],
            &["😎T0", "😎T1", "😎T2", "😎T3"],
        ),
    ),
    (
        "🌱S2",
        (
            &["🌱S1", "🌱S3", "🌱S4", "🌱S5", "🌱S6", "🌱S0"],
            &["😎T0", "😎T1", "😎T2", "😎T3"],
        ),
    ),
    (
        "🌱S3",
        (
            &["🌱S2", "🌱S4", "🌱S0", "🌱T1", "🌱T8"],
            &["😎T0", "😎T1", "😎T2", "😎T3"],
        ),
    ),
    ("🌱S4", (&["🌱S2", "🌱S3", "🌱S5", "🌱S7", "🌱T8"], &[])),
    ("🌱S5", (&["🌱S2", "🌱S4", "🌱S6", "🌱S7", "🌱S8"], &[])),
    (
        "🌱S6",
        (&["🌱S8", "🌱S1", "🌱S2", "🌱S5", "😎P6", "😎P8"], &[]),
    ),
    (
        "🌱S7",
        (
            &["🌱S8", "🌱S4", "🌱S5", "🌱T8", "🌱R3", "🌱R4", "🌱R7"],
            &[],
        ),
    ),
    (
        "🌱S8",
        (&["🌱S5", "🌱S6", "🌱S7", "🌱R3", "😎P1", "😎P6"], &[]),
    ),
    (
        "🌱T0",
        (&["🌱T1", "🌱T2", "🌱T3"], &["😎T0", "😎T1", "😎T2", "😎T3"]),
    ),
    (
        "🌱T1",
        (
            &["🌱T8", "🌱T2", "🌱T6", "🌱T0", "🌱S3"],
            &["😎T0", "😎T1", "😎T2", "😎T3"],
        ),
    ),
    (
        "🌱T2",
        (
            &["🌱T1", "🌱T3", "🌱T5", "🌱T6", "🌱T0"],
            &["😎T0", "😎T1", "😎T2", "😎T3"],
        ),
    ),
    (
        "🌱T3",
        (
            &["🌱T2", "🌱T4", "🌱T5", "🌱T0", "🌱U1", "🌱U6"],
            &["😎T0", "😎T1", "😎T2", "😎T3"],
        ),
    ),
    ("🌱T4", (&["🌱T3", "🌱T5", "🌱T7", "🌱U6", "🌱U8"], &[])),
    ("🌱T5", (&["🌱T2", "🌱T3", "🌱T4", "🌱T6", "🌱T7"], &[])),
    ("🌱T6", (&["🌱T1", "🌱T2", "🌱T5", "🌱T7", "🌱T8"], &[])),
    ("🌱T7", (&["🌱T8", "🌱T4", "🌱T5", "🌱T6"], &[])),
    (
        "🌱T8",
        (
            &["🌱T1", "🌱T6", "🌱T7", "🌱S3", "🌱S4", "🌱S7", "🌱R7"],
            &[],
        ),
    ),
    (
        "🌱U0",
        (&["🌱U1", "🌱U2", "🌱U3"], &["😎T0", "😎T1", "😎T2", "😎T3"]),
    ),
    (
        "🌱U1",
        (
            &["🌱U2", "🌱U6", "🌱U0", "🌱T3"],
            &["😎T0", "😎T1", "😎T2", "😎T3"],
        ),
    ),
    (
        "🌱U2",
        (
            &["🌱U1", "🌱U3", "🌱U4", "🌱U5", "🌱U6", "🌱U0"],
            &["😎T0", "😎T1", "😎T2", "😎T3"],
        ),
    ),
    (
        "🌱U3",
        (
            &["🌱U2", "🌱U4", "🌱U0", "🌵S8", "🌵S7", "🌵R7", "🌵Q8"],
            &["😎T0", "😎T1", "😎T2", "😎T3"],
        ),
    ),
    (
        "🌱U4",
        (&["🌱U2", "🌱U3", "🌱U5", "🌱U7", "🌵S7", "🌵S4"], &[]),
    ),
    ("🌱U5", (&["🌱U2", "🌱U4", "🌱U6", "🌱U7", "🌱U8"], &[])),
    (
        "🌱U6",
        (&["🌱U8", "🌱U1", "🌱U2", "🌱U5", "🌱T3", "🌱T4"], &[]),
    ),
    ("🌱U7", (&["🌱U8", "🌱U4", "🌱U5", "🌵S4", "🌵T8"], &[])),
    ("🌱U8", (&["🌱U5", "🌱U6", "🌱U7", "🌱T4"], &[])),
    (
        "🐀P0",
        (&["🐀P1", "🐀P2", "🐀P3"], &["😎Q0", "😎Q1", "😎Q2", "😎Q3"]),
    ),
    (
        "🐀P1",
        (
            &[
                "🐀P8", "🐀P2", "🐀P5", "🐀P7", "🐀P0", "🌵P6", "🌵P8", "🌵Q3",
            ],
            &["😎Q0", "😎Q1", "😎Q2", "😎Q3"],
        ),
    ),
    (
        "🐀P2",
        (
            &["🐀P1", "🐀P3", "🐀P4", "🐀P5", "🐀P0"],
            &["😎Q0", "😎Q1", "😎Q2", "😎Q3"],
        ),
    ),
    (
        "🐀P3",
        (
            &["🐀P2", "🐀P4", "🐀P0", "🐀Q1"],
            &["😎Q0", "😎Q1", "😎Q2", "😎Q3"],
        ),
    ),
    (
        "🐀P4",
        (
            &["🐀P2", "🐀P3", "🐀P5", "🐀P6", "🐀Q1", "🐀Q7", "🐀Q8"],
            &[],
        ),
    ),
    ("🐀P5", (&["🐀P1", "🐀P2", "🐀P4", "🐀P6", "🐀P7"], &[])),
    ("🐀P6", (&["🐀P4", "🐀P5", "🐀P7", "🐀Q8", "🧀U4"], &[])),
    (
        "🐀P7",
        (&["🐀P8", "🐀P1", "🐀P5", "🐀P6", "🧀U4", "🧀U3"], &[]),
    ),
    ("🐀P8", (&["🐀P1", "🐀P7", "🌵P8", "🧀U3"], &[])),
    (
        "🐀Q0",
        (&["🐀Q1", "🐀Q2", "🐀Q3"], &["😎Q0", "😎Q1", "😎Q2", "😎Q3"]),
    ),
    (
        "🐀Q1",
        (
            &["🐀Q2", "🐀Q5", "🐀Q7", "🐀Q0", "🐀P3", "🐀P4"],
            &["😎Q0", "😎Q1", "😎Q2", "😎Q3"],
        ),
    ),
    (
        "🐀Q2",
        (
            &["🐀Q1", "🐀Q3", "🐀Q5", "🐀Q0"],
            &["😎Q0", "😎Q1", "😎Q2", "😎Q3"],
        ),
    ),
    (
        "🐀Q3",
        (
            &["🐀Q2", "🐀Q4", "🐀Q5", "🐀Q0", "🐀R1", "🐀R6"],
            &["😎Q0", "😎Q1", "😎Q2", "😎Q3"],
        ),
    ),
    (
        "🐀Q4",
        (&["🐀Q3", "🐀Q5", "🐀Q6", "🐀Q7", "🐀R6", "🐀R8"], &[]),
    ),
    ("🐀Q5", (&["🐀Q1", "🐀Q2", "🐀Q3", "🐀Q4", "🐀Q7"], &[])),
    ("🐀Q6", (&["🐀Q8", "🐀Q4", "🐀Q7", "🐀R8"], &[])),
    (
        "🐀Q7",
        (&["🐀Q1", "🐀Q4", "🐀Q5", "🐀Q6", "🐀Q8", "🐀P4"], &[]),
    ),
    ("🐀Q8", (&["🐀Q6", "🐀Q7", "🐀P4", "🐀P6", "🧀U4"], &[])),
    (
        "🐀R0",
        (&["🐀R1", "🐀R2", "🐀R3"], &["😎Q0", "😎Q1", "😎Q2", "😎Q3"]),
    ),
    (
        "🐀R1",
        (
            &["🐀R2", "🐀R4", "🐀R5", "🐀R6", "🐀R0", "🐀Q3"],
            &["😎Q0", "😎Q1", "😎Q2", "😎Q3"],
        ),
    ),
    (
        "🐀R2",
        (
            &["🐀R1", "🐀R3", "🐀R4", "🐀R0"],
            &["😎Q0", "😎Q1", "😎Q2", "😎Q3"],
        ),
    ),
    (
        "🐀R3",
        (
            &["🐀R2", "🐀R4", "🐀R0", "🐀S8", "🐀S7", "😎T1"],
            &["😎Q0", "😎Q1", "😎Q2", "😎Q3"],
        ),
    ),
    (
        "🐀R4",
        (&["🐀R1", "🐀R2", "🐀R3", "🐀R5", "🐀R7", "🐀S7"], &[]),
    ),
    ("🐀R5", (&["🐀R1", "🐀R4", "🐀R6", "🐀R7"], &[])),
    (
        "🐀R6",
        (&["🐀R8", "🐀R1", "🐀R5", "🐀R7", "🐀Q3", "🐀Q4"], &[]),
    ),
    (
        "🐀R7",
        (&["🐀R8", "🐀R4", "🐀R5", "🐀R6", "🐀S7", "🐀T8"], &[]),
    ),
    ("🐀R8", (&["🐀R6", "🐀R7", "🐀Q4", "🐀Q6"], &[])),
    (
        "🐀S0",
        (&["🐀S1", "🐀S2", "🐀S3"], &["🏝️P0", "🏝️P1", "🏝️P2", "🏝️P3"]),
    ),
    (
        "🐀S1",
        (
            &["🐀S2", "🐀S4", "🐀S5", "🐀S6", "🐀S0", "😎T8", "😎U3"],
            &["🏝️P0", "🏝️P1", "🏝️P2", "🏝️P3"],
        ),
    ),
    (
        "🐀S2",
        (
            &["🐀S1", "🐀S3", "🐀S4", "🐀S0"],
            &["🏝️P0", "🏝️P1", "🏝️P2", "🏝️P3"],
        ),
    ),
    (
        "🐀S3",
        (
            &["🐀S2", "🐀S4", "🐀S0", "🐀T1", "🐀T6"],
            &["🏝️P0", "🏝️P1", "🏝️P2", "🏝️P3"],
        ),
    ),
    (
        "🐀S4",
        (&["🐀S1", "🐀S2", "🐀S3", "🐀S5", "🐀T6", "🐀T8"], &[]),
    ),
    (
        "🐀S5",
        (&["🐀S1", "🐀S4", "🐀S6", "🐀S7", "🐀S8", "🐀T8"], &[]),
    ),
    ("🐀S6", (&["🐀S8", "🐀S1", "🐀S5", "😎T7", "😎T8"], &[])),
    (
        "🐀S7",
        (&["🐀S8", "🐀S5", "🐀T8", "🐀R3", "🐀R4", "🐀R7"], &[]),
    ),
    (
        "🐀S8",
        (&["🐀S5", "🐀S6", "🐀S7", "🐀R3", "😎T1", "😎T7"], &[]),
    ),
    (
        "🐀T0",
        (&["🐀T1", "🐀T4", "🐀T0"], &["🏝️P0", "🏝️P1", "🏝️P2", "🏝️P3"]),
    ),
    (
        "🐀T1",
        (
            &["🐀T4", "🐀T5", "🐀T6", "🐀T0", "🐀S3"],
            &["🏝️P0", "🏝️P1", "🏝️P2", "🏝️P3"],
        ),
    ),
    (
        "🐀T4",
        (
            &["🐀T1", "🐀T5", "🐀T7", "🐀T0"],
            &["🏝️P0", "🏝️P1", "🏝️P2", "🏝️P3"],
        ),
    ),
    ("🐀T5", (&["🐀T1", "🐀T4", "🐀T6", "🐀T7"], &[])),
    (
        "🐀T6",
        (&["🐀T8", "🐀T1", "🐀T5", "🐀T7", "🐀S3", "🐀S4"], &[]),
    ),
    ("🐀T7", (&["🐀T8", "🐀T4", "🐀T5", "🐀T6"], &[])),
    (
        "🐀T8",
        (&["🐀T6", "🐀T7", "🐀S4", "🐀S5", "🐀S7", "🐀R7"], &[]),
    ),
    (
        "🏝️P0",
        (
            &["🏝️P1", "🏝️P2", "🏝️P3"],
            &["🐀S0", "🐀S1", "🐀S2", "🐀S3", "🐀T0", "🐀T1", "🐀T4"],
        ),
    ),
    (
        "🏝️P1",
        (
            &["🏝️P2", "🏝️P5", "🏝️P6", "🏝️P0", "🐍S8", "🐍R3"],
            &["🐀S0", "🐀S1", "🐀S2", "🐀S3", "🐀T0", "🐀T1", "🐀T4"],
        ),
    ),
    (
        "🏝️P2",
        (
            &["🏝️P1", "🏝️P3", "🏝️P4", "🏝️P5", "🏝️P0"],
            &["🐀S0", "🐀S1", "🐀S2", "🐀S3", "🐀T0", "🐀T1", "🐀T4"],
        ),
    ),
    (
        "🏝️P3",
        (
            &["🏝️P2", "🏝️P4", "🏝️P0", "🏝️R8", "😎U1"],
            &["🐀S0", "🐀S1", "🐀S2", "🐀S3", "🐀T0", "🐀T1", "🐀T4"],
        ),
    ),
    (
        "🏝️P4",
        (
            &["🏝️P2", "🏝️P3", "🏝️P5", "🏝️P7", "🏝️P8", "🏝️R7", "🏝️R8"],
            &[],
        ),
    ),
    ("🏝️P5", (&["🏝️P1", "🏝️P2", "🏝️P4", "🏝️P6", "🏝️P8"], &[])),
    (
        "🏝️P6",
        (&["🏝️P8", "🏝️P1", "🏝️P5", "🐍S8", "🐍S6", "🐍S1"], &[]),
    ),
    (
        "🏝️P7",
        (&["🏝️P8", "🏝️P4", "🏝️Q6", "🏝️Q4", "🏝️R6", "🏝️R7"], &[]),
    ),
    (
        "🏝️P8",
        (
            &["🏝️P4", "🏝️P5", "🏝️P6", "🏝️P7", "🏝️Q4", "🏝️Q3", "🐍S1"],
            &[],
        ),
    ),
    (
        "🏝️Q0",
        (
            &["🏝️Q1", "🏝️Q2", "🏝️Q3"],
            &[
                "♾️P0", "♾️P1", "♾️P2", "♾️P3", "♾️Q0", "♾️Q1", "♾️Q2", "♾️Q3", "♾️R0", "♾️R1",
                "♾️R2", "♾️R3", "🌙Q0", "🌙Q1", "🌙Q2", "🌙Q3",
            ],
        ),
    ),
    (
        "🏝️Q1",
        (
            &[
                "🏝️Q8",
                "🏝️Q2",
                "🏝️Q5",
                "🏝️Q7",
                "🏝️Q0",
                "🏝️S1",
                "🏝️S6",
                "🏝️T3",
            ],
            &[
                "♾️P0", "♾️P1", "♾️P2", "♾️P3", "♾️Q0", "♾️Q1", "♾️Q2", "♾️Q3", "♾️R0", "♾️R1",
                "♾️R2", "♾️R3", "🌙Q0", "🌙Q1", "🌙Q2", "🌙Q3",
            ],
        ),
    ),
    (
        "🏝️Q2",
        (
            &["🏝️Q1", "🏝️Q3", "🏝️Q4", "🏝️Q5", "🏝️Q0"],
            &[
                "♾️P0", "♾️P1", "♾️P2", "♾️P3", "♾️Q0", "♾️Q1", "♾️Q2", "♾️Q3", "♾️R0", "♾️R1",
                "♾️R2", "♾️R3", "🌙Q0", "🌙Q1", "🌙Q2", "🌙Q3",
            ],
        ),
    ),
    (
        "🏝️Q3",
        (
            &["🏝️Q2", "🏝️Q4", "🏝️Q0", "🏝️P8", "🐍S1"],
            &[
                "♾️P0", "♾️P1", "♾️P2", "♾️P3", "♾️Q0", "♾️Q1", "♾️Q2", "♾️Q3", "♾️R0", "♾️R1",
                "♾️R2", "♾️R3", "🌙Q0", "🌙Q1", "🌙Q2", "🌙Q3",
            ],
        ),
    ),
    (
        "🏝️Q4",
        (&["🏝️Q2", "🏝️Q3", "🏝️Q5", "🏝️Q6", "🏝️P7", "🏝️P8"], &[]),
    ),
    ("🏝️Q5", (&["🏝️Q1", "🏝️Q2", "🏝️Q4", "🏝️Q6", "🏝️Q7"], &[])),
    ("🏝️Q6", (&["🏝️Q4", "🏝️Q5", "🏝️Q7", "🏝️P7", "🏝️R6"], &[])),
    (
        "🏝️Q7",
        (&["🏝️Q8", "🏝️Q1", "🏝️Q5", "🏝️Q6", "🏝️R6", "🏝️R4"], &[]),
    ),
    ("🏝️Q8", (&["🏝️Q1", "🏝️Q7", "🏝️R4", "🏝️R3", "🏝️S1"], &[])),
    ("🏝️R0", (&["🏝️R1", "🏝️R2", "🏝️R3"], &[])),
    (
        "🏝️R1",
        (
            &[
                "🏝️R8",
                "🏝️R2",
                "🏝️R5",
                "🏝️R7",
                "🏝️R0",
                "😎U8",
                "😎U6",
                "😎U1",
                "😎S3",
            ],
            &[],
        ),
    ),
    ("🏝️R2", (&["🏝️R1", "🏝️R3", "🏝️R4", "🏝️R5", "🏝️R0"], &[])),
    ("🏝️R3", (&["🏝️R2", "🏝️R4", "🏝️R0", "🏝️Q8", "🏝️S1"], &[])),
    (
        "🏝️R4",
        (&["🏝️R2", "🏝️R3", "🏝️R5", "🏝️R6", "🏝️Q7", "🏝️Q8"], &[]),
    ),
    ("🏝️R5", (&["🏝️R1", "🏝️R2", "🏝️R4", "🏝️R6", "🏝️R7"], &[])),
    (
        "🏝️R6",
        (&["🏝️R4", "🏝️R5", "🏝️R7", "🏝️Q6", "🏝️Q7", "🏝️P7"], &[]),
    ),
    (
        "🏝️R7",
        (&["🏝️R8", "🏝️R1", "🏝️R5", "🏝️R6", "🏝️P7", "🏝️P4"], &[]),
    ),
    ("🏝️R8", (&["🏝️R1", "🏝️R7", "🏝️P4", "🏝️P3", "😎U1"], &[])),
    ("🏝️S0", (&["🏝️S1", "🏝️S2", "🏝️S3"], &[])),
    (
        "🏝️S1",
        (&["🏝️S2", "🏝️S6", "🏝️S0", "🏝️Q8", "🏝️Q1", "🏝️R3"], &[]),
    ),
    (
        "🏝️S2",
        (&["🏝️S1", "🏝️S3", "🏝️S4", "🏝️S5", "🏝️S6", "🏝️S0"], &[]),
    ),
    (
        "🏝️S3",
        (&["🏝️S2", "🏝️S4", "🏝️S0", "🏝️U7", "🏝️U8", "💖R1"], &[]),
    ),
    (
        "🏝️S4",
        (&["🏝️S2", "🏝️S3", "🏝️S5", "🏝️S7", "🏝️U4", "🏝️U7"], &[]),
    ),
    ("🏝️S5", (&["🏝️S2", "🏝️S4", "🏝️S6", "🏝️S7"], &[])),
    ("🏝️S6", (&["🏝️S1", "🏝️S2", "🏝️S5", "🏝️Q1"], &[])),
    ("🏝️S7", (&["🏝️S4", "🏝️S5", "🏝️T6", "🏝️T4", "🏝️U4"], &[])),
    (
        "🏝️T0",
        (
            &["🏝️T1", "🏝️T2", "🏝️T3"],
            &[
                "🐍S0", "🐍S1", "🐍S2", "🐍S3", "🐍T0", "🐍T1", "🐍T2", "🐍T3", "🐍U0", "🐍U1",
                "🐍U2", "🐍U3",
            ],
        ),
    ),
    (
        "🏝️T1",
        (
            &["🏝️T2", "🏝️T5", "🏝️T7", "🏝️T0", "♾️S8", "♾️S6", "♾️R3"],
            &[
                "🐍S0", "🐍S1", "🐍S2", "🐍S3", "🐍T0", "🐍T1", "🐍T2", "🐍T3", "🐍U0", "🐍U1",
                "🐍U2", "🐍U3",
            ],
        ),
    ),
    (
        "🏝️T2",
        (
            &["🏝️T1", "🏝️T3", "🏝️T5", "🏝️T0"],
            &[
                "🐍S0", "🐍S1", "🐍S2", "🐍S3", "🐍T0", "🐍T1", "🐍T2", "🐍T3", "🐍U0", "🐍U1",
                "🐍U2", "🐍U3",
            ],
        ),
    ),
    (
        "🏝️T3",
        (
            &["🏝️T2", "🏝️T4", "🏝️T5", "🏝️T0", "🏝️Q1"],
            &[
                "🐍S0", "🐍S1", "🐍S2", "🐍S3", "🐍T0", "🐍T1", "🐍T2", "🐍T3", "🐍U0", "🐍U1",
                "🐍U2", "🐍U3",
            ],
        ),
    ),
    ("🏝️T4", (&["🏝️T3", "🏝️T5", "🏝️T6", "🏝️T7", "🏝️S7"], &[])),
    ("🏝️T5", (&["🏝️T1", "🏝️T2", "🏝️T3", "🏝️T4", "🏝️T7"], &[])),
    ("🏝️T6", (&["🏝️T8", "🏝️T4", "🏝️T7", "🏝️S7", "🏝️U4"], &[])),
    (
        "🏝️T7",
        (
            &["🏝️T1", "🏝️T4", "🏝️T5", "🏝️T6", "🏝️T8", "♾️S6", "♾️S1"],
            &[],
        ),
    ),
    ("🏝️T8", (&["🏝️T6", "🏝️T7", "🏝️U4", "🏝️U3", "♾️S1"], &[])),
    (
        "🏝️U0",
        (
            &["🏝️U1", "🏝️U2", "🏝️U3"],
            &[
                "🇲🇺Q0",
                "🇲🇺Q1",
                "🇲🇺Q2",
                "🇲🇺Q3",
                "🇲🇺R0",
                "🇲🇺R1",
                "🇲🇺R2",
                "🇲🇺R3",
                "🦋Q0",
                "🦋Q1",
                "🦋Q2",
                "🦋Q3",
            ],
        ),
    ),
    (
        "🏝️U1",
        (
            &["🏝️U8", "🏝️U2", "🏝️U6", "🏝️U0", "💖R1", "💖R8", "💖P3"],
            &[
                "🇲🇺Q0",
                "🇲🇺Q1",
                "🇲🇺Q2",
                "🇲🇺Q3",
                "🇲🇺R0",
                "🇲🇺R1",
                "🇲🇺R2",
                "🇲🇺R3",
                "🦋Q0",
                "🦋Q1",
                "🦋Q2",
                "🦋Q3",
            ],
        ),
    ),
    (
        "🏝️U2",
        (
            &["🏝️U1", "🏝️U3", "🏝️U5", "🏝️U6", "🏝️U0"],
            &[
                "🇲🇺Q0",
                "🇲🇺Q1",
                "🇲🇺Q2",
                "🇲🇺Q3",
                "🇲🇺R0",
                "🇲🇺R1",
                "🇲🇺R2",
                "🇲🇺R3",
                "🦋Q0",
                "🦋Q1",
                "🦋Q2",
                "🦋Q3",
            ],
        ),
    ),
    (
        "🏝️U3",
        (
            &["🏝️U2", "🏝️U4", "🏝️U5", "🏝️U0", "🏝️T8", "♾️S1"],
            &[
                "🇲🇺Q0",
                "🇲🇺Q1",
                "🇲🇺Q2",
                "🇲🇺Q3",
                "🇲🇺R0",
                "🇲🇺R1",
                "🇲🇺R2",
                "🇲🇺R3",
                "🦋Q0",
                "🦋Q1",
                "🦋Q2",
                "🦋Q3",
            ],
        ),
    ),
    (
        "🏝️U4",
        (
            &["🏝️U3", "🏝️U5", "🏝️U7", "🏝️T6", "🏝️T8", "🏝️S7", "🏝️S4"],
            &[],
        ),
    ),
    ("🏝️U5", (&["🏝️U2", "🏝️U3", "🏝️U4", "🏝️U6", "🏝️U7"], &[])),
    ("🏝️U6", (&["🏝️U1", "🏝️U2", "🏝️U5", "🏝️U7", "🏝️U8"], &[])),
    (
        "🏝️U7",
        (&["🏝️U8", "🏝️U4", "🏝️U5", "🏝️U6", "🏝️S4", "🏝️S3"], &[]),
    ),
    ("🏝️U8", (&["🏝️U1", "🏝️U6", "🏝️U7", "🏝️S3", "💖R1"], &[])),
    (
        "💖P0",
        (
            &["💖P1", "💖P2", "💖P3"],
            &[
                "♾️S0", "♾️S1", "♾️S2", "♾️S3", "♾️T0", "♾️T1", "♾️T2", "♾️T3",
            ],
        ),
    ),
    (
        "💖P1",
        (
            &["💖P2", "💖P6", "💖P0", "🇲🇺S8", "🇲🇺R3"],
            &[
                "♾️S0", "♾️S1", "♾️S2", "♾️S3", "♾️T0", "♾️T1", "♾️T2", "♾️T3",
            ],
        ),
    ),
    (
        "💖P2",
        (
            &["💖P1", "💖P3", "💖P4", "💖P5", "💖P6", "💖P0"],
            &[
                "♾️S0", "♾️S1", "♾️S2", "♾️S3", "♾️T0", "♾️T1", "♾️T2", "♾️T3",
            ],
        ),
    ),
    (
        "💖P3",
        (
            &["💖P2", "💖P4", "💖P0", "💖R7", "💖R8", "🏝️U1"],
            &[
                "♾️S0", "♾️S1", "♾️S2", "♾️S3", "♾️T0", "♾️T1", "♾️T2", "♾️T3",
            ],
        ),
    ),
    (
        "💖P4",
        (&["💖P2", "💖P3", "💖P5", "💖P7", "💖R4", "💖R7"], &[]),
    ),
    ("💖P5", (&["💖P2", "💖P4", "💖P6", "💖P7", "💖P8"], &[])),
    (
        "💖P6",
        (&["💖P8", "💖P1", "💖P2", "💖P5", "🇲🇺S8", "🇲🇺S6"], &[]),
    ),
    (
        "💖P7",
        (
            &["💖P8", "💖P4", "💖P5", "💖Q7", "💖Q5", "💖Q4", "💖R4"],
            &[],
        ),
    ),
    (
        "💖P8",
        (
            &["💖P5", "💖P6", "💖P7", "💖Q4", "💖Q3", "🇲🇺S6", "🇲🇺S1"],
            &[],
        ),
    ),
    (
        "💖Q0",
        (
            &["💖Q1", "💖Q2", "💖Q3"],
            &[
                "⚽P0", "⚽P1", "⚽P2", "⚽P3", "⚽Q0", "⚽Q1", "⚽Q2", "⚽Q3", "⚽R0", "⚽R1",
                "⚽R2", "⚽R3", "👻Q0", "👻Q1", "👻Q2", "👻Q3",
            ],
        ),
    ),
    (
        "💖Q1",
        (
            &["💖Q2", "💖Q4", "💖Q5", "💖Q6", "💖Q0", "💖S8", "💖T3"],
            &[
                "⚽P0", "⚽P1", "⚽P2", "⚽P3", "⚽Q0", "⚽Q1", "⚽Q2", "⚽Q3", "⚽R0", "⚽R1",
                "⚽R2", "⚽R3", "👻Q0", "👻Q1", "👻Q2", "👻Q3",
            ],
        ),
    ),
    (
        "💖Q2",
        (
            &["💖Q1", "💖Q3", "💖Q4", "💖Q0"],
            &[
                "⚽P0", "⚽P1", "⚽P2", "⚽P3", "⚽Q0", "⚽Q1", "⚽Q2", "⚽Q3", "⚽R0", "⚽R1",
                "⚽R2", "⚽R3", "👻Q0", "👻Q1", "👻Q2", "👻Q3",
            ],
        ),
    ),
    (
        "💖Q3",
        (
            &["💖Q2", "💖Q4", "💖Q0", "💖P8", "🇲🇺S1"],
            &[
                "⚽P0", "⚽P1", "⚽P2", "⚽P3", "⚽Q0", "⚽Q1", "⚽Q2", "⚽Q3", "⚽R0", "⚽R1",
                "⚽R2", "⚽R3", "👻Q0", "👻Q1", "👻Q2", "👻Q3",
            ],
        ),
    ),
    (
        "💖Q4",
        (&["💖Q1", "💖Q2", "💖Q3", "💖Q5", "💖P7", "💖P8"], &[]),
    ),
    (
        "💖Q5",
        (&["💖Q1", "💖Q4", "💖Q6", "💖Q7", "💖Q8", "💖P7"], &[]),
    ),
    ("💖Q6", (&["💖Q8", "💖Q1", "💖Q5", "💖S1", "💖S8"], &[])),
    ("💖Q7", (&["💖Q8", "💖Q5", "💖P7", "💖R4", "💖R3"], &[])),
    ("💖Q8", (&["💖Q5", "💖Q6", "💖Q7", "💖R3", "💖S1"], &[])),
    ("💖R0", (&["💖R1", "💖R2", "💖R3"], &[])),
    (
        "💖R1",
        (
            &["💖R8", "💖R2", "💖R6", "💖R0", "🏝️U8", "🏝️U1", "🏝️S3"],
            &[],
        ),
    ),
    ("💖R2", (&["💖R1", "💖R3", "💖R5", "💖R6", "💖R0"], &[])),
    (
        "💖R3",
        (
            &["💖R2", "💖R4", "💖R5", "💖R0", "💖Q7", "💖Q8", "💖S1"],
            &[],
        ),
    ),
    (
        "💖R4",
        (&["💖R3", "💖R5", "💖R7", "💖Q7", "💖P7", "💖P4"], &[]),
    ),
    ("💖R5", (&["💖R2", "💖R3", "💖R4", "💖R6", "💖R7"], &[])),
    ("💖R6", (&["💖R1", "💖R2", "💖R5", "💖R7", "💖R8"], &[])),
    (
        "💖R7",
        (&["💖R8", "💖R4", "💖R5", "💖R6", "💖P4", "💖P3"], &[]),
    ),
    ("💖R8", (&["💖R1", "💖R6", "💖R7", "💖P3", "🏝️U1"], &[])),
    ("💖S0", (&["💖S1", "💖S2", "💖S3"], &[])),
    (
        "💖S1",
        (
            &["💖S8", "💖S2", "💖S6", "💖S0", "💖Q8", "💖Q6", "💖R3"],
            &[],
        ),
    ),
    ("💖S2", (&["💖S1", "💖S3", "💖S5", "💖S6", "💖S0"], &[])),
    (
        "💖S3",
        (
            &["💖S2", "💖S4", "💖S5", "💖S0", "💖U7", "💖U8", "😎R1"],
            &[],
        ),
    ),
    ("💖S4", (&["💖S3", "💖S5", "💖S7", "💖T7", "💖U7"], &[])),
    ("💖S5", (&["💖S2", "💖S3", "💖S4", "💖S6", "💖S7"], &[])),
    ("💖S6", (&["💖S1", "💖S2", "💖S5", "💖S7", "💖S8"], &[])),
    (
        "💖S7",
        (
            &["💖S8", "💖S4", "💖S5", "💖S6", "💖T7", "💖T4", "💖T3"],
            &[],
        ),
    ),
    (
        "💖S8",
        (&["💖S1", "💖S6", "💖S7", "💖T3", "💖Q6", "💖Q1"], &[]),
    ),
    (
        "💖T0",
        (
            &["💖T1", "💖T2", "💖T3"],
            &[
                "🇲🇺S0",
                "🇲🇺S1",
                "🇲🇺S2",
                "🇲🇺S3",
                "🇲🇺T0",
                "🇲🇺T1",
                "🇲🇺T2",
                "🇲🇺T3",
                "🇲🇺U0",
                "🇲🇺U1",
                "🇲🇺U2",
                "🇲🇺U3",
            ],
        ),
    ),
    (
        "💖T1",
        (
            &["💖T2", "💖T5", "💖T6", "💖T0", "⚽S8", "⚽R3"],
            &[
                "🇲🇺S0",
                "🇲🇺S1",
                "🇲🇺S2",
                "🇲🇺S3",
                "🇲🇺T0",
                "🇲🇺T1",
                "🇲🇺T2",
                "🇲🇺T3",
                "🇲🇺U0",
                "🇲🇺U1",
                "🇲🇺U2",
                "🇲🇺U3",
            ],
        ),
    ),
    (
        "💖T2",
        (
            &["💖T1", "💖T3", "💖T4", "💖T5", "💖T0"],
            &[
                "🇲🇺S0",
                "🇲🇺S1",
                "🇲🇺S2",
                "🇲🇺S3",
                "🇲🇺T0",
                "🇲🇺T1",
                "🇲🇺T2",
                "🇲🇺T3",
                "🇲🇺U0",
                "🇲🇺U1",
                "🇲🇺U2",
                "🇲🇺U3",
            ],
        ),
    ),
    (
        "💖T3",
        (
            &["💖T2", "💖T4", "💖T0", "💖S7", "💖S8", "💖Q1"],
            &[
                "🇲🇺S0",
                "🇲🇺S1",
                "🇲🇺S2",
                "🇲🇺S3",
                "🇲🇺T0",
                "🇲🇺T1",
                "🇲🇺T2",
                "🇲🇺T3",
                "🇲🇺U0",
                "🇲🇺U1",
                "🇲🇺U2",
                "🇲🇺U3",
            ],
        ),
    ),
    (
        "💖T4",
        (&["💖T2", "💖T3", "💖T5", "💖T7", "💖T8", "💖S7"], &[]),
    ),
    ("💖T5", (&["💖T1", "💖T2", "💖T4", "💖T6", "💖T8"], &[])),
    (
        "💖T6",
        (&["💖T8", "💖T1", "💖T5", "⚽S8", "⚽S6", "⚽S1"], &[]),
    ),
    (
        "💖T7",
        (
            &["💖T8", "💖T4", "💖S4", "💖S7", "💖U7", "💖U4", "💖U3"],
            &[],
        ),
    ),
    (
        "💖T8",
        (&["💖T4", "💖T5", "💖T6", "💖T7", "💖U3", "⚽S1"], &[]),
    ),
    (
        "💖U0",
        (
            &["💖U1", "💖U2", "💖U3"],
            &[
                "🌱P0", "🌱P1", "🌱P2", "🌱P3", "🌱Q0", "🌱Q1", "🌱Q2", "🌱Q3", "🌱R0", "🌱R1",
                "🌱R2", "🌱R3", "🧩Q0", "🧩Q1", "🧩Q2", "🧩Q3",
            ],
        ),
    ),
    (
        "💖U1",
        (
            &["💖U2", "💖U4", "💖U5", "💖U6", "💖U0", "😎R8", "😎P3"],
            &[
                "🌱P0", "🌱P1", "🌱P2", "🌱P3", "🌱Q0", "🌱Q1", "🌱Q2", "🌱Q3", "🌱R0", "🌱R1",
                "🌱R2", "🌱R3", "🧩Q0", "🧩Q1", "🧩Q2", "🧩Q3",
            ],
        ),
    ),
    (
        "💖U2",
        (
            &["💖U1", "💖U3", "💖U4", "💖U0"],
            &[
                "🌱P0", "🌱P1", "🌱P2", "🌱P3", "🌱Q0", "🌱Q1", "🌱Q2", "🌱Q3", "🌱R0", "🌱R1",
                "🌱R2", "🌱R3", "🧩Q0", "🧩Q1", "🧩Q2", "🧩Q3",
            ],
        ),
    ),
    (
        "💖U3",
        (
            &["💖U2", "💖U4", "💖U0", "💖T7", "💖T8", "⚽S1"],
            &[
                "🌱P0", "🌱P1", "🌱P2", "🌱P3", "🌱Q0", "🌱Q1", "🌱Q2", "🌱Q3", "🌱R0", "🌱R1",
                "🌱R2", "🌱R3", "🧩Q0", "🧩Q1", "🧩Q2", "🧩Q3",
            ],
        ),
    ),
    (
        "💖U4",
        (&["💖U1", "💖U2", "💖U3", "💖U5", "💖U7", "💖T7"], &[]),
    ),
    ("💖U5", (&["💖U1", "💖U4", "💖U6", "💖U7"], &[])),
    (
        "💖U6",
        (&["💖U8", "💖U1", "💖U5", "💖U7", "😎R6", "😎R8"], &[]),
    ),
    (
        "💖U7",
        (
            &["💖U8", "💖U4", "💖U5", "💖U6", "💖T7", "💖S4", "💖S3"],
            &[],
        ),
    ),
    ("💖U8", (&["💖U6", "💖U7", "💖S3", "😎R1", "😎R6"], &[])),
    (
        "😎P0",
        (
            &["😎P1", "😎P2", "😎P3"],
            &[
                "⚽S0", "⚽S1", "⚽S2", "⚽S3", "⚽T0", "⚽T1", "⚽T2", "⚽T3", "⚽U0", "⚽U1",
                "⚽U2", "⚽U3",
            ],
        ),
    ),
    (
        "😎P1",
        (
            &["😎P2", "😎P4", "😎P5", "😎P6", "😎P0", "🌱S8", "🌱R3"],
            &[
                "⚽S0", "⚽S1", "⚽S2", "⚽S3", "⚽T0", "⚽T1", "⚽T2", "⚽T3", "⚽U0", "⚽U1",
                "⚽U2", "⚽U3",
            ],
        ),
    ),
    (
        "😎P2",
        (
            &["😎P1", "😎P3", "😎P4", "😎P0"],
            &[
                "⚽S0", "⚽S1", "⚽S2", "⚽S3", "⚽T0", "⚽T1", "⚽T2", "⚽T3", "⚽U0", "⚽U1",
                "⚽U2", "⚽U3",
            ],
        ),
    ),
    (
        "😎P3",
        (
            &["😎P2", "😎P4", "😎P0", "😎R7", "😎R8", "💖U1"],
            &[
                "⚽S0", "⚽S1", "⚽S2", "⚽S3", "⚽T0", "⚽T1", "⚽T2", "⚽T3", "⚽U0", "⚽U1",
                "⚽U2", "⚽U3",
            ],
        ),
    ),
    (
        "😎P4",
        (
            &["😎P1", "😎P2", "😎P3", "😎P5", "😎P7", "😎R4", "😎R7"],
            &[],
        ),
    ),
    ("😎P5", (&["😎P1", "😎P4", "😎P6", "😎P7"], &[])),
    (
        "😎P6",
        (&["😎P8", "😎P1", "😎P5", "😎P7", "🌱S8", "🌱S6"], &[]),
    ),
    (
        "😎P7",
        (
            &["😎P8", "😎P4", "😎P5", "😎P6", "😎Q7", "😎Q4", "😎R4"],
            &[],
        ),
    ),
    (
        "😎P8",
        (&["😎P6", "😎P7", "😎Q4", "😎Q3", "🌱S6", "🌱S1"], &[]),
    ),
    (
        "😎Q0",
        (
            &["😎Q1", "😎Q2", "😎Q3"],
            &[
                "🐀P0", "🐀P1", "🐀P2", "🐀P3", "🐀Q0", "🐀Q1", "🐀Q2", "🐀Q3", "🐀R0", "🐀R1",
                "🐀R2", "🐀R3", "🌵Q0", "🌵Q1", "🌵Q2", "🌵Q3",
            ],
        ),
    ),
    (
        "😎Q1",
        (
            &["😎Q2", "😎Q5", "😎Q6", "😎Q0", "😎S6", "😎S8", "😎T3"],
            &[
                "🐀P0", "🐀P1", "🐀P2", "🐀P3", "🐀Q0", "🐀Q1", "🐀Q2", "🐀Q3", "🐀R0", "🐀R1",
                "🐀R2", "🐀R3", "🌵Q0", "🌵Q1", "🌵Q2", "🌵Q3",
            ],
        ),
    ),
    (
        "😎Q2",
        (
            &["😎Q1", "😎Q3", "😎Q4", "😎Q5", "😎Q0"],
            &[
                "🐀P0", "🐀P1", "🐀P2", "🐀P3", "🐀Q0", "🐀Q1", "🐀Q2", "🐀Q3", "🐀R0", "🐀R1",
                "🐀R2", "🐀R3", "🌵Q0", "🌵Q1", "🌵Q2", "🌵Q3",
            ],
        ),
    ),
    (
        "😎Q3",
        (
            &["😎Q2", "😎Q4", "😎Q0", "😎P8", "🌱S1"],
            &[
                "🐀P0", "🐀P1", "🐀P2", "🐀P3", "🐀Q0", "🐀Q1", "🐀Q2", "🐀Q3", "🐀R0", "🐀R1",
                "🐀R2", "🐀R3", "🌵Q0", "🌵Q1", "🌵Q2", "🌵Q3",
            ],
        ),
    ),
    (
        "😎Q4",
        (
            &["😎Q2", "😎Q3", "😎Q5", "😎Q7", "😎Q8", "😎P7", "😎P8"],
            &[],
        ),
    ),
    ("😎Q5", (&["😎Q1", "😎Q2", "😎Q4", "😎Q6", "😎Q8"], &[])),
    ("😎Q6", (&["😎Q8", "😎Q1", "😎Q5", "😎S1", "😎S6"], &[])),
    ("😎Q7", (&["😎Q8", "😎Q4", "😎P7", "😎R4"], &[])),
    (
        "😎Q8",
        (
            &["😎Q4", "😎Q5", "😎Q6", "😎Q7", "😎R4", "😎R3", "😎S1"],
            &[],
        ),
    ),
    ("😎R0", (&["😎R1", "😎R2", "😎R3"], &[])),
    (
        "😎R1",
        (&["😎R2", "😎R5", "😎R6", "😎R0", "💖U8", "💖S3"], &[]),
    ),
    ("😎R2", (&["😎R1", "😎R3", "😎R4", "😎R5", "😎R0"], &[])),
    ("😎R3", (&["😎R2", "😎R4", "😎R0", "😎Q8", "😎S1"], &[])),
    (
        "😎R4",
        (
            &[
                "😎R2", "😎R3", "😎R5", "😎R7", "😎Q7", "😎Q8", "😎P7", "😎P4",
            ],
            &[],
        ),
    ),
    ("😎R5", (&["😎R1", "😎R2", "😎R4", "😎R6", "😎R7"], &[])),
    (
        "😎R6",
        (&["😎R8", "😎R1", "😎R5", "😎R7", "💖U8", "💖U6"], &[]),
    ),
    (
        "😎R7",
        (&["😎R8", "😎R4", "😎R5", "😎R6", "😎P4", "😎P3"], &[]),
    ),
    ("😎R8", (&["😎R6", "😎R7", "😎P3", "💖U6", "💖U1"], &[])),
    ("😎S0", (&["😎S1", "😎S2", "😎S3"], &[])),
    (
        "😎S1",
        (&["😎S2", "😎S6", "😎S0", "😎Q8", "😎Q6", "😎R3"], &[]),
    ),
    (
        "😎S2",
        (&["😎S1", "😎S3", "😎S4", "😎S5", "😎S6", "😎S0"], &[]),
    ),
    (
        "😎S3",
        (&["😎S2", "😎S4", "😎S0", "😎U7", "😎U8", "🏝️R1"], &[]),
    ),
    (
        "😎S4",
        (&["😎S2", "😎S3", "😎S5", "😎S7", "😎U4", "😎U7"], &[]),
    ),
    ("😎S5", (&["😎S2", "😎S4", "😎S6", "😎S7", "😎S8"], &[])),
    (
        "😎S6",
        (&["😎S8", "😎S1", "😎S2", "😎S5", "😎Q6", "😎Q1"], &[]),
    ),
    (
        "😎S7",
        (&["😎S8", "😎S4", "😎S5", "😎T6", "😎T4", "😎U4"], &[]),
    ),
    (
        "😎S8",
        (&["😎S5", "😎S6", "😎S7", "😎T4", "😎T3", "😎Q1"], &[]),
    ),
    (
        "😎T0",
        (
            &["😎T1", "😎T2", "😎T3"],
            &[
                "🌱S0", "🌱S1", "🌱S2", "🌱S3", "🌱T0", "🌱T1", "🌱T2", "🌱T3", "🌱U0", "🌱U1",
                "🌱U2", "🌱U3",
            ],
        ),
    ),
    (
        "😎T1",
        (
            &["😎T2", "😎T5", "😎T7", "😎T0", "🐀S8", "🐀R3"],
            &[
                "🌱S0", "🌱S1", "🌱S2", "🌱S3", "🌱T0", "🌱T1", "🌱T2", "🌱T3", "🌱U0", "🌱U1",
                "🌱U2", "🌱U3",
            ],
        ),
    ),
    (
        "😎T2",
        (
            &["😎T1", "😎T3", "😎T5", "😎T0"],
            &[
                "🌱S0", "🌱S1", "🌱S2", "🌱S3", "🌱T0", "🌱T1", "🌱T2", "🌱T3", "🌱U0", "🌱U1",
                "🌱U2", "🌱U3",
            ],
        ),
    ),
    (
        "😎T3",
        (
            &["😎T2", "😎T4", "😎T5", "😎T0", "😎S8", "😎Q1"],
            &[
                "🌱S0", "🌱S1", "🌱S2", "🌱S3", "🌱T0", "🌱T1", "🌱T2", "🌱T3", "🌱U0", "🌱U1",
                "🌱U2", "🌱U3",
            ],
        ),
    ),
    (
        "😎T4",
        (&["😎T3", "😎T5", "😎T6", "😎T7", "😎S7", "😎S8"], &[]),
    ),
    ("😎T5", (&["😎T1", "😎T2", "😎T3", "😎T4", "😎T7"], &[])),
    ("😎T6", (&["😎T8", "😎T4", "😎T7", "😎S7", "😎U4"], &[])),
    (
        "😎T7",
        (
            &["😎T1", "😎T4", "😎T5", "😎T6", "😎T8", "🐀S8", "🐀S6"],
            &[],
        ),
    ),
    (
        "😎T8",
        (&["😎T6", "😎T7", "😎U4", "😎U3", "🐀S6", "🐀S1"], &[]),
    ),
    (
        "😎U0",
        (
            &["😎U1", "😎U2", "😎U3"],
            &[
                "🐍P0", "🐍P1", "🐍P2", "🐍P3", "🐍Q0", "🐍Q1", "🐍Q2", "🐍Q3", "🐍R0", "🐍R1",
                "🐍R2", "🐍R3", "🧀Q0", "🧀Q1", "🧀Q2", "🧀Q3",
            ],
        ),
    ),
    (
        "😎U1",
        (
            &["😎U2", "😎U5", "😎U6", "😎U0", "🏝️R1", "🏝️R8", "🏝️P3"],
            &[
                "🐍P0", "🐍P1", "🐍P2", "🐍P3", "🐍Q0", "🐍Q1", "🐍Q2", "🐍Q3", "🐍R0", "🐍R1",
                "🐍R2", "🐍R3", "🧀Q0", "🧀Q1", "🧀Q2", "🧀Q3",
            ],
        ),
    ),
    (
        "😎U2",
        (
            &["😎U1", "😎U3", "😎U4", "😎U5", "😎U0"],
            &[
                "🐍P0", "🐍P1", "🐍P2", "🐍P3", "🐍Q0", "🐍Q1", "🐍Q2", "🐍Q3", "🐍R0", "🐍R1",
                "🐍R2", "🐍R3", "🧀Q0", "🧀Q1", "🧀Q2", "🧀Q3",
            ],
        ),
    ),
    (
        "😎U3",
        (
            &["😎U2", "😎U4", "😎U0", "😎T8", "🐀S1"],
            &[
                "🐍P0", "🐍P1", "🐍P2", "🐍P3", "🐍Q0", "🐍Q1", "🐍Q2", "🐍Q3", "🐍R0", "🐍R1",
                "🐍R2", "🐍R3", "🧀Q0", "🧀Q1", "🧀Q2", "🧀Q3",
            ],
        ),
    ),
    (
        "😎U4",
        (
            &[
                "😎U2", "😎U3", "😎U5", "😎U7", "😎T6", "😎T8", "😎S7", "😎S4",
            ],
            &[],
        ),
    ),
    ("😎U5", (&["😎U1", "😎U2", "😎U4", "😎U6", "😎U7"], &[])),
    ("😎U6", (&["😎U8", "😎U1", "😎U5", "😎U7", "🏝️R1"], &[])),
    (
        "😎U7",
        (&["😎U8", "😎U4", "😎U5", "😎U6", "😎S4", "😎S3"], &[]),
    ),
    ("😎U8", (&["😎U6", "😎U7", "😎S3", "🏝️R1"], &[])),
    (
        "🍌P0",
        (&["🍌P1", "🍌P2", "🍌P3"], &["🧀P0", "🧀P1", "🧀P2", "🧀P3"]),
    ),
    (
        "🍌P1",
        (
            &["🍌P2", "🍌P5", "🍌P6", "🍌P0"],
            &["🧀P0", "🧀P1", "🧀P2", "🧀P3"],
        ),
    ),
    (
        "🍌P2",
        (
            &["🍌P1", "🍌P3", "🍌P4", "🍌P5", "🍌P0"],
            &["🧀P0", "🧀P1", "🧀P2", "🧀P3"],
        ),
    ),
    (
        "🍌P3",
        (
            &["🍌P2", "🍌P4", "🍌P0", "🍌Q7", "🍌Q4", "🍌Q3", "🍌R8"],
            &["🧀P0", "🧀P1", "🧀P2", "🧀P3"],
        ),
    ),
    (
        "🍌P4",
        (&["🍌P2", "🍌P3", "🍌P5", "🍌P7", "🍌Q3", "🥯P1"], &[]),
    ),
    ("🍌P5", (&["🍌P1", "🍌P2", "🍌P4", "🍌P6", "🍌P7"], &[])),
    ("🍌P6", (&["🍌P8", "🍌P1", "🍌P5", "🍌P7"], &[])),
    (
        "🍌P7",
        (&["🍌P8", "🍌P4", "🍌P5", "🍌P6", "🥯P1", "🥯P6"], &[]),
    ),
    ("🍌P8", (&["🍌P6", "🍌P7", "🥯P6", "🥯P8", "🐺U3"], &[])),
    (
        "🍌Q0",
        (&["🍌Q1", "🍌Q2", "🍌Q3"], &["🐑U0", "🐑U1", "🐑U2", "🐑U3"]),
    ),
    (
        "🍌Q1",
        (
            &["🍌Q2", "🍌Q4", "🍌Q5", "🍌Q6", "🍌Q0"],
            &["🐑U0", "🐑U1", "🐑U2", "🐑U3"],
        ),
    ),
    (
        "🍌Q2",
        (
            &["🍌Q1", "🍌Q3", "🍌Q4", "🍌Q0"],
            &["🐑U0", "🐑U1", "🐑U2", "🐑U3"],
        ),
    ),
    (
        "🍌Q3",
        (
            &["🍌Q2", "🍌Q4", "🍌Q0", "🍌P3", "🍌P4", "🥯P1"],
            &["🐑U0", "🐑U1", "🐑U2", "🐑U3"],
        ),
    ),
    (
        "🍌Q4",
        (&["🍌Q1", "🍌Q2", "🍌Q3", "🍌Q5", "🍌Q7", "🍌P3"], &[]),
    ),
    ("🍌Q5", (&["🍌Q1", "🍌Q4", "🍌Q6", "🍌Q7"], &[])),
    ("🍌Q6", (&["🍌Q8", "🍌Q1", "🍌Q5", "🍌Q7"], &[])),
    (
        "🍌Q7",
        (
            &["🍌Q8", "🍌Q4", "🍌Q5", "🍌Q6", "🍌P3", "🍌R8", "🍌R7"],
            &[],
        ),
    ),
    (
        "🍌Q8",
        (&["🍌Q6", "🍌Q7", "🍌R7", "🍌R4", "🍌S8", "🎈U3"], &[]),
    ),
    (
        "🍌R0",
        (&["🍌R1", "🍌R2", "🍌R3"], &["🧀R0", "🧀R1", "🧀R2", "🧀R3"]),
    ),
    (
        "🍌R1",
        (
            &["🍌R8", "🍌R2", "🍌R6", "🍌R0"],
            &["🧀R0", "🧀R1", "🧀R2", "🧀R3"],
        ),
    ),
    (
        "🍌R2",
        (
            &["🍌R1", "🍌R3", "🍌R5", "🍌R6", "🍌R0"],
            &["🧀R0", "🧀R1", "🧀R2", "🧀R3"],
        ),
    ),
    (
        "🍌R3",
        (
            &["🍌R2", "🍌R4", "🍌R5", "🍌R0", "🍌S1", "🍌S6"],
            &["🧀R0", "🧀R1", "🧀R2", "🧀R3"],
        ),
    ),
    (
        "🍌R4",
        (
            &["🍌R3", "🍌R5", "🍌R7", "🍌Q8", "🍌S6", "🍌S8", "🎈U3"],
            &[],
        ),
    ),
    ("🍌R5", (&["🍌R2", "🍌R3", "🍌R4", "🍌R6", "🍌R7"], &[])),
    ("🍌R6", (&["🍌R1", "🍌R2", "🍌R5", "🍌R7", "🍌R8"], &[])),
    (
        "🍌R7",
        (&["🍌R8", "🍌R4", "🍌R5", "🍌R6", "🍌Q7", "🍌Q8"], &[]),
    ),
    ("🍌R8", (&["🍌R1", "🍌R6", "🍌R7", "🍌Q7", "🍌P3"], &[])),
    (
        "🍌S0",
        (&["🍌S1", "🍌S2", "🍌S3"], &["🧀S0", "🧀S1", "🧀S2", "🧀S3"]),
    ),
    (
        "🍌S1",
        (
            &["🍌S2", "🍌S4", "🍌S5", "🍌S6", "🍌S0", "🍌R3"],
            &["🧀S0", "🧀S1", "🧀S2", "🧀S3"],
        ),
    ),
    (
        "🍌S2",
        (
            &["🍌S1", "🍌S3", "🍌S4", "🍌S0"],
            &["🧀S0", "🧀S1", "🧀S2", "🧀S3"],
        ),
    ),
    (
        "🍌S3",
        (
            &["🍌S2", "🍌S4", "🍌S0", "🍌T1", "🍌T6"],
            &["🧀S0", "🧀S1", "🧀S2", "🧀S3"],
        ),
    ),
    (
        "🍌S4",
        (
            &["🍌S1", "🍌S2", "🍌S3", "🍌S5", "🍌S7", "🍌T6", "🍌T8"],
            &[],
        ),
    ),
    ("🍌S5", (&["🍌S1", "🍌S4", "🍌S6", "🍌S7"], &[])),
    (
        "🍌S6",
        (&["🍌S8", "🍌S1", "🍌S5", "🍌S7", "🍌R3", "🍌R4"], &[]),
    ),
    (
        "🍌S7",
        (
            &["🍌S8", "🍌S4", "🍌S5", "🍌S6", "🍌T8", "🎈U4", "🎈U7"],
            &[],
        ),
    ),
    (
        "🍌S8",
        (&["🍌S6", "🍌S7", "🍌R4", "🍌Q8", "🎈U3", "🎈U4"], &[]),
    ),
    (
        "🍌T0",
        (&["🍌T1", "🍌T2", "🍌T3"], &["🧀T0", "🧀T1", "🧀T2", "🧀T3"]),
    ),
    (
        "🍌T1",
        (
            &["🍌T2", "🍌T5", "🍌T6", "🍌T0", "🍌S3"],
            &["🧀T0", "🧀T1", "🧀T2", "🧀T3"],
        ),
    ),
    (
        "🍌T2",
        (
            &["🍌T1", "🍌T3", "🍌T4", "🍌T5", "🍌T0"],
            &["🧀T0", "🧀T1", "🧀T2", "🧀T3"],
        ),
    ),
    (
        "🍌T3",
        (
            &["🍌T2", "🍌T4", "🍌T0", "🍌U1"],
            &["🧀T0", "🧀T1", "🧀T2", "🧀T3"],
        ),
    ),
    (
        "🍌T4",
        (&["🍌T2", "🍌T3", "🍌T5", "🍌T7", "🍌T8", "🍌U1"], &[]),
    ),
    ("🍌T5", (&["🍌T1", "🍌T2", "🍌T4", "🍌T6", "🍌T8"], &[])),
    ("🍌T6", (&["🍌T8", "🍌T1", "🍌T5", "🍌S3", "🍌S4"], &[])),
    ("🍌T7", (&["🍌T8", "🍌T4", "🍌U1", "🍌U8"], &[])),
    (
        "🍌T8",
        (
            &["🍌T4", "🍌T5", "🍌T6", "🍌T7", "🍌S4", "🍌S7", "🎈U7"],
            &[],
        ),
    ),
    (
        "🍌U0",
        (&["🍌U1", "🍌U2", "🍌U3"], &["🧀U0", "🧀U1", "🧀U2", "🧀U3"]),
    ),
    (
        "🍌U1",
        (
            &[
                "🍌U8", "🍌U2", "🍌U5", "🍌U7", "🍌U0", "🍌T3", "🍌T4", "🍌T7",
            ],
            &["🧀U0", "🧀U1", "🧀U2", "🧀U3"],
        ),
    ),
    (
        "🍌U2",
        (
            &["🍌U1", "🍌U3", "🍌U4", "🍌U5", "🍌U0"],
            &["🧀U0", "🧀U1", "🧀U2", "🧀U3"],
        ),
    ),
    (
        "🍌U3",
        (
            &["🍌U2", "🍌U4", "🍌U0", "🎈P8", "📚P8"],
            &["🧀U0", "🧀U1", "🧀U2", "🧀U3"],
        ),
    ),
    (
        "🍌U4",
        (&["🍌U2", "🍌U3", "🍌U5", "🍌U6", "🎈P7", "🎈P8"], &[]),
    ),
    ("🍌U5", (&["🍌U1", "🍌U2", "🍌U4", "🍌U6", "🍌U7"], &[])),
    ("🍌U6", (&["🍌U4", "🍌U5", "🍌U7", "🎈P7", "🎈Q8"], &[])),
    ("🍌U7", (&["🍌U8", "🍌U1", "🍌U5", "🍌U6"], &[])),
    ("🍌U8", (&["🍌U1", "🍌U7", "🍌T7"], &[])),
    (
        "🐺P0",
        (&["🐺P1", "🐺P2", "🐺P3"], &["🌙P0", "🌙P1", "🌙P2", "🌙P3"]),
    ),
    (
        "🐺P1",
        (&["🐺P2", "🐺P6", "🐺P0"], &["🌙P0", "🌙P1", "🌙P2", "🌙P3"]),
    ),
    (
        "🐺P2",
        (
            &["🐺P1", "🐺P3", "🐺P4", "🐺P5", "🐺P6", "🐺P0"],
            &["🌙P0", "🌙P1", "🌙P2", "🌙P3"],
        ),
    ),
    (
        "🐺P3",
        (
            &["🐺P2", "🐺P4", "🐺P0", "🐺Q7", "🐺Q4", "🐺Q3", "🐺R8"],
            &["🌙P0", "🌙P1", "🌙P2", "🌙P3"],
        ),
    ),
    ("🐺P4", (&["🐺P2", "🐺P3", "🐺P5", "🐺P7", "🐺Q3"], &[])),
    ("🐺P5", (&["🐺P2", "🐺P4", "🐺P6", "🐺P7", "🐺P8"], &[])),
    ("🐺P6", (&["🐺P8", "🐺P1", "🐺P2", "🐺P5"], &[])),
    ("🐺P7", (&["🐺P8", "🐺P4", "🐺P5", "🐺Q3", "🤠P1"], &[])),
    (
        "🐺P8",
        (&["🐺P5", "🐺P6", "🐺P7", "🤠P1", "🤠P8", "🍕U3"], &[]),
    ),
    (
        "🐺Q0",
        (&["🐺Q1", "🐺Q2", "🐺Q3"], &["⚡Q0", "⚡Q1", "⚡Q2", "⚡Q3"]),
    ),
    (
        "🐺Q1",
        (
            &["🐺Q2", "🐺Q4", "🐺Q5", "🐺Q6", "🐺Q0"],
            &["⚡Q0", "⚡Q1", "⚡Q2", "⚡Q3"],
        ),
    ),
    (
        "🐺Q2",
        (
            &["🐺Q1", "🐺Q3", "🐺Q4", "🐺Q0"],
            &["⚡Q0", "⚡Q1", "⚡Q2", "⚡Q3"],
        ),
    ),
    (
        "🐺Q3",
        (
            &["🐺Q2", "🐺Q4", "🐺Q0", "🐺P3", "🐺P4", "🐺P7", "🤠P1"],
            &["⚡Q0", "⚡Q1", "⚡Q2", "⚡Q3"],
        ),
    ),
    (
        "🐺Q4",
        (&["🐺Q1", "🐺Q2", "🐺Q3", "🐺Q5", "🐺Q7", "🐺P3"], &[]),
    ),
    ("🐺Q5", (&["🐺Q1", "🐺Q4", "🐺Q6", "🐺Q7"], &[])),
    ("🐺Q6", (&["🐺Q8", "🐺Q1", "🐺Q5", "🐺Q7"], &[])),
    (
        "🐺Q7",
        (&["🐺Q8", "🐺Q4", "🐺Q5", "🐺Q6", "🐺P3", "🐺R8"], &[]),
    ),
    (
        "🐺Q8",
        (&["🐺Q6", "🐺Q7", "🐺R8", "🐺R6", "🐺S8", "🥯U3"], &[]),
    ),
    (
        "🐺R0",
        (&["🐺R1", "🐺R2", "🐺R3"], &["🌙R0", "🌙R1", "🌙R2", "🌙R3"]),
    ),
    (
        "🐺R1",
        (
            &["🐺R2", "🐺R5", "🐺R7", "🐺R0"],
            &["🌙R0", "🌙R1", "🌙R2", "🌙R3"],
        ),
    ),
    (
        "🐺R2",
        (
            &["🐺R1", "🐺R3", "🐺R5", "🐺R0"],
            &["🌙R0", "🌙R1", "🌙R2", "🌙R3"],
        ),
    ),
    (
        "🐺R3",
        (
            &["🐺R2", "🐺R4", "🐺R5", "🐺R0", "🐺S1"],
            &["🌙R0", "🌙R1", "🌙R2", "🌙R3"],
        ),
    ),
    (
        "🐺R4",
        (&["🐺R3", "🐺R5", "🐺R6", "🐺R7", "🐺S1", "🐺S8"], &[]),
    ),
    ("🐺R5", (&["🐺R1", "🐺R2", "🐺R3", "🐺R4", "🐺R7"], &[])),
    (
        "🐺R6",
        (&["🐺R8", "🐺R4", "🐺R7", "🐺Q8", "🐺S8", "🥯U3"], &[]),
    ),
    ("🐺R7", (&["🐺R1", "🐺R4", "🐺R5", "🐺R6", "🐺R8"], &[])),
    ("🐺R8", (&["🐺R6", "🐺R7", "🐺Q7", "🐺Q8", "🐺P3"], &[])),
    (
        "🐺S0",
        (&["🐺S1", "🐺S2", "🐺S3"], &["🌙S0", "🌙S1", "🌙S2", "🌙S3"]),
    ),
    (
        "🐺S1",
        (
            &["🐺S8", "🐺S2", "🐺S5", "🐺S7", "🐺S0", "🐺R3", "🐺R4"],
            &["🌙S0", "🌙S1", "🌙S2", "🌙S3"],
        ),
    ),
    (
        "🐺S2",
        (
            &["🐺S1", "🐺S3", "🐺S4", "🐺S5", "🐺S0"],
            &["🌙S0", "🌙S1", "🌙S2", "🌙S3"],
        ),
    ),
    (
        "🐺S3",
        (
            &["🐺S2", "🐺S4", "🐺S0", "🐺T1"],
            &["🌙S0", "🌙S1", "🌙S2", "🌙S3"],
        ),
    ),
    (
        "🐺S4",
        (&["🐺S2", "🐺S3", "🐺S5", "🐺S6", "🐺T1", "🐺T6"], &[]),
    ),
    ("🐺S5", (&["🐺S1", "🐺S2", "🐺S4", "🐺S6", "🐺S7"], &[])),
    (
        "🐺S6",
        (
            &["🐺S4", "🐺S5", "🐺S7", "🐺T6", "🐺T8", "🥯U4", "🥯U7"],
            &[],
        ),
    ),
    (
        "🐺S7",
        (&["🐺S8", "🐺S1", "🐺S5", "🐺S6", "🥯U3", "🥯U4"], &[]),
    ),
    (
        "🐺S8",
        (&["🐺S1", "🐺S7", "🐺R4", "🐺R6", "🐺Q8", "🥯U3"], &[]),
    ),
    (
        "🐺T0",
        (&["🐺T1", "🐺T2", "🐺T3"], &["🌙T0", "🌙T1", "🌙T2", "🌙T3"]),
    ),
    (
        "🐺T1",
        (
            &["🐺T2", "🐺T5", "🐺T6", "🐺T0", "🐺S3", "🐺S4"],
            &["🌙T0", "🌙T1", "🌙T2", "🌙T3"],
        ),
    ),
    (
        "🐺T2",
        (
            &["🐺T1", "🐺T3", "🐺T4", "🐺T5", "🐺T0"],
            &["🌙T0", "🌙T1", "🌙T2", "🌙T3"],
        ),
    ),
    (
        "🐺T3",
        (
            &["🐺T2", "🐺T4", "🐺T0", "🐺U1"],
            &["🌙T0", "🌙T1", "🌙T2", "🌙T3"],
        ),
    ),
    (
        "🐺T4",
        (&["🐺T2", "🐺T3", "🐺T5", "🐺T7", "🐺T8", "🐺U1"], &[]),
    ),
    ("🐺T5", (&["🐺T1", "🐺T2", "🐺T4", "🐺T6", "🐺T8"], &[])),
    ("🐺T6", (&["🐺T8", "🐺T1", "🐺T5", "🐺S4", "🐺S6"], &[])),
    ("🐺T7", (&["🐺T8", "🐺T4", "🐺U1", "🐺U8"], &[])),
    (
        "🐺T8",
        (&["🐺T4", "🐺T5", "🐺T6", "🐺T7", "🐺S6", "🥯U7"], &[]),
    ),
    (
        "🐺U0",
        (&["🐺U1", "🐺U2", "🐺U3"], &["🌙U0", "🌙U1", "🌙U2", "🌙U3"]),
    ),
    (
        "🐺U1",
        (
            &[
                "🐺U8", "🐺U2", "🐺U5", "🐺U7", "🐺U0", "🐺T3", "🐺T4", "🐺T7",
            ],
            &["🌙U0", "🌙U1", "🌙U2", "🌙U3"],
        ),
    ),
    (
        "🐺U2",
        (
            &["🐺U1", "🐺U3", "🐺U4", "🐺U5", "🐺U0"],
            &["🌙U0", "🌙U1", "🌙U2", "🌙U3"],
        ),
    ),
    (
        "🐺U3",
        (
            &["🐺U2", "🐺U4", "🐺U0", "🥯P8", "🍌P8"],
            &["🌙U0", "🌙U1", "🌙U2", "🌙U3"],
        ),
    ),
    (
        "🐺U4",
        (&["🐺U2", "🐺U3", "🐺U5", "🐺U6", "🥯P7", "🥯P8"], &[]),
    ),
    ("🐺U5", (&["🐺U1", "🐺U2", "🐺U4", "🐺U6", "🐺U7"], &[])),
    ("🐺U6", (&["🐺U4", "🐺U5", "🐺U7", "🥯P7", "🥯Q8"], &[])),
    ("🐺U7", (&["🐺U8", "🐺U1", "🐺U5", "🐺U6"], &[])),
    ("🐺U8", (&["🐺U1", "🐺U7", "🐺T7"], &[])),
    (
        "🍕P0",
        (&["🍕P1", "🍕P2", "🍕P3"], &["🦋P0", "🦋P1", "🦋P2", "🦋P3"]),
    ),
    (
        "🍕P1",
        (
            &["🍕P2", "🍕P5", "🍕P6", "🍕P0"],
            &["🦋P0", "🦋P1", "🦋P2", "🦋P3"],
        ),
    ),
    (
        "🍕P2",
        (
            &["🍕P1", "🍕P3", "🍕P4", "🍕P5", "🍕P0"],
            &["🦋P0", "🦋P1", "🦋P2", "🦋P3"],
        ),
    ),
    (
        "🍕P3",
        (
            &["🍕P2", "🍕P4", "🍕P0", "🍕Q7", "🍕Q4", "🍕R8"],
            &["🦋P0", "🦋P1", "🦋P2", "🦋P3"],
        ),
    ),
    (
        "🍕P4",
        (
            &["🍕P2", "🍕P3", "🍕P5", "🍕P7", "🍕P8", "🍕Q4", "🍕Q3"],
            &[],
        ),
    ),
    ("🍕P5", (&["🍕P1", "🍕P2", "🍕P4", "🍕P6", "🍕P8"], &[])),
    ("🍕P6", (&["🍕P8", "🍕P1", "🍕P5"], &[])),
    ("🍕P7", (&["🍕P8", "🍕P4", "🍕Q3"], &[])),
    ("🍕P8", (&["🍕P4", "🍕P5", "🍕P6", "🍕P7", "🍔U3"], &[])),
    ("🍕Q0", (&["🍕Q1", "🍕Q2", "🍕Q3"], &["⚡U0"])),
    ("🍕Q1", (&["🍕Q2", "🍕Q6", "🍕Q0"], &["⚡U0"])),
    (
        "🍕Q2",
        (&["🍕Q1", "🍕Q3", "🍕Q4", "🍕Q5", "🍕Q6", "🍕Q0"], &["⚡U0"]),
    ),
    (
        "🍕Q3",
        (&["🍕Q2", "🍕Q4", "🍕Q0", "🍕P4", "🍕P7"], &["⚡U0"]),
    ),
    (
        "🍕Q4",
        (&["🍕Q2", "🍕Q3", "🍕Q5", "🍕Q7", "🍕P3", "🍕P4"], &[]),
    ),
    ("🍕Q5", (&["🍕Q2", "🍕Q4", "🍕Q6", "🍕Q7", "🍕Q8"], &[])),
    ("🍕Q6", (&["🍕Q8", "🍕Q1", "🍕Q2", "🍕Q5"], &[])),
    ("🍕Q7", (&["🍕Q8", "🍕Q4", "🍕Q5", "🍕P3", "🍕R8"], &[])),
    (
        "🍕Q8",
        (&["🍕Q5", "🍕Q6", "🍕Q7", "🍕R8", "🍕R6", "🍕S8"], &[]),
    ),
    (
        "🍕R0",
        (&["🍕R1", "🍕R2", "🍕R3"], &["🦋R0", "🦋R1", "🦋R2", "🦋R3"]),
    ),
    (
        "🍕R1",
        (
            &["🍕R2", "🍕R5", "🍕R7", "🍕R0"],
            &["🦋R0", "🦋R1", "🦋R2", "🦋R3"],
        ),
    ),
    (
        "🍕R2",
        (
            &["🍕R1", "🍕R3", "🍕R5", "🍕R0"],
            &["🦋R0", "🦋R1", "🦋R2", "🦋R3"],
        ),
    ),
    (
        "🍕R3",
        (
            &["🍕R2", "🍕R4", "🍕R5", "🍕R0", "🍕S1", "🍕S6"],
            &["🦋R0", "🦋R1", "🦋R2", "🦋R3"],
        ),
    ),
    (
        "🍕R4",
        (&["🍕R3", "🍕R5", "🍕R6", "🍕R7", "🍕S6", "🍕S8"], &[]),
    ),
    ("🍕R5", (&["🍕R1", "🍕R2", "🍕R3", "🍕R4", "🍕R7"], &[])),
    ("🍕R6", (&["🍕R8", "🍕R4", "🍕R7", "🍕Q8", "🍕S8"], &[])),
    ("🍕R7", (&["🍕R1", "🍕R4", "🍕R5", "🍕R6", "🍕R8"], &[])),
    ("🍕R8", (&["🍕R6", "🍕R7", "🍕Q7", "🍕Q8", "🍕P3"], &[])),
    (
        "🍕S0",
        (&["🍕S1", "🍕S2", "🍕S3"], &["🦋S0", "🦋S1", "🦋S2", "🦋S3"]),
    ),
    (
        "🍕S1",
        (
            &["🍕S2", "🍕S5", "🍕S6", "🍕S0", "🍕R3"],
            &["🦋S0", "🦋S1", "🦋S2", "🦋S3"],
        ),
    ),
    (
        "🍕S2",
        (
            &["🍕S1", "🍕S3", "🍕S4", "🍕S5", "🍕S0"],
            &["🦋S0", "🦋S1", "🦋S2", "🦋S3"],
        ),
    ),
    (
        "🍕S3",
        (
            &["🍕S2", "🍕S4", "🍕S0", "🍕T1"],
            &["🦋S0", "🦋S1", "🦋S2", "🦋S3"],
        ),
    ),
    (
        "🍕S4",
        (&["🍕S2", "🍕S3", "🍕S5", "🍕S7", "🍕S8", "🍕T1"], &[]),
    ),
    ("🍕S5", (&["🍕S1", "🍕S2", "🍕S4", "🍕S6", "🍕S8"], &[])),
    ("🍕S6", (&["🍕S8", "🍕S1", "🍕S5", "🍕R3", "🍕R4"], &[])),
    ("🍕S7", (&["🍕S8", "🍕S4", "🍕T1", "🍕T8"], &[])),
    (
        "🍕S8",
        (
            &["🍕S4", "🍕S5", "🍕S6", "🍕S7", "🍕R4", "🍕R6", "🍕Q8"],
            &[],
        ),
    ),
    (
        "🍕T0",
        (&["🍕T1", "🍕T2", "🍕T3"], &["🦋T0", "🦋T1", "🦋T2", "🦋T3"]),
    ),
    (
        "🍕T1",
        (
            &[
                "🍕T8", "🍕T2", "🍕T5", "🍕T7", "🍕T0", "🍕S3", "🍕S4", "🍕S7",
            ],
            &["🦋T0", "🦋T1", "🦋T2", "🦋T3"],
        ),
    ),
    (
        "🍕T2",
        (
            &["🍕T1", "🍕T3", "🍕T4", "🍕T5", "🍕T0"],
            &["🦋T0", "🦋T1", "🦋T2", "🦋T3"],
        ),
    ),
    (
        "🍕T3",
        (
            &["🍕T2", "🍕T4", "🍕T0", "🍕U1"],
            &["🦋T0", "🦋T1", "🦋T2", "🦋T3"],
        ),
    ),
    (
        "🍕T4",
        (
            &["🍕T2", "🍕T3", "🍕T5", "🍕T6", "🍕U1", "🍕U6", "🍕U8"],
            &[],
        ),
    ),
    ("🍕T5", (&["🍕T1", "🍕T2", "🍕T4", "🍕T6", "🍕T7"], &[])),
    ("🍕T6", (&["🍕T4", "🍕T5", "🍕T7", "🍕U8"], &[])),
    ("🍕T7", (&["🍕T8", "🍕T1", "🍕T5", "🍕T6"], &[])),
    ("🍕T8", (&["🍕T1", "🍕T7", "🍕S7"], &[])),
    ("🍕U0", (&["🍕U1", "🍕U2", "🍕U3"], &[])),
    ("🍕U1", (&["🍕U2", "🍕U6", "🍕U0", "🍕T3", "🍕T4"], &[])),
    (
        "🍕U2",
        (&["🍕U1", "🍕U3", "🍕U4", "🍕U5", "🍕U6", "🍕U0"], &[]),
    ),
    (
        "🍕U3",
        (&["🍕U2", "🍕U4", "🍕U0", "🤠P7", "🤠P8", "🐺P8"], &[]),
    ),
    (
        "🍕U4",
        (&["🍕U2", "🍕U3", "🍕U5", "🍕U7", "🤠P4", "🤠P7"], &[]),
    ),
    ("🍕U5", (&["🍕U2", "🍕U4", "🍕U6", "🍕U7", "🍕U8"], &[])),
    ("🍕U6", (&["🍕U8", "🍕U1", "🍕U2", "🍕U5", "🍕T4"], &[])),
    ("🍕U7", (&["🍕U8", "🍕U4", "🍕U5", "🤠P4", "🤠Q8"], &[])),
    ("🍕U8", (&["🍕U5", "🍕U6", "🍕U7", "🍕T4", "🍕T6"], &[])),
    (
        "🍔P0",
        (&["🍔P1", "🍔P2", "🍔P3"], &["👻P0", "👻P1", "👻P2", "👻P3"]),
    ),
    (
        "🍔P1",
        (
            &["🍔P2", "🍔P4", "🍔P5", "🍔P6", "🍔P0"],
            &["👻P0", "👻P1", "👻P2", "👻P3"],
        ),
    ),
    (
        "🍔P2",
        (
            &["🍔P1", "🍔P3", "🍔P4", "🍔P0"],
            &["👻P0", "👻P1", "👻P2", "👻P3"],
        ),
    ),
    (
        "🍔P3",
        (
            &["🍔P2", "🍔P4", "🍔P0", "🍔Q4", "🍔R8"],
            &["👻P0", "👻P1", "👻P2", "👻P3"],
        ),
    ),
    (
        "🍔P4",
        (&["🍔P1", "🍔P2", "🍔P3", "🍔P5", "🍔Q4", "🍔Q3"], &[]),
    ),
    (
        "🍔P5",
        (&["🍔P1", "🍔P4", "🍔P6", "🍔P7", "🍔P8", "🍔Q3"], &[]),
    ),
    ("🍔P6", (&["🍔P8", "🍔P1", "🍔P5"], &[])),
    ("🍔P7", (&["🍔P8", "🍔P5", "🍔Q3", "🐉P1", "🐉P8"], &[])),
    ("🍔P8", (&["🍔P5", "🍔P6", "🍔P7", "🐉P8", "🐦‍🔥U3"], &[])),
    ("🍔Q0", (&["🍔Q1", "🍔Q2", "🍔Q3"], &["🤖Q0"])),
    ("🍔Q1", (&["🍔Q2", "🍔Q5", "🍔Q6", "🍔Q0"], &["🤖Q0"])),
    (
        "🍔Q2",
        (&["🍔Q1", "🍔Q3", "🍔Q4", "🍔Q5", "🍔Q0"], &["🤖Q0"]),
    ),
    (
        "🍔Q3",
        (
            &["🍔Q2", "🍔Q4", "🍔Q0", "🍔P4", "🍔P5", "🍔P7", "🐉P1"],
            &["🤖Q0"],
        ),
    ),
    (
        "🍔Q4",
        (
            &["🍔Q2", "🍔Q3", "🍔Q5", "🍔Q7", "🍔P3", "🍔P4", "🍔R8"],
            &[],
        ),
    ),
    ("🍔Q5", (&["🍔Q1", "🍔Q2", "🍔Q4", "🍔Q6", "🍔Q7"], &[])),
    ("🍔Q6", (&["🍔Q8", "🍔Q1", "🍔Q5", "🍔Q7"], &[])),
    ("🍔Q7", (&["🍔Q8", "🍔Q4", "🍔Q5", "🍔Q6", "🍔R8"], &[])),
    ("🍔Q8", (&["🍔Q6", "🍔Q7", "🍔R8", "🍔R6", "🍔S8"], &[])),
    (
        "🍔R0",
        (&["🍔R1", "🍔R2", "🍔R3"], &["👻R0", "👻R1", "👻R2", "👻R3"]),
    ),
    (
        "🍔R1",
        (
            &["🍔R2", "🍔R5", "🍔R7", "🍔R0"],
            &["👻R0", "👻R1", "👻R2", "👻R3"],
        ),
    ),
    (
        "🍔R2",
        (
            &["🍔R1", "🍔R3", "🍔R5", "🍔R0"],
            &["👻R0", "👻R1", "👻R2", "👻R3"],
        ),
    ),
    (
        "🍔R3",
        (
            &["🍔R2", "🍔R4", "🍔R5", "🍔R0", "🍔S1", "🍔S6"],
            &["👻R0", "👻R1", "👻R2", "👻R3"],
        ),
    ),
    (
        "🍔R4",
        (&["🍔R3", "🍔R5", "🍔R6", "🍔R7", "🍔S6", "🍔S8"], &[]),
    ),
    ("🍔R5", (&["🍔R1", "🍔R2", "🍔R3", "🍔R4", "🍔R7"], &[])),
    ("🍔R6", (&["🍔R8", "🍔R4", "🍔R7", "🍔Q8", "🍔S8"], &[])),
    ("🍔R7", (&["🍔R1", "🍔R4", "🍔R5", "🍔R6", "🍔R8"], &[])),
    (
        "🍔R8",
        (&["🍔R6", "🍔R7", "🍔Q4", "🍔Q7", "🍔Q8", "🍔P3"], &[]),
    ),
    (
        "🍔S0",
        (&["🍔S1", "🍔S2", "🍔S3"], &["👻S0", "👻S1", "👻S2", "👻S3"]),
    ),
    (
        "🍔S1",
        (
            &["🍔S2", "🍔S6", "🍔S0", "🍔R3"],
            &["👻S0", "👻S1", "👻S2", "👻S3"],
        ),
    ),
    (
        "🍔S2",
        (
            &["🍔S1", "🍔S3", "🍔S4", "🍔S5", "🍔S6", "🍔S0"],
            &["👻S0", "👻S1", "👻S2", "👻S3"],
        ),
    ),
    (
        "🍔S3",
        (
            &["🍔S2", "🍔S4", "🍔S0", "🍔T1", "🍔T6"],
            &["👻S0", "👻S1", "👻S2", "👻S3"],
        ),
    ),
    (
        "🍔S4",
        (&["🍔S2", "🍔S3", "🍔S5", "🍔S7", "🍔T6", "🍔T8"], &[]),
    ),
    ("🍔S5", (&["🍔S2", "🍔S4", "🍔S6", "🍔S7", "🍔S8"], &[])),
    (
        "🍔S6",
        (&["🍔S8", "🍔S1", "🍔S2", "🍔S5", "🍔R3", "🍔R4"], &[]),
    ),
    ("🍔S7", (&["🍔S8", "🍔S4", "🍔S5", "🍔T8"], &[])),
    (
        "🍔S8",
        (&["🍔S5", "🍔S6", "🍔S7", "🍔R4", "🍔R6", "🍔Q8"], &[]),
    ),
    (
        "🍔T0",
        (&["🍔T1", "🍔T2", "🍔T3"], &["👻T0", "👻T1", "👻T2", "👻T3"]),
    ),
    (
        "🍔T1",
        (
            &["🍔T2", "🍔T6", "🍔T0", "🍔S3"],
            &["👻T0", "👻T1", "👻T2", "👻T3"],
        ),
    ),
    (
        "🍔T2",
        (
            &["🍔T1", "🍔T3", "🍔T4", "🍔T5", "🍔T6", "🍔T0"],
            &["👻T0", "👻T1", "👻T2", "👻T3"],
        ),
    ),
    (
        "🍔T3",
        (
            &["🍔T2", "🍔T4", "🍔T0", "🍔U1", "🍔U7"],
            &["👻T0", "👻T1", "👻T2", "👻T3"],
        ),
    ),
    (
        "🍔T4",
        (&["🍔T2", "🍔T3", "🍔T5", "🍔T7", "🍔U7", "🍔U8"], &[]),
    ),
    ("🍔T5", (&["🍔T2", "🍔T4", "🍔T6", "🍔T7", "🍔T8"], &[])),
    (
        "🍔T6",
        (&["🍔T8", "🍔T1", "🍔T2", "🍔T5", "🍔S3", "🍔S4"], &[]),
    ),
    ("🍔T7", (&["🍔T8", "🍔T4", "🍔T5", "🍔U8"], &[])),
    ("🍔T8", (&["🍔T5", "🍔T6", "🍔T7", "🍔S4", "🍔S7"], &[])),
    (
        "🍔U0",
        (&["🍔U1", "🍔U2", "🍔U3"], &["👻U0", "👻U1", "👻U2", "👻U3"]),
    ),
    (
        "🍔U1",
        (
            &["🍔U2", "🍔U5", "🍔U7", "🍔U0", "🍔T3"],
            &["👻U0", "👻U1", "👻U2", "👻U3"],
        ),
    ),
    (
        "🍔U2",
        (
            &["🍔U1", "🍔U3", "🍔U5", "🍔U0"],
            &["👻U0", "👻U1", "👻U2", "👻U3"],
        ),
    ),
    (
        "🍔U3",
        (
            &["🍔U2", "🍔U4", "🍔U5", "🍔U0", "🍕P8"],
            &["👻U0", "👻U1", "👻U2", "👻U3"],
        ),
    ),
    ("🍔U4", (&["🍔U3", "🍔U5", "🍔U6", "🍔U7"], &[])),
    ("🍔U5", (&["🍔U1", "🍔U2", "🍔U3", "🍔U4", "🍔U7"], &[])),
    ("🍔U6", (&["🍔U8", "🍔U4", "🍔U7"], &[])),
    (
        "🍔U7",
        (
            &["🍔U1", "🍔U4", "🍔U5", "🍔U6", "🍔U8", "🍔T3", "🍔T4"],
            &[],
        ),
    ),
    ("🍔U8", (&["🍔U6", "🍔U7", "🍔T4", "🍔T7"], &[])),
    (
        "🐦‍🔥P0",
        (&["🐦‍🔥P1", "🐦‍🔥P2", "🐦‍🔥P3"], &["🧩P0", "🧩P1", "🧩P2", "🧩P3"]),
    ),
    (
        "🐦‍🔥P1",
        (
            &["🐦‍🔥P2", "🐦‍🔥P5", "🐦‍🔥P6", "🐦‍🔥P0"],
            &["🧩P0", "🧩P1", "🧩P2", "🧩P3"],
        ),
    ),
    (
        "🐦‍🔥P2",
        (
            &["🐦‍🔥P1", "🐦‍🔥P3", "🐦‍🔥P4", "🐦‍🔥P5", "🐦‍🔥P0"],
            &["🧩P0", "🧩P1", "🧩P2", "🧩P3"],
        ),
    ),
    (
        "🐦‍🔥P3",
        (
            &["🐦‍🔥P2", "🐦‍🔥P4", "🐦‍🔥P0", "🐦‍🔥Q4", "🐦‍🔥R8"],
            &["🧩P0", "🧩P1", "🧩P2", "🧩P3"],
        ),
    ),
    (
        "🐦‍🔥P4",
        (
            &["🐦‍🔥P2", "🐦‍🔥P3", "🐦‍🔥P5", "🐦‍🔥P7", "🐦‍🔥Q4", "🐦‍🔥Q3", "⛰️P1"],
            &[],
        ),
    ),
    ("🐦‍🔥P5", (&["🐦‍🔥P1", "🐦‍🔥P2", "🐦‍🔥P4", "🐦‍🔥P6", "🐦‍🔥P7"], &[])),
    ("🐦‍🔥P6", (&["🐦‍🔥P8", "🐦‍🔥P1", "🐦‍🔥P5", "🐦‍🔥P7"], &[])),
    (
        "🐦‍🔥P7",
        (&["🐦‍🔥P8", "🐦‍🔥P4", "🐦‍🔥P5", "🐦‍🔥P6", "⛰️P1", "⛰️P6"], &[]),
    ),
    ("🐦‍🔥P8", (&["🐦‍🔥P6", "🐦‍🔥P7", "⛰️P6", "⛰️P8", "📚U3"], &[])),
    ("🐦‍🔥Q0", (&["🐦‍🔥Q1", "🐦‍🔥Q2", "🐦‍🔥Q3"], &[])),
    ("🐦‍🔥Q1", (&["🐦‍🔥Q2", "🐦‍🔥Q5", "🐦‍🔥Q6", "🐦‍🔥Q0"], &[])),
    ("🐦‍🔥Q2", (&["🐦‍🔥Q1", "🐦‍🔥Q3", "🐦‍🔥Q4", "🐦‍🔥Q5", "🐦‍🔥Q0"], &[])),
    ("🐦‍🔥Q3", (&["🐦‍🔥Q2", "🐦‍🔥Q4", "🐦‍🔥Q0", "🐦‍🔥P4", "⛰️P1"], &[])),
    (
        "🐦‍🔥Q4",
        (
            &["🐦‍🔥Q2", "🐦‍🔥Q3", "🐦‍🔥Q5", "🐦‍🔥Q7", "🐦‍🔥P3", "🐦‍🔥P4", "🐦‍🔥R8"],
            &[],
        ),
    ),
    ("🐦‍🔥Q5", (&["🐦‍🔥Q1", "🐦‍🔥Q2", "🐦‍🔥Q4", "🐦‍🔥Q6", "🐦‍🔥Q7"], &[])),
    ("🐦‍🔥Q6", (&["🐦‍🔥Q8", "🐦‍🔥Q1", "🐦‍🔥Q5", "🐦‍🔥Q7"], &[])),
    ("🐦‍🔥Q7", (&["🐦‍🔥Q8", "🐦‍🔥Q4", "🐦‍🔥Q5", "🐦‍🔥Q6", "🐦‍🔥R8"], &[])),
    (
        "🐦‍🔥Q8",
        (&["🐦‍🔥Q6", "🐦‍🔥Q7", "🐦‍🔥R8", "🐦‍🔥R7", "🐦‍🔥S8", "🐉U3"], &[]),
    ),
    (
        "🐦‍🔥R0",
        (&["🐦‍🔥R1", "🐦‍🔥R2", "🐦‍🔥R3"], &["🧩R0", "🧩R1", "🧩R2", "🧩R3"]),
    ),
    (
        "🐦‍🔥R1",
        (&["🐦‍🔥R2", "🐦‍🔥R6", "🐦‍🔥R0"], &["🧩R0", "🧩R1", "🧩R2", "🧩R3"]),
    ),
    (
        "🐦‍🔥R2",
        (
            &["🐦‍🔥R1", "🐦‍🔥R3", "🐦‍🔥R4", "🐦‍🔥R5", "🐦‍🔥R6", "🐦‍🔥R0"],
            &["🧩R0", "🧩R1", "🧩R2", "🧩R3"],
        ),
    ),
    (
        "🐦‍🔥R3",
        (
            &["🐦‍🔥R2", "🐦‍🔥R4", "🐦‍🔥R0", "🐦‍🔥S1", "🐦‍🔥S6", "🐦‍🔥S8"],
            &["🧩R0", "🧩R1", "🧩R2", "🧩R3"],
        ),
    ),
    ("🐦‍🔥R4", (&["🐦‍🔥R2", "🐦‍🔥R3", "🐦‍🔥R5", "🐦‍🔥R7", "🐦‍🔥S8"], &[])),
    ("🐦‍🔥R5", (&["🐦‍🔥R2", "🐦‍🔥R4", "🐦‍🔥R6", "🐦‍🔥R7", "🐦‍🔥R8"], &[])),
    ("🐦‍🔥R6", (&["🐦‍🔥R8", "🐦‍🔥R1", "🐦‍🔥R2", "🐦‍🔥R5"], &[])),
    (
        "🐦‍🔥R7",
        (&["🐦‍🔥R8", "🐦‍🔥R4", "🐦‍🔥R5", "🐦‍🔥Q8", "🐦‍🔥S8", "🐉U3"], &[]),
    ),
    (
        "🐦‍🔥R8",
        (
            &["🐦‍🔥R5", "🐦‍🔥R6", "🐦‍🔥R7", "🐦‍🔥Q4", "🐦‍🔥Q7", "🐦‍🔥Q8", "🐦‍🔥P3"],
            &[],
        ),
    ),
    (
        "🐦‍🔥S0",
        (&["🐦‍🔥S1", "🐦‍🔥S2", "🐦‍🔥S3"], &["🧩S0", "🧩S1", "🧩S2", "🧩S3"]),
    ),
    (
        "🐦‍🔥S1",
        (
            &["🐦‍🔥S2", "🐦‍🔥S5", "🐦‍🔥S6", "🐦‍🔥S0", "🐦‍🔥R3"],
            &["🧩S0", "🧩S1", "🧩S2", "🧩S3"],
        ),
    ),
    (
        "🐦‍🔥S2",
        (
            &["🐦‍🔥S1", "🐦‍🔥S3", "🐦‍🔥S4", "🐦‍🔥S5", "🐦‍🔥S0"],
            &["🧩S0", "🧩S1", "🧩S2", "🧩S3"],
        ),
    ),
    (
        "🐦‍🔥S3",
        (
            &["🐦‍🔥S2", "🐦‍🔥S4", "🐦‍🔥S0", "🐦‍🔥T1", "🐦‍🔥T7"],
            &["🧩S0", "🧩S1", "🧩S2", "🧩S3"],
        ),
    ),
    (
        "🐦‍🔥S4",
        (
            &[
                "🐦‍🔥S2",
                "🐦‍🔥S3",
                "🐦‍🔥S5",
                "🐦‍🔥S7",
                "🐦‍🔥T7",
                "🐦‍🔥T8",
                "🐉U4",
                "🐉U7",
            ],
            &[],
        ),
    ),
    ("🐦‍🔥S5", (&["🐦‍🔥S1", "🐦‍🔥S2", "🐦‍🔥S4", "🐦‍🔥S6", "🐦‍🔥S7"], &[])),
    ("🐦‍🔥S6", (&["🐦‍🔥S8", "🐦‍🔥S1", "🐦‍🔥S5", "🐦‍🔥S7", "🐦‍🔥R3"], &[])),
    (
        "🐦‍🔥S7",
        (&["🐦‍🔥S8", "🐦‍🔥S4", "🐦‍🔥S5", "🐦‍🔥S6", "🐉U3", "🐉U4"], &[]),
    ),
    (
        "🐦‍🔥S8",
        (
            &["🐦‍🔥S6", "🐦‍🔥S7", "🐦‍🔥R3", "🐦‍🔥R4", "🐦‍🔥R7", "🐦‍🔥Q8", "🐉U3"],
            &[],
        ),
    ),
    (
        "🐦‍🔥T0",
        (&["🐦‍🔥T1", "🐦‍🔥T2", "🐦‍🔥T3"], &["🧩T0", "🧩T1", "🧩T2", "🧩T3"]),
    ),
    (
        "🐦‍🔥T1",
        (
            &["🐦‍🔥T2", "🐦‍🔥T5", "🐦‍🔥T7", "🐦‍🔥T0", "🐦‍🔥S3"],
            &["🧩T0", "🧩T1", "🧩T2", "🧩T3"],
        ),
    ),
    (
        "🐦‍🔥T2",
        (
            &["🐦‍🔥T1", "🐦‍🔥T3", "🐦‍🔥T5", "🐦‍🔥T0"],
            &["🧩T0", "🧩T1", "🧩T2", "🧩T3"],
        ),
    ),
    (
        "🐦‍🔥T3",
        (
            &["🐦‍🔥T2", "🐦‍🔥T4", "🐦‍🔥T5", "🐦‍🔥T0", "🐦‍🔥U1", "🐦‍🔥U6"],
            &["🧩T0", "🧩T1", "🧩T2", "🧩T3"],
        ),
    ),
    (
        "🐦‍🔥T4",
        (&["🐦‍🔥T3", "🐦‍🔥T5", "🐦‍🔥T6", "🐦‍🔥T7", "🐦‍🔥U6", "🐦‍🔥U8"], &[]),
    ),
    ("🐦‍🔥T5", (&["🐦‍🔥T1", "🐦‍🔥T2", "🐦‍🔥T3", "🐦‍🔥T4", "🐦‍🔥T7"], &[])),
    ("🐦‍🔥T6", (&["🐦‍🔥T8", "🐦‍🔥T4", "🐦‍🔥T7", "🐦‍🔥U8"], &[])),
    (
        "🐦‍🔥T7",
        (
            &["🐦‍🔥T1", "🐦‍🔥T4", "🐦‍🔥T5", "🐦‍🔥T6", "🐦‍🔥T8", "🐦‍🔥S3", "🐦‍🔥S4"],
            &[],
        ),
    ),
    ("🐦‍🔥T8", (&["🐦‍🔥T6", "🐦‍🔥T7", "🐦‍🔥S4", "🐉U7"], &[])),
    (
        "🐦‍🔥U0",
        (&["🐦‍🔥U1", "🐦‍🔥U2", "🐦‍🔥U3"], &["🧩U0", "🧩U1", "🧩U2", "🧩U3"]),
    ),
    (
        "🐦‍🔥U1",
        (
            &["🐦‍🔥U2", "🐦‍🔥U4", "🐦‍🔥U5", "🐦‍🔥U6", "🐦‍🔥U0", "🐦‍🔥T3"],
            &["🧩U0", "🧩U1", "🧩U2", "🧩U3"],
        ),
    ),
    (
        "🐦‍🔥U2",
        (
            &["🐦‍🔥U1", "🐦‍🔥U3", "🐦‍🔥U4", "🐦‍🔥U0"],
            &["🧩U0", "🧩U1", "🧩U2", "🧩U3"],
        ),
    ),
    (
        "🐦‍🔥U3",
        (
            &["🐦‍🔥U2", "🐦‍🔥U4", "🐦‍🔥U0", "🐉P7", "🐉P8", "🍔P8"],
            &["🧩U0", "🧩U1", "🧩U2", "🧩U3"],
        ),
    ),
    ("🐦‍🔥U4", (&["🐦‍🔥U1", "🐦‍🔥U2", "🐦‍🔥U3", "🐦‍🔥U5", "🐉P7"], &[])),
    (
        "🐦‍🔥U5",
        (
            &["🐦‍🔥U1", "🐦‍🔥U4", "🐦‍🔥U6", "🐦‍🔥U7", "🐦‍🔥U8", "🐉P4", "🐉P7"],
            &[],
        ),
    ),
    ("🐦‍🔥U6", (&["🐦‍🔥U8", "🐦‍🔥U1", "🐦‍🔥U5", "🐦‍🔥T3", "🐦‍🔥T4"], &[])),
    ("🐦‍🔥U7", (&["🐦‍🔥U8", "🐦‍🔥U5", "🐉P4", "🐉Q8"], &[])),
    ("🐦‍🔥U8", (&["🐦‍🔥U5", "🐦‍🔥U6", "🐦‍🔥U7", "🐦‍🔥T4", "🐦‍🔥T6"], &[])),
    (
        "📚P0",
        (&["📚P1", "📚P2", "📚P3"], &["🌵P0", "🌵P1", "🌵P2", "🌵P3"]),
    ),
    (
        "📚P1",
        (&["📚P2", "📚P6", "📚P0"], &["🌵P0", "🌵P1", "🌵P2", "🌵P3"]),
    ),
    (
        "📚P2",
        (
            &["📚P1", "📚P3", "📚P4", "📚P5", "📚P6", "📚P0"],
            &["🌵P0", "🌵P1", "🌵P2", "🌵P3"],
        ),
    ),
    (
        "📚P3",
        (
            &["📚P2", "📚P4", "📚P0", "📚Q4", "📚Q3", "📚R8"],
            &["🌵P0", "🌵P1", "🌵P2", "🌵P3"],
        ),
    ),
    ("📚P4", (&["📚P2", "📚P3", "📚P5", "📚P7", "📚Q3"], &[])),
    ("📚P5", (&["📚P2", "📚P4", "📚P6", "📚P7", "📚P8"], &[])),
    ("📚P6", (&["📚P8", "📚P1", "📚P2", "📚P5"], &[])),
    (
        "📚P7",
        (&["📚P8", "📚P4", "📚P5", "📚Q3", "🎈P1", "🎈P6"], &[]),
    ),
    (
        "📚P8",
        (&["📚P5", "📚P6", "📚P7", "🎈P6", "🎈P8", "🍌U3"], &[]),
    ),
    (
        "📚Q0",
        (&["📚Q1", "📚Q2", "📚Q3"], &["🐑Q0", "🐑Q1", "🐑Q2", "🐑Q3"]),
    ),
    (
        "📚Q1",
        (
            &["📚Q2", "📚Q5", "📚Q6", "📚Q0"],
            &["🐑Q0", "🐑Q1", "🐑Q2", "🐑Q3"],
        ),
    ),
    (
        "📚Q2",
        (
            &["📚Q1", "📚Q3", "📚Q4", "📚Q5", "📚Q0"],
            &["🐑Q0", "🐑Q1", "🐑Q2", "🐑Q3"],
        ),
    ),
    (
        "📚Q3",
        (
            &["📚Q2", "📚Q4", "📚Q0", "📚P3", "📚P4", "📚P7", "🎈P1"],
            &["🐑Q0", "🐑Q1", "🐑Q2", "🐑Q3"],
        ),
    ),
    (
        "📚Q4",
        (&["📚Q2", "📚Q3", "📚Q5", "📚Q7", "📚P3", "📚R8"], &[]),
    ),
    ("📚Q5", (&["📚Q1", "📚Q2", "📚Q4", "📚Q6", "📚Q7"], &[])),
    ("📚Q6", (&["📚Q8", "📚Q1", "📚Q5", "📚Q7"], &[])),
    ("📚Q7", (&["📚Q8", "📚Q4", "📚Q5", "📚Q6", "📚R8"], &[])),
    (
        "📚Q8",
        (&["📚Q6", "📚Q7", "📚R8", "📚R7", "📚S8", "⛰️U3"], &[]),
    ),
    (
        "📚R0",
        (&["📚R1", "📚R2", "📚R3"], &["🌵R0", "🌵R1", "🌵R2", "🌵R3"]),
    ),
    (
        "📚R1",
        (
            &["📚R2", "📚R5", "📚R6", "📚R0"],
            &["🌵R0", "🌵R1", "🌵R2", "🌵R3"],
        ),
    ),
    (
        "📚R2",
        (
            &["📚R1", "📚R3", "📚R4", "📚R5", "📚R0"],
            &["🌵R0", "🌵R1", "🌵R2", "🌵R3"],
        ),
    ),
    (
        "📚R3",
        (
            &["📚R2", "📚R4", "📚R0", "📚S1"],
            &["🌵R0", "🌵R1", "🌵R2", "🌵R3"],
        ),
    ),
    (
        "📚R4",
        (
            &[
                "📚R2", "📚R3", "📚R5", "📚R7", "📚R8", "📚S1", "📚S7", "📚S8",
            ],
            &[],
        ),
    ),
    ("📚R5", (&["📚R1", "📚R2", "📚R4", "📚R6", "📚R8"], &[])),
    ("📚R6", (&["📚R8", "📚R1", "📚R5"], &[])),
    ("📚R7", (&["📚R8", "📚R4", "📚Q8", "📚S8", "⛰️U3"], &[])),
    (
        "📚R8",
        (
            &[
                "📚R4", "📚R5", "📚R6", "📚R7", "📚Q4", "📚Q7", "📚Q8", "📚P3",
            ],
            &[],
        ),
    ),
    (
        "📚S0",
        (&["📚S1", "📚S2", "📚S3"], &["🌵S0", "🌵S1", "🌵S2", "🌵S3"]),
    ),
    (
        "📚S1",
        (
            &["📚S2", "📚S5", "📚S7", "📚S0", "📚R3", "📚R4"],
            &["🌵S0", "🌵S1", "🌵S2", "🌵S3"],
        ),
    ),
    (
        "📚S2",
        (
            &["📚S1", "📚S3", "📚S5", "📚S0"],
            &["🌵S0", "🌵S1", "🌵S2", "🌵S3"],
        ),
    ),
    (
        "📚S3",
        (
            &["📚S2", "📚S4", "📚S5", "📚S0", "📚T1", "📚T7"],
            &["🌵S0", "🌵S1", "🌵S2", "🌵S3"],
        ),
    ),
    (
        "📚S4",
        (&["📚S3", "📚S5", "📚S6", "📚S7", "📚T7", "📚T8"], &[]),
    ),
    ("📚S5", (&["📚S1", "📚S2", "📚S3", "📚S4", "📚S7"], &[])),
    (
        "📚S6",
        (&["📚S8", "📚S4", "📚S7", "📚T8", "⛰️U4", "⛰️U6"], &[]),
    ),
    (
        "📚S7",
        (&["📚S1", "📚S4", "📚S5", "📚S6", "📚S8", "📚R4"], &[]),
    ),
    (
        "📚S8",
        (
            &["📚S6", "📚S7", "📚R4", "📚R7", "📚Q8", "⛰️U3", "⛰️U4"],
            &[],
        ),
    ),
    (
        "📚T0",
        (&["📚T1", "📚T2", "📚T3"], &["🌵T0", "🌵T1", "🌵T2", "🌵T3"]),
    ),
    (
        "📚T1",
        (
            &["📚T2", "📚T5", "📚T7", "📚T0", "📚S3"],
            &["🌵T0", "🌵T1", "🌵T2", "🌵T3"],
        ),
    ),
    (
        "📚T2",
        (
            &["📚T1", "📚T3", "📚T5", "📚T0"],
            &["🌵T0", "🌵T1", "🌵T2", "🌵T3"],
        ),
    ),
    (
        "📚T3",
        (
            &["📚T2", "📚T4", "📚T5", "📚T0", "📚U1", "📚U6"],
            &["🌵T0", "🌵T1", "🌵T2", "🌵T3"],
        ),
    ),
    (
        "📚T4",
        (&["📚T3", "📚T5", "📚T6", "📚T7", "📚U6", "📚U8"], &[]),
    ),
    ("📚T5", (&["📚T1", "📚T2", "📚T3", "📚T4", "📚T7"], &[])),
    ("📚T6", (&["📚T8", "📚T4", "📚T7", "📚U8"], &[])),
    (
        "📚T7",
        (
            &["📚T1", "📚T4", "📚T5", "📚T6", "📚T8", "📚S3", "📚S4"],
            &[],
        ),
    ),
    ("📚T8", (&["📚T6", "📚T7", "📚S4", "📚S6", "⛰️U6"], &[])),
    (
        "📚U0",
        (&["📚U1", "📚U2", "📚U3"], &["🌵U0", "🌵U1", "🌵U2", "🌵U3"]),
    ),
    (
        "📚U1",
        (
            &["📚U2", "📚U5", "📚U6", "📚U0", "📚T3"],
            &["🌵U0", "🌵U1", "🌵U2", "🌵U3"],
        ),
    ),
    (
        "📚U2",
        (
            &["📚U1", "📚U3", "📚U4", "📚U5", "📚U0"],
            &["🌵U0", "🌵U1", "🌵U2", "🌵U3"],
        ),
    ),
    (
        "📚U3",
        (
            &["📚U2", "📚U4", "📚U0", "⛰️P8", "🐦‍🔥P8"],
            &["🌵U0", "🌵U1", "🌵U2", "🌵U3"],
        ),
    ),
    (
        "📚U4",
        (
            &["📚U2", "📚U3", "📚U5", "📚U7", "📚U8", "⛰️P7", "⛰️P8"],
            &[],
        ),
    ),
    ("📚U5", (&["📚U1", "📚U2", "📚U4", "📚U6", "📚U8"], &[])),
    ("📚U6", (&["📚U8", "📚U1", "📚U5", "📚T3", "📚T4"], &[])),
    ("📚U7", (&["📚U8", "📚U4", "⛰️P4", "⛰️P7", "⛰️Q8"], &[])),
    (
        "📚U8",
        (&["📚U4", "📚U5", "📚U6", "📚U7", "📚T4", "📚T6"], &[]),
    ),
    (
        "🥯P0",
        (&["🥯P1", "🥯P2", "🥯P3"], &["🐑U0", "🐑U1", "🐑U2", "🐑U3"]),
    ),
    (
        "🥯P1",
        (
            &[
                "🥯P2", "🥯P4", "🥯P5", "🥯P6", "🥯P0", "🍌P4", "🍌P7", "🍌Q3",
            ],
            &["🐑U0", "🐑U1", "🐑U2", "🐑U3"],
        ),
    ),
    (
        "🥯P2",
        (
            &["🥯P1", "🥯P3", "🥯P4", "🥯P0"],
            &["🐑U0", "🐑U1", "🐑U2", "🐑U3"],
        ),
    ),
    (
        "🥯P3",
        (
            &["🥯P2", "🥯P4", "🥯P0", "🥯Q1", "🥯Q7"],
            &["🐑U0", "🐑U1", "🐑U2", "🐑U3"],
        ),
    ),
    (
        "🥯P4",
        (&["🥯P1", "🥯P2", "🥯P3", "🥯P5", "🥯Q7", "🥯Q8"], &[]),
    ),
    (
        "🥯P5",
        (&["🥯P1", "🥯P4", "🥯P6", "🥯P7", "🥯P8", "🥯Q8"], &[]),
    ),
    ("🥯P6", (&["🥯P8", "🥯P1", "🥯P5", "🍌P7", "🍌P8"], &[])),
    ("🥯P7", (&["🥯P8", "🥯P5", "🥯Q8", "🐺U6", "🐺U4"], &[])),
    (
        "🥯P8",
        (&["🥯P5", "🥯P6", "🥯P7", "🍌P8", "🐺U4", "🐺U3"], &[]),
    ),
    (
        "🥯Q0",
        (&["🥯Q1", "🥯Q2", "🥯Q3"], &["🐑U0", "🐑U1", "🐑U2", "🐑U3"]),
    ),
    (
        "🥯Q1",
        (
            &["🥯Q2", "🥯Q5", "🥯Q7", "🥯Q0", "🥯P3"],
            &["🐑U0", "🐑U1", "🐑U2", "🐑U3"],
        ),
    ),
    (
        "🥯Q2",
        (
            &["🥯Q1", "🥯Q3", "🥯Q5", "🥯Q0"],
            &["🐑U0", "🐑U1", "🐑U2", "🐑U3"],
        ),
    ),
    (
        "🥯Q3",
        (
            &["🥯Q2", "🥯Q4", "🥯Q5", "🥯Q0", "🥯R1", "🥯R6"],
            &["🐑U0", "🐑U1", "🐑U2", "🐑U3"],
        ),
    ),
    (
        "🥯Q4",
        (&["🥯Q3", "🥯Q5", "🥯Q6", "🥯Q7", "🥯R6", "🥯R8"], &[]),
    ),
    ("🥯Q5", (&["🥯Q1", "🥯Q2", "🥯Q3", "🥯Q4", "🥯Q7"], &[])),
    ("🥯Q6", (&["🥯Q8", "🥯Q4", "🥯Q7", "🥯R8"], &[])),
    (
        "🥯Q7",
        (
            &["🥯Q1", "🥯Q4", "🥯Q5", "🥯Q6", "🥯Q8", "🥯P3", "🥯P4"],
            &[],
        ),
    ),
    (
        "🥯Q8",
        (&["🥯Q6", "🥯Q7", "🥯P4", "🥯P5", "🥯P7", "🐺U6"], &[]),
    ),
    (
        "🥯R0",
        (&["🥯R1", "🥯R2", "🥯R3"], &["🐑U0", "🐑U1", "🐑U2", "🐑U3"]),
    ),
    (
        "🥯R1",
        (
            &["🥯R2", "🥯R6", "🥯R0", "🥯Q3"],
            &["🐑U0", "🐑U1", "🐑U2", "🐑U3"],
        ),
    ),
    (
        "🥯R2",
        (
            &["🥯R1", "🥯R3", "🥯R4", "🥯R5", "🥯R6", "🥯R0"],
            &["🐑U0", "🐑U1", "🐑U2", "🐑U3"],
        ),
    ),
    (
        "🥯R3",
        (
            &["🥯R2", "🥯R4", "🥯R0", "🥯S8", "🥯S7", "⚡P1"],
            &["🐑U0", "🐑U1", "🐑U2", "🐑U3"],
        ),
    ),
    (
        "🥯R4",
        (&["🥯R2", "🥯R3", "🥯R5", "🥯R7", "🥯S7", "🥯S6"], &[]),
    ),
    ("🥯R5", (&["🥯R2", "🥯R4", "🥯R6", "🥯R7", "🥯R8"], &[])),
    (
        "🥯R6",
        (&["🥯R8", "🥯R1", "🥯R2", "🥯R5", "🥯Q3", "🥯Q4"], &[]),
    ),
    ("🥯R7", (&["🥯R8", "🥯R4", "🥯R5", "🥯S6", "🥯T8"], &[])),
    ("🥯R8", (&["🥯R5", "🥯R6", "🥯R7", "🥯Q4", "🥯Q6"], &[])),
    (
        "🥯S0",
        (&["🥯S1", "🥯S2", "🥯S3"], &["⚡T0", "⚡T1", "⚡T2", "⚡T3"]),
    ),
    (
        "🥯S1",
        (
            &[
                "🥯S8", "🥯S2", "🥯S5", "🥯S7", "🥯S0", "⚡P6", "⚡P8", "⚡Q3",
            ],
            &["⚡T0", "⚡T1", "⚡T2", "⚡T3"],
        ),
    ),
    (
        "🥯S2",
        (
            &["🥯S1", "🥯S3", "🥯S4", "🥯S5", "🥯S0"],
            &["⚡T0", "⚡T1", "⚡T2", "⚡T3"],
        ),
    ),
    (
        "🥯S3",
        (
            &["🥯S2", "🥯S4", "🥯S0", "🥯T1"],
            &["⚡T0", "⚡T1", "⚡T2", "⚡T3"],
        ),
    ),
    (
        "🥯S4",
        (&["🥯S2", "🥯S3", "🥯S5", "🥯S6", "🥯T1", "🥯T6"], &[]),
    ),
    ("🥯S5", (&["🥯S1", "🥯S2", "🥯S4", "🥯S6", "🥯S7"], &[])),
    (
        "🥯S6",
        (
            &["🥯S4", "🥯S5", "🥯S7", "🥯T6", "🥯T8", "🥯R4", "🥯R7"],
            &[],
        ),
    ),
    (
        "🥯S7",
        (&["🥯S8", "🥯S1", "🥯S5", "🥯S6", "🥯R3", "🥯R4"], &[]),
    ),
    ("🥯S8", (&["🥯S1", "🥯S7", "🥯R3", "⚡P1", "⚡P6"], &[])),
    (
        "🥯T0",
        (&["🥯T1", "🥯T2", "🥯T3"], &["⚡T0", "⚡T1", "⚡T2", "⚡T3"]),
    ),
    (
        "🥯T1",
        (
            &["🥯T2", "🥯T5", "🥯T6", "🥯T0", "🥯S3", "🥯S4"],
            &["⚡T0", "⚡T1", "⚡T2", "⚡T3"],
        ),
    ),
    (
        "🥯T2",
        (
            &["🥯T1", "🥯T3", "🥯T4", "🥯T5", "🥯T0"],
            &["⚡T0", "⚡T1", "⚡T2", "⚡T3"],
        ),
    ),
    (
        "🥯T3",
        (
            &["🥯T2", "🥯T4", "🥯T0", "🥯U1", "🥯U6"],
            &["⚡T0", "⚡T1", "⚡T2", "⚡T3"],
        ),
    ),
    (
        "🥯T4",
        (
            &["🥯T2", "🥯T3", "🥯T5", "🥯T7", "🥯T8", "🥯U6", "🥯U8"],
            &[],
        ),
    ),
    ("🥯T5", (&["🥯T1", "🥯T2", "🥯T4", "🥯T6", "🥯T8"], &[])),
    ("🥯T6", (&["🥯T8", "🥯T1", "🥯T5", "🥯S4", "🥯S6"], &[])),
    ("🥯T7", (&["🥯T8", "🥯T4", "🥯U8"], &[])),
    (
        "🥯T8",
        (&["🥯T4", "🥯T5", "🥯T6", "🥯T7", "🥯S6", "🥯R7"], &[]),
    ),
    (
        "🥯U0",
        (&["🥯U1", "🥯U2", "🥯U3"], &["⚡T0", "⚡T1", "⚡T2", "⚡T3"]),
    ),
    (
        "🥯U1",
        (
            &["🥯U2", "🥯U4", "🥯U5", "🥯U6", "🥯U0", "🥯T3"],
            &["⚡T0", "⚡T1", "⚡T2", "⚡T3"],
        ),
    ),
    (
        "🥯U2",
        (
            &["🥯U1", "🥯U3", "🥯U4", "🥯U0"],
            &["⚡T0", "⚡T1", "⚡T2", "⚡T3"],
        ),
    ),
    (
        "🥯U3",
        (
            &["🥯U2", "🥯U4", "🥯U0", "🐺S8", "🐺S7", "🐺R6", "🐺Q8"],
            &["⚡T0", "⚡T1", "⚡T2", "⚡T3"],
        ),
    ),
    (
        "🥯U4",
        (
            &["🥯U1", "🥯U2", "🥯U3", "🥯U5", "🥯U7", "🐺S7", "🐺S6"],
            &[],
        ),
    ),
    ("🥯U5", (&["🥯U1", "🥯U4", "🥯U6", "🥯U7"], &[])),
    (
        "🥯U6",
        (&["🥯U8", "🥯U1", "🥯U5", "🥯U7", "🥯T3", "🥯T4"], &[]),
    ),
    (
        "🥯U7",
        (&["🥯U8", "🥯U4", "🥯U5", "🥯U6", "🐺S6", "🐺T8"], &[]),
    ),
    ("🥯U8", (&["🥯U6", "🥯U7", "🥯T4", "🥯T7"], &[])),
    (
        "🤠P0",
        (&["🤠P1", "🤠P2", "🤠P3"], &["⚡Q0", "⚡Q1", "⚡Q2", "⚡Q3"]),
    ),
    (
        "🤠P1",
        (
            &["🤠P8", "🤠P2", "🤠P6", "🤠P0", "🐺P7", "🐺P8", "🐺Q3"],
            &["⚡Q0", "⚡Q1", "⚡Q2", "⚡Q3"],
        ),
    ),
    (
        "🤠P2",
        (
            &["🤠P1", "🤠P3", "🤠P5", "🤠P6", "🤠P0"],
            &["⚡Q0", "⚡Q1", "⚡Q2", "⚡Q3"],
        ),
    ),
    (
        "🤠P3",
        (
            &["🤠P2", "🤠P4", "🤠P5", "🤠P0", "🤠Q1", "🤠Q6"],
            &["⚡Q0", "⚡Q1", "⚡Q2", "⚡Q3"],
        ),
    ),
    (
        "🤠P4",
        (
            &["🤠P3", "🤠P5", "🤠P7", "🤠Q6", "🤠Q8", "🍕U7", "🍕U4"],
            &[],
        ),
    ),
    ("🤠P5", (&["🤠P2", "🤠P3", "🤠P4", "🤠P6", "🤠P7"], &[])),
    ("🤠P6", (&["🤠P1", "🤠P2", "🤠P5", "🤠P7", "🤠P8"], &[])),
    (
        "🤠P7",
        (&["🤠P8", "🤠P4", "🤠P5", "🤠P6", "🍕U4", "🍕U3"], &[]),
    ),
    ("🤠P8", (&["🤠P1", "🤠P6", "🤠P7", "🐺P8", "🍕U3"], &[])),
    (
        "🤠Q0",
        (&["🤠Q1", "🤠Q2", "🤠Q3"], &["⚡Q0", "⚡Q1", "⚡Q2", "⚡Q3"]),
    ),
    (
        "🤠Q1",
        (
            &["🤠Q2", "🤠Q4", "🤠Q5", "🤠Q6", "🤠Q0", "🤠P3"],
            &["⚡Q0", "⚡Q1", "⚡Q2", "⚡Q3"],
        ),
    ),
    (
        "🤠Q2",
        (
            &["🤠Q1", "🤠Q3", "🤠Q4", "🤠Q0"],
            &["⚡Q0", "⚡Q1", "⚡Q2", "⚡Q3"],
        ),
    ),
    (
        "🤠Q3",
        (
            &["🤠Q2", "🤠Q4", "🤠Q0", "🤠R1", "🤠R6"],
            &["⚡Q0", "⚡Q1", "⚡Q2", "⚡Q3"],
        ),
    ),
    ("🤠Q4", (&["🤠Q1", "🤠Q2", "🤠Q3", "🤠Q5", "🤠R6"], &[])),
    (
        "🤠Q5",
        (
            &["🤠Q1", "🤠Q4", "🤠Q6", "🤠Q7", "🤠Q8", "🤠R6", "🤠R8"],
            &[],
        ),
    ),
    ("🤠Q6", (&["🤠Q8", "🤠Q1", "🤠Q5", "🤠P3", "🤠P4"], &[])),
    ("🤠Q7", (&["🤠Q8", "🤠Q5", "🤠R8"], &[])),
    ("🤠Q8", (&["🤠Q5", "🤠Q6", "🤠Q7", "🤠P4", "🍕U7"], &[])),
    (
        "🤠R0",
        (&["🤠R1", "🤠R2", "🤠R3"], &["⚡Q0", "⚡Q1", "⚡Q2", "⚡Q3"]),
    ),
    (
        "🤠R1",
        (
            &["🤠R2", "🤠R5", "🤠R6", "🤠R0", "🤠Q3"],
            &["⚡Q0", "⚡Q1", "⚡Q2", "⚡Q3"],
        ),
    ),
    (
        "🤠R2",
        (
            &["🤠R1", "🤠R3", "🤠R4", "🤠R5", "🤠R0"],
            &["⚡Q0", "⚡Q1", "⚡Q2", "⚡Q3"],
        ),
    ),
    (
        "🤠R3",
        (
            &["🤠R2", "🤠R4", "🤠R0", "🤠S8", "⚡T1"],
            &["⚡Q0", "⚡Q1", "⚡Q2", "⚡Q3"],
        ),
    ),
    (
        "🤠R4",
        (
            &["🤠R2", "🤠R3", "🤠R5", "🤠R7", "🤠R8", "🤠S8", "🤠S7"],
            &[],
        ),
    ),
    ("🤠R5", (&["🤠R1", "🤠R2", "🤠R4", "🤠R6", "🤠R8"], &[])),
    (
        "🤠R6",
        (&["🤠R8", "🤠R1", "🤠R5", "🤠Q3", "🤠Q4", "🤠Q5"], &[]),
    ),
    ("🤠R7", (&["🤠R8", "🤠R4", "🤠S7", "🤠T8"], &[])),
    (
        "🤠R8",
        (&["🤠R4", "🤠R5", "🤠R6", "🤠R7", "🤠Q5", "🤠Q7"], &[]),
    ),
    ("🤠S0", (&["🤠S1", "🤠S2", "🤠S3"], &[])),
    (
        "🤠S1",
        (&["🤠S2", "🤠S4", "🤠S5", "🤠S6", "🤠S0", "⚡T8"], &[]),
    ),
    ("🤠S2", (&["🤠S1", "🤠S3", "🤠S4", "🤠S0"], &[])),
    ("🤠S3", (&["🤠S2", "🤠S4", "🤠S0", "🤠T1", "🤠T6"], &[])),
    (
        "🤠S4",
        (
            &["🤠S1", "🤠S2", "🤠S3", "🤠S5", "🤠S7", "🤠T6", "🤠T8"],
            &[],
        ),
    ),
    ("🤠S5", (&["🤠S1", "🤠S4", "🤠S6", "🤠S7"], &[])),
    (
        "🤠S6",
        (&["🤠S8", "🤠S1", "🤠S5", "🤠S7", "⚡T1", "⚡T8"], &[]),
    ),
    (
        "🤠S7",
        (
            &["🤠S8", "🤠S4", "🤠S5", "🤠S6", "🤠T8", "🤠R4", "🤠R7"],
            &[],
        ),
    ),
    ("🤠S8", (&["🤠S6", "🤠S7", "🤠R3", "🤠R4", "⚡T1"], &[])),
    ("🤠T0", (&["🤠T1", "🤠T2", "🤠T3"], &[])),
    ("🤠T1", (&["🤠T2", "🤠T5", "🤠T6", "🤠T0", "🤠S3"], &[])),
    ("🤠T2", (&["🤠T1", "🤠T3", "🤠T4", "🤠T5", "🤠T0"], &[])),
    ("🤠T3", (&["🤠T2", "🤠T4", "🤠T0"], &[])),
    ("🤠T4", (&["🤠T2", "🤠T3", "🤠T5", "🤠T7", "🤠T8"], &[])),
    ("🤠T5", (&["🤠T1", "🤠T2", "🤠T4", "🤠T6", "🤠T8"], &[])),
    ("🤠T6", (&["🤠T8", "🤠T1", "🤠T5", "🤠S3", "🤠S4"], &[])),
    ("🤠T7", (&["🤠T8", "🤠T4"], &[])),
    (
        "🤠T8",
        (
            &["🤠T4", "🤠T5", "🤠T6", "🤠T7", "🤠S4", "🤠S7", "🤠R7"],
            &[],
        ),
    ),
    ("🍪Q0", (&["🍪Q1", "🍪Q2", "🍪Q3"], &["⚡U0"])),
    ("🍪Q1", (&["🍪Q2", "🍪Q5", "🍪Q6", "🍪Q0"], &["⚡U0"])),
    (
        "🍪Q2",
        (&["🍪Q1", "🍪Q3", "🍪Q4", "🍪Q5", "🍪Q0"], &["⚡U0"]),
    ),
    (
        "🍪Q3",
        (&["🍪Q2", "🍪Q4", "🍪Q0", "🍪R1", "🍪R6"], &["⚡U0"]),
    ),
    (
        "🍪Q4",
        (&["🍪Q2", "🍪Q3", "🍪Q5", "🍪Q7", "🍪R6", "🍪R8"], &[]),
    ),
    ("🍪Q5", (&["🍪Q1", "🍪Q2", "🍪Q4", "🍪Q6"], &[])),
    ("🍪Q6", (&["🍪Q1", "🍪Q5"], &[])),
    ("🍪Q7", (&["🍪Q4", "🍪R8"], &[])),
    ("🍪R0", (&["🍪R1", "🍪R2", "🍪R3"], &["⚡U0"])),
    (
        "🍪R1",
        (&["🍪R2", "🍪R4", "🍪R5", "🍪R6", "🍪R0", "🍪Q3"], &["⚡U0"]),
    ),
    ("🍪R2", (&["🍪R1", "🍪R3", "🍪R4", "🍪R0"], &["⚡U0"])),
    (
        "🍪R3",
        (&["🍪R2", "🍪R4", "🍪R0", "🍪S8", "🍪S7"], &["⚡U0"]),
    ),
    (
        "🍪R4",
        (
            &["🍪R1", "🍪R2", "🍪R3", "🍪R5", "🍪R7", "🍪S7", "🍪S4"],
            &[],
        ),
    ),
    ("🍪R5", (&["🍪R1", "🍪R4", "🍪R6", "🍪R7"], &[])),
    (
        "🍪R6",
        (&["🍪R8", "🍪R1", "🍪R5", "🍪R7", "🍪Q3", "🍪Q4"], &[]),
    ),
    (
        "🍪R7",
        (&["🍪R8", "🍪R4", "🍪R5", "🍪R6", "🍪S4", "🍪T8"], &[]),
    ),
    ("🍪R8", (&["🍪R6", "🍪R7", "🍪Q4", "🍪Q7"], &[])),
    ("🍪S0", (&["🍪S2", "🍪S3"], &[])),
    ("🍪S2", (&["🍪S3", "🍪S5", "🍪S6", "🍪S0"], &[])),
    (
        "🍪S3",
        (&["🍪S2", "🍪S4", "🍪S5", "🍪S0", "🍪T1", "🍪T6"], &[]),
    ),
    (
        "🍪S4",
        (
            &["🍪S3", "🍪S5", "🍪S7", "🍪T6", "🍪T8", "🍪R4", "🍪R7"],
            &[],
        ),
    ),
    ("🍪S5", (&["🍪S2", "🍪S3", "🍪S4", "🍪S6", "🍪S7"], &[])),
    ("🍪S6", (&["🍪S2", "🍪S5", "🍪S7", "🍪S8"], &[])),
    (
        "🍪S7",
        (&["🍪S8", "🍪S4", "🍪S5", "🍪S6", "🍪R3", "🍪R4"], &[]),
    ),
    ("🍪S8", (&["🍪S6", "🍪S7", "🍪R3"], &[])),
    ("🍪T0", (&["🍪T1", "🍪T2", "🍪T3"], &[])),
    (
        "🍪T1",
        (&["🍪T2", "🍪T4", "🍪T5", "🍪T6", "🍪T0", "🍪S3"], &[]),
    ),
    ("🍪T2", (&["🍪T1", "🍪T3", "🍪T4", "🍪T0"], &[])),
    ("🍪T3", (&["🍪T2", "🍪T4", "🍪T0"], &[])),
    ("🍪T4", (&["🍪T1", "🍪T2", "🍪T3", "🍪T5"], &[])),
    ("🍪T5", (&["🍪T1", "🍪T4", "🍪T6", "🍪T7", "🍪T8"], &[])),
    ("🍪T6", (&["🍪T8", "🍪T1", "🍪T5", "🍪S3", "🍪S4"], &[])),
    ("🍪T7", (&["🍪T8", "🍪T5"], &[])),
    ("🍪T8", (&["🍪T5", "🍪T6", "🍪T7", "🍪S4", "🍪R7"], &[])),
    ("🐉P0", (&["🐉P1", "🐉P2", "🐉P3"], &["🤖Q0"])),
    (
        "🐉P1",
        (&["🐉P8", "🐉P2", "🐉P6", "🐉P0", "🍔P7", "🍔Q3"], &["🤖Q0"]),
    ),
    (
        "🐉P2",
        (&["🐉P1", "🐉P3", "🐉P5", "🐉P6", "🐉P0"], &["🤖Q0"]),
    ),
    (
        "🐉P3",
        (&["🐉P2", "🐉P4", "🐉P5", "🐉P0", "🐉Q1"], &["🤖Q0"]),
    ),
    (
        "🐉P4",
        (
            &["🐉P3", "🐉P5", "🐉P7", "🐉Q1", "🐉Q8", "🐦‍🔥U7", "🐦‍🔥U5"],
            &[],
        ),
    ),
    ("🐉P5", (&["🐉P2", "🐉P3", "🐉P4", "🐉P6", "🐉P7"], &[])),
    ("🐉P6", (&["🐉P1", "🐉P2", "🐉P5", "🐉P7", "🐉P8"], &[])),
    (
        "🐉P7",
        (
            &["🐉P8", "🐉P4", "🐉P5", "🐉P6", "🐦‍🔥U5", "🐦‍🔥U4", "🐦‍🔥U3"],
            &[],
        ),
    ),
    (
        "🐉P8",
        (&["🐉P1", "🐉P6", "🐉P7", "🍔P7", "🍔P8", "🐦‍🔥U3"], &[]),
    ),
    ("🐉Q0", (&["🐉Q1", "🐉Q2", "🐉Q3"], &["🤖Q0"])),
    (
        "🐉Q1",
        (
            &["🐉Q8", "🐉Q2", "🐉Q5", "🐉Q7", "🐉Q0", "🐉P3", "🐉P4"],
            &["🤖Q0"],
        ),
    ),
    (
        "🐉Q2",
        (&["🐉Q1", "🐉Q3", "🐉Q4", "🐉Q5", "🐉Q0"], &["🤖Q0"]),
    ),
    (
        "🐉Q3",
        (&["🐉Q2", "🐉Q4", "🐉Q0", "🐉R1", "🐉R6"], &["🤖Q0"]),
    ),
    (
        "🐉Q4",
        (&["🐉Q2", "🐉Q3", "🐉Q5", "🐉Q6", "🐉R6", "🐉R8"], &[]),
    ),
    ("🐉Q5", (&["🐉Q1", "🐉Q2", "🐉Q4", "🐉Q6", "🐉Q7"], &[])),
    ("🐉Q6", (&["🐉Q4", "🐉Q5", "🐉Q7", "🐉R8"], &[])),
    ("🐉Q7", (&["🐉Q8", "🐉Q1", "🐉Q5", "🐉Q6"], &[])),
    ("🐉Q8", (&["🐉Q1", "🐉Q7", "🐉P4", "🐦‍🔥U7"], &[])),
    ("🐉R0", (&["🐉R1", "🐉R2", "🐉R3"], &["🤖Q0"])),
    (
        "🐉R1",
        (&["🐉R2", "🐉R5", "🐉R6", "🐉R0", "🐉Q3"], &["🤖Q0"]),
    ),
    (
        "🐉R2",
        (&["🐉R1", "🐉R3", "🐉R4", "🐉R5", "🐉R0"], &["🤖Q0"]),
    ),
    (
        "🐉R3",
        (&["🐉R2", "🐉R4", "🐉R0", "🐉S8", "🐉S7"], &["🤖Q0"]),
    ),
    (
        "🐉R4",
        (
            &["🐉R2", "🐉R3", "🐉R5", "🐉R7", "🐉S7", "🐉S4", "🐉T8"],
            &[],
        ),
    ),
    ("🐉R5", (&["🐉R1", "🐉R2", "🐉R4", "🐉R6", "🐉R7"], &[])),
    (
        "🐉R6",
        (&["🐉R8", "🐉R1", "🐉R5", "🐉R7", "🐉Q3", "🐉Q4"], &[]),
    ),
    ("🐉R7", (&["🐉R8", "🐉R4", "🐉R5", "🐉R6"], &[])),
    ("🐉R8", (&["🐉R6", "🐉R7", "🐉Q4", "🐉Q6"], &[])),
    (
        "🐉S0",
        (&["🐉S1", "🐉S2", "🐉S3"], &["🐑P0", "🐑P1", "🐑P2", "🐑P3"]),
    ),
    (
        "🐉S1",
        (
            &["🐉S8", "🐉S2", "🐉S6", "🐉S0"],
            &["🐑P0", "🐑P1", "🐑P2", "🐑P3"],
        ),
    ),
    (
        "🐉S2",
        (
            &["🐉S1", "🐉S3", "🐉S5", "🐉S6", "🐉S0"],
            &["🐑P0", "🐑P1", "🐑P2", "🐑P3"],
        ),
    ),
    (
        "🐉S3",
        (
            &["🐉S2", "🐉S4", "🐉S5", "🐉S0", "🐉T1", "🐉T6"],
            &["🐑P0", "🐑P1", "🐑P2", "🐑P3"],
        ),
    ),
    (
        "🐉S4",
        (&["🐉S3", "🐉S5", "🐉S7", "🐉T6", "🐉T8", "🐉R4"], &[]),
    ),
    ("🐉S5", (&["🐉S2", "🐉S3", "🐉S4", "🐉S6", "🐉S7"], &[])),
    ("🐉S6", (&["🐉S1", "🐉S2", "🐉S5", "🐉S7", "🐉S8"], &[])),
    (
        "🐉S7",
        (&["🐉S8", "🐉S4", "🐉S5", "🐉S6", "🐉R3", "🐉R4"], &[]),
    ),
    ("🐉S8", (&["🐉S1", "🐉S6", "🐉S7", "🐉R3"], &[])),
    (
        "🐉T0",
        (&["🐉T1", "🐉T2", "🐉T3"], &["🐑P0", "🐑P1", "🐑P2", "🐑P3"]),
    ),
    (
        "🐉T1",
        (
            &["🐉T2", "🐉T6", "🐉T0", "🐉S3"],
            &["🐑P0", "🐑P1", "🐑P2", "🐑P3"],
        ),
    ),
    (
        "🐉T2",
        (
            &["🐉T1", "🐉T3", "🐉T4", "🐉T5", "🐉T6", "🐉T0"],
            &["🐑P0", "🐑P1", "🐑P2", "🐑P3"],
        ),
    ),
    (
        "🐉T3",
        (
            &["🐉T2", "🐉T4", "🐉T0", "🐉U1", "🐉U6", "🐉U8"],
            &["🐑P0", "🐑P1", "🐑P2", "🐑P3"],
        ),
    ),
    ("🐉T4", (&["🐉T2", "🐉T3", "🐉T5", "🐉T7", "🐉U8"], &[])),
    ("🐉T5", (&["🐉T2", "🐉T4", "🐉T6", "🐉T7", "🐉T8"], &[])),
    (
        "🐉T6",
        (&["🐉T8", "🐉T1", "🐉T2", "🐉T5", "🐉S3", "🐉S4"], &[]),
    ),
    ("🐉T7", (&["🐉T8", "🐉T4", "🐉T5", "🐉U8"], &[])),
    ("🐉T8", (&["🐉T5", "🐉T6", "🐉T7", "🐉S4", "🐉R4"], &[])),
    (
        "🐉U0",
        (&["🐉U1", "🐉U2", "🐉U3"], &["🐑P0", "🐑P1", "🐑P2", "🐑P3"]),
    ),
    (
        "🐉U1",
        (
            &["🐉U2", "🐉U4", "🐉U5", "🐉U6", "🐉U0", "🐉T3"],
            &["🐑P0", "🐑P1", "🐑P2", "🐑P3"],
        ),
    ),
    (
        "🐉U2",
        (
            &["🐉U1", "🐉U3", "🐉U4", "🐉U0"],
            &["🐑P0", "🐑P1", "🐑P2", "🐑P3"],
        ),
    ),
    (
        "🐉U3",
        (
            &["🐉U2", "🐉U4", "🐉U0", "🐦‍🔥S8", "🐦‍🔥S7", "🐦‍🔥R7", "🐦‍🔥Q8"],
            &["🐑P0", "🐑P1", "🐑P2", "🐑P3"],
        ),
    ),
    (
        "🐉U4",
        (
            &["🐉U1", "🐉U2", "🐉U3", "🐉U5", "🐉U7", "🐦‍🔥S7", "🐦‍🔥S4"],
            &[],
        ),
    ),
    ("🐉U5", (&["🐉U1", "🐉U4", "🐉U6", "🐉U7"], &[])),
    ("🐉U6", (&["🐉U8", "🐉U1", "🐉U5", "🐉U7", "🐉T3"], &[])),
    (
        "🐉U7",
        (&["🐉U8", "🐉U4", "🐉U5", "🐉U6", "🐦‍🔥S4", "🐦‍🔥T8"], &[]),
    ),
    ("🐉U8", (&["🐉U6", "🐉U7", "🐉T3", "🐉T4", "🐉T7"], &[])),
    ("⛰️P0", (&["⛰️P1", "⛰️P2", "⛰️P3"], &[])),
    (
        "⛰️P1",
        (
            &["⛰️P2", "⛰️P5", "⛰️P6", "⛰️P0", "🐦‍🔥P4", "🐦‍🔥P7", "🐦‍🔥Q3"],
            &[],
        ),
    ),
    ("⛰️P2", (&["⛰️P1", "⛰️P3", "⛰️P4", "⛰️P5", "⛰️P0"], &[])),
    ("⛰️P3", (&["⛰️P2", "⛰️P4", "⛰️P0", "⛰️Q1", "⛰️Q6"], &[])),
    (
        "⛰️P4",
        (
            &["⛰️P2", "⛰️P3", "⛰️P5", "⛰️P7", "⛰️Q6", "⛰️Q8", "📚U7"],
            &[],
        ),
    ),
    ("⛰️P5", (&["⛰️P1", "⛰️P2", "⛰️P4", "⛰️P6", "⛰️P7"], &[])),
    (
        "⛰️P6",
        (&["⛰️P8", "⛰️P1", "⛰️P5", "⛰️P7", "🐦‍🔥P7", "🐦‍🔥P8"], &[]),
    ),
    (
        "⛰️P7",
        (&["⛰️P8", "⛰️P4", "⛰️P5", "⛰️P6", "📚U7", "📚U4"], &[]),
    ),
    ("⛰️P8", (&["⛰️P6", "⛰️P7", "🐦‍🔥P8", "📚U4", "📚U3"], &[])),
    ("⛰️Q0", (&["⛰️Q1", "⛰️Q2", "⛰️Q3"], &[])),
    ("⛰️Q1", (&["⛰️Q2", "⛰️Q6", "⛰️Q0", "⛰️P3"], &[])),
    (
        "⛰️Q2",
        (&["⛰️Q1", "⛰️Q3", "⛰️Q4", "⛰️Q5", "⛰️Q6", "⛰️Q0"], &[]),
    ),
    (
        "⛰️Q3",
        (&["⛰️Q2", "⛰️Q4", "⛰️Q0", "⛰️R6", "⛰️R8", "⛰️R0"], &[]),
    ),
    ("⛰️Q4", (&["⛰️Q2", "⛰️Q3", "⛰️Q5", "⛰️Q7", "⛰️R8"], &[])),
    ("⛰️Q5", (&["⛰️Q2", "⛰️Q4", "⛰️Q6", "⛰️Q7", "⛰️Q8"], &[])),
    (
        "⛰️Q6",
        (&["⛰️Q8", "⛰️Q1", "⛰️Q2", "⛰️Q5", "⛰️P3", "⛰️P4"], &[]),
    ),
    ("⛰️Q7", (&["⛰️Q8", "⛰️Q4", "⛰️Q5", "⛰️R8"], &[])),
    ("⛰️Q8", (&["⛰️Q5", "⛰️Q6", "⛰️Q7", "⛰️P4", "📚U7"], &[])),
    (
        "⛰️R0",
        (
            &[
                "⛰️R5", "⛰️R6", "⛰️R0", "⛰️Q3", "⛰️S8", "🐑P1", "⛰️R7", "⛰️S4", "⛰️T8", "⛰️S0",
            ],
            &[],
        ),
    ),
    ("⛰️R5", (&["⛰️R6", "⛰️R7", "⛰️R0"], &[])),
    ("⛰️R6", (&["⛰️R8", "⛰️R5", "⛰️R7", "⛰️Q3", "⛰️R0"], &[])),
    ("⛰️R7", (&["⛰️R8", "⛰️R5", "⛰️R6", "⛰️R0"], &[])),
    ("⛰️R8", (&["⛰️R6", "⛰️R7", "⛰️Q3", "⛰️Q4", "⛰️Q7"], &[])),
    (
        "⛰️S0",
        (
            &[
                "⛰️S1", "⛰️S2", "⛰️S3", "⛰️S8", "⛰️S4", "⛰️S5", "⛰️S6", "⛰️R0",
            ],
            &["🐑T0", "🐑T1", "🐑T2", "🐑T3"],
        ),
    ),
    (
        "⛰️S1",
        (
            &["⛰️S8", "⛰️S2", "⛰️S6", "⛰️S0", "🐑P1", "🐑P8", "🐑Q3"],
            &["🐑T0", "🐑T1", "🐑T2", "🐑T3"],
        ),
    ),
    (
        "⛰️S2",
        (
            &["⛰️S1", "⛰️S3", "⛰️S5", "⛰️S6", "⛰️S0"],
            &["🐑T0", "🐑T1", "🐑T2", "🐑T3"],
        ),
    ),
    (
        "⛰️S3",
        (
            &["⛰️S2", "⛰️S4", "⛰️S5", "⛰️S0", "⛰️T1", "⛰️T6"],
            &["🐑T0", "🐑T1", "🐑T2", "🐑T3"],
        ),
    ),
    (
        "⛰️S4",
        (
            &["⛰️S3", "⛰️S5", "⛰️T6", "⛰️T8", "⛰️R0", "⛰️S0"],
            &["🐑T0", "🐑T1", "🐑T2", "🐑T3"],
        ),
    ),
    (
        "⛰️S5",
        (
            &["⛰️S2", "⛰️S3", "⛰️S4", "⛰️S6", "⛰️S0"],
            &["🐑T0", "🐑T1", "🐑T2", "🐑T3"],
        ),
    ),
    (
        "⛰️S6",
        (
            &["⛰️S1", "⛰️S2", "⛰️S5", "⛰️S8", "⛰️S0"],
            &["🐑T0", "🐑T1", "🐑T2", "🐑T3"],
        ),
    ),
    (
        "⛰️S8",
        (
            &["⛰️S1", "⛰️S6", "🐑P1", "⛰️R0", "⛰️S0"],
            &["🐑T0", "🐑T1", "🐑T2", "🐑T3"],
        ),
    ),
    (
        "⛰️T0",
        (&["⛰️T1", "⛰️T2", "⛰️T3"], &["🐑T0", "🐑T1", "🐑T2", "🐑T3"]),
    ),
    (
        "⛰️T1",
        (
            &["⛰️T2", "⛰️T4", "⛰️T5", "⛰️T6", "⛰️T0", "⛰️S3"],
            &["🐑T0", "🐑T1", "🐑T2", "🐑T3"],
        ),
    ),
    (
        "⛰️T2",
        (
            &["⛰️T1", "⛰️T3", "⛰️T4", "⛰️T0"],
            &["🐑T0", "🐑T1", "🐑T2", "🐑T3"],
        ),
    ),
    (
        "⛰️T3",
        (
            &["⛰️T2", "⛰️T4", "⛰️T0", "⛰️U1"],
            &["🐑T0", "🐑T1", "🐑T2", "🐑T3"],
        ),
    ),
    ("⛰️T4", (&["⛰️T1", "⛰️T2", "⛰️T3", "⛰️T5", "⛰️U1"], &[])),
    (
        "⛰️T5",
        (
            &["⛰️T1", "⛰️T4", "⛰️T6", "⛰️T7", "⛰️T8", "⛰️U1", "⛰️U8"],
            &[],
        ),
    ),
    ("⛰️T6", (&["⛰️T8", "⛰️T1", "⛰️T5", "⛰️S3", "⛰️S4"], &[])),
    ("⛰️T7", (&["⛰️T8", "⛰️T5", "⛰️U8"], &[])),
    (
        "⛰️T8",
        (
            &["⛰️T5", "⛰️T6", "⛰️T7", "⛰️S4", "⛰️R0"],
            &["🐑T0", "🐑T1", "🐑T2", "🐑T3"],
        ),
    ),
    (
        "⛰️U0",
        (&["⛰️U1", "⛰️U2", "⛰️U3"], &["🐑T0", "🐑T1", "🐑T2", "🐑T3"]),
    ),
    (
        "⛰️U1",
        (
            &[
                "⛰️U8", "⛰️U2", "⛰️U5", "⛰️U7", "⛰️U0", "⛰️T3", "⛰️T4", "⛰️T5",
            ],
            &["🐑T0", "🐑T1", "🐑T2", "🐑T3"],
        ),
    ),
    (
        "⛰️U2",
        (
            &["⛰️U1", "⛰️U3", "⛰️U4", "⛰️U5", "⛰️U0"],
            &["🐑T0", "🐑T1", "🐑T2", "🐑T3"],
        ),
    ),
    (
        "⛰️U3",
        (
            &["⛰️U2", "⛰️U4", "⛰️U0", "📚S8", "📚R7", "📚Q8"],
            &["🐑T0", "🐑T1", "🐑T2", "🐑T3"],
        ),
    ),
    (
        "⛰️U4",
        (&["⛰️U2", "⛰️U3", "⛰️U5", "⛰️U6", "📚S8", "📚S6"], &[]),
    ),
    ("⛰️U5", (&["⛰️U1", "⛰️U2", "⛰️U4", "⛰️U6", "⛰️U7"], &[])),
    ("⛰️U6", (&["⛰️U4", "⛰️U5", "⛰️U7", "📚S6", "📚T8"], &[])),
    ("⛰️U7", (&["⛰️U8", "⛰️U1", "⛰️U5", "⛰️U6"], &[])),
    ("⛰️U8", (&["⛰️U1", "⛰️U7", "⛰️T5", "⛰️T7"], &[])),
    (
        "🎈P0",
        (&["🎈P1", "🎈P2", "🎈P3"], &["🐑Q0", "🐑Q1", "🐑Q2", "🐑Q3"]),
    ),
    (
        "🎈P1",
        (
            &["🎈P2", "🎈P4", "🎈P5", "🎈P6", "🎈P0", "📚P7", "📚Q3"],
            &["🐑Q0", "🐑Q1", "🐑Q2", "🐑Q3"],
        ),
    ),
    (
        "🎈P2",
        (
            &["🎈P1", "🎈P3", "🎈P4", "🎈P0"],
            &["🐑Q0", "🐑Q1", "🐑Q2", "🐑Q3"],
        ),
    ),
    (
        "🎈P3",
        (
            &["🎈P2", "🎈P4", "🎈P0", "🎈Q1", "🎈Q6"],
            &["🐑Q0", "🐑Q1", "🐑Q2", "🐑Q3"],
        ),
    ),
    (
        "🎈P4",
        (
            &["🎈P1", "🎈P2", "🎈P3", "🎈P5", "🎈P7", "🎈Q6", "🎈Q8"],
            &[],
        ),
    ),
    ("🎈P5", (&["🎈P1", "🎈P4", "🎈P6", "🎈P7"], &[])),
    (
        "🎈P6",
        (&["🎈P8", "🎈P1", "🎈P5", "🎈P7", "📚P7", "📚P8"], &[]),
    ),
    (
        "🎈P7",
        (
            &["🎈P8", "🎈P4", "🎈P5", "🎈P6", "🎈Q8", "🍌U6", "🍌U4"],
            &[],
        ),
    ),
    ("🎈P8", (&["🎈P6", "🎈P7", "📚P8", "🍌U4", "🍌U3"], &[])),
    (
        "🎈Q0",
        (&["🎈Q1", "🎈Q2", "🎈Q3"], &["🐑Q0", "🐑Q1", "🐑Q2", "🐑Q3"]),
    ),
    (
        "🎈Q1",
        (
            &["🎈Q2", "🎈Q5", "🎈Q6", "🎈Q0", "🎈P3"],
            &["🐑Q0", "🐑Q1", "🐑Q2", "🐑Q3"],
        ),
    ),
    (
        "🎈Q2",
        (
            &["🎈Q1", "🎈Q3", "🎈Q4", "🎈Q5", "🎈Q0"],
            &["🐑Q0", "🐑Q1", "🐑Q2", "🐑Q3"],
        ),
    ),
    (
        "🎈Q3",
        (
            &["🎈Q2", "🎈Q4", "🎈Q0", "🎈R1"],
            &["🐑Q0", "🐑Q1", "🐑Q2", "🐑Q3"],
        ),
    ),
    (
        "🎈Q4",
        (&["🎈Q2", "🎈Q3", "🎈Q5", "🎈Q7", "🎈Q8", "🎈R1"], &[]),
    ),
    ("🎈Q5", (&["🎈Q1", "🎈Q2", "🎈Q4", "🎈Q6", "🎈Q8"], &[])),
    ("🎈Q6", (&["🎈Q8", "🎈Q1", "🎈Q5", "🎈P3", "🎈P4"], &[])),
    ("🎈Q7", (&["🎈Q8", "🎈Q4", "🎈R1", "🎈R8"], &[])),
    (
        "🎈Q8",
        (
            &["🎈Q4", "🎈Q5", "🎈Q6", "🎈Q7", "🎈P4", "🎈P7", "🍌U6"],
            &[],
        ),
    ),
    (
        "🎈R0",
        (&["🎈R1", "🎈R2", "🎈R3"], &["🐑Q0", "🐑Q1", "🐑Q2", "🐑Q3"]),
    ),
    (
        "🎈R1",
        (
            &[
                "🎈R8", "🎈R2", "🎈R5", "🎈R7", "🎈R0", "🎈Q3", "🎈Q4", "🎈Q7",
            ],
            &["🐑Q0", "🐑Q1", "🐑Q2", "🐑Q3"],
        ),
    ),
    (
        "🎈R2",
        (
            &["🎈R1", "🎈R3", "🎈R4", "🎈R5", "🎈R0"],
            &["🐑Q0", "🐑Q1", "🐑Q2", "🐑Q3"],
        ),
    ),
    (
        "🎈R3",
        (
            &["🎈R2", "🎈R4", "🎈R0", "🎈S8", "🐑T1"],
            &["🐑Q0", "🐑Q1", "🐑Q2", "🐑Q3"],
        ),
    ),
    (
        "🎈R4",
        (&["🎈R2", "🎈R3", "🎈R5", "🎈R6", "🎈S8", "🎈S7"], &[]),
    ),
    ("🎈R5", (&["🎈R1", "🎈R2", "🎈R4", "🎈R6", "🎈R7"], &[])),
    ("🎈R6", (&["🎈R4", "🎈R5", "🎈R7", "🎈S7", "🎈T8"], &[])),
    ("🎈R7", (&["🎈R8", "🎈R1", "🎈R5", "🎈R6"], &[])),
    ("🎈R8", (&["🎈R1", "🎈R7", "🎈Q7"], &[])),
    (
        "🎈S0",
        (&["🎈S1", "🎈S2", "🎈S3"], &["⚡P0", "⚡P1", "⚡P2", "⚡P3"]),
    ),
    (
        "🎈S1",
        (
            &["🎈S2", "🎈S4", "🎈S5", "🎈S6", "🎈S0", "🐑T8", "🐑U3"],
            &["⚡P0", "⚡P1", "⚡P2", "⚡P3"],
        ),
    ),
    (
        "🎈S2",
        (
            &["🎈S1", "🎈S3", "🎈S4", "🎈S0"],
            &["⚡P0", "⚡P1", "⚡P2", "⚡P3"],
        ),
    ),
    (
        "🎈S3",
        (
            &["🎈S2", "🎈S4", "🎈S0", "🎈T1", "🎈T7"],
            &["⚡P0", "⚡P1", "⚡P2", "⚡P3"],
        ),
    ),
    (
        "🎈S4",
        (
            &["🎈S1", "🎈S2", "🎈S3", "🎈S5", "🎈S7", "🎈T7", "🎈T8"],
            &[],
        ),
    ),
    ("🎈S5", (&["🎈S1", "🎈S4", "🎈S6", "🎈S7"], &[])),
    (
        "🎈S6",
        (&["🎈S8", "🎈S1", "🎈S5", "🎈S7", "🐑T1", "🐑T8"], &[]),
    ),
    (
        "🎈S7",
        (
            &["🎈S8", "🎈S4", "🎈S5", "🎈S6", "🎈T8", "🎈R4", "🎈R6"],
            &[],
        ),
    ),
    ("🎈S8", (&["🎈S6", "🎈S7", "🎈R3", "🎈R4", "🐑T1"], &[])),
    (
        "🎈T0",
        (&["🎈T1", "🎈T2", "🎈T3"], &["⚡P0", "⚡P1", "⚡P2", "⚡P3"]),
    ),
    (
        "🎈T1",
        (
            &["🎈T2", "🎈T5", "🎈T7", "🎈T0", "🎈S3"],
            &["⚡P0", "⚡P1", "⚡P2", "⚡P3"],
        ),
    ),
    (
        "🎈T2",
        (
            &["🎈T1", "🎈T3", "🎈T5", "🎈T0"],
            &["⚡P0", "⚡P1", "⚡P2", "⚡P3"],
        ),
    ),
    (
        "🎈T3",
        (
            &["🎈T2", "🎈T4", "🎈T5", "🎈T0", "🎈U1", "🎈U6"],
            &["⚡P0", "⚡P1", "⚡P2", "⚡P3"],
        ),
    ),
    (
        "🎈T4",
        (&["🎈T3", "🎈T5", "🎈T6", "🎈T7", "🎈U6", "🎈U8"], &[]),
    ),
    ("🎈T5", (&["🎈T1", "🎈T2", "🎈T3", "🎈T4", "🎈T7"], &[])),
    ("🎈T6", (&["🎈T8", "🎈T4", "🎈T7", "🎈U8"], &[])),
    (
        "🎈T7",
        (
            &["🎈T1", "🎈T4", "🎈T5", "🎈T6", "🎈T8", "🎈S3", "🎈S4"],
            &[],
        ),
    ),
    ("🎈T8", (&["🎈T6", "🎈T7", "🎈S4", "🎈S7", "🎈R6"], &[])),
    (
        "🎈U0",
        (&["🎈U1", "🎈U2", "🎈U3"], &["⚡P0", "⚡P1", "⚡P2", "⚡P3"]),
    ),
    (
        "🎈U1",
        (
            &["🎈U2", "🎈U5", "🎈U6", "🎈U0", "🎈T3"],
            &["⚡P0", "⚡P1", "⚡P2", "⚡P3"],
        ),
    ),
    (
        "🎈U2",
        (
            &["🎈U1", "🎈U3", "🎈U4", "🎈U5", "🎈U0"],
            &["⚡P0", "⚡P1", "⚡P2", "⚡P3"],
        ),
    ),
    (
        "🎈U3",
        (
            &["🎈U2", "🎈U4", "🎈U0", "🍌S8", "🍌R4", "🍌Q8"],
            &["⚡P0", "⚡P1", "⚡P2", "⚡P3"],
        ),
    ),
    (
        "🎈U4",
        (
            &["🎈U2", "🎈U3", "🎈U5", "🎈U7", "🎈U8", "🍌S8", "🍌S7"],
            &[],
        ),
    ),
    ("🎈U5", (&["🎈U1", "🎈U2", "🎈U4", "🎈U6", "🎈U8"], &[])),
    ("🎈U6", (&["🎈U8", "🎈U1", "🎈U5", "🎈T3", "🎈T4"], &[])),
    ("🎈U7", (&["🎈U8", "🎈U4", "🍌S7", "🍌T8"], &[])),
    (
        "🎈U8",
        (&["🎈U4", "🎈U5", "🎈U6", "🎈U7", "🎈T4", "🎈T6"], &[]),
    ),
    (
        "⚡P0",
        (
            &["⚡P1", "⚡P2", "⚡P3"],
            &[
                "🎈S0", "🎈S1", "🎈S2", "🎈S3", "🎈T0", "🎈T1", "🎈T2", "🎈T3", "🎈U0", "🎈U1",
                "🎈U2", "🎈U3",
            ],
        ),
    ),
    (
        "⚡P1",
        (
            &["⚡P2", "⚡P4", "⚡P5", "⚡P6", "⚡P0", "🥯S8", "🥯R3"],
            &[
                "🎈S0", "🎈S1", "🎈S2", "🎈S3", "🎈T0", "🎈T1", "🎈T2", "🎈T3", "🎈U0", "🎈U1",
                "🎈U2", "🎈U3",
            ],
        ),
    ),
    (
        "⚡P2",
        (
            &["⚡P1", "⚡P3", "⚡P4", "⚡P0"],
            &[
                "🎈S0", "🎈S1", "🎈S2", "🎈S3", "🎈T0", "🎈T1", "🎈T2", "🎈T3", "🎈U0", "🎈U1",
                "🎈U2", "🎈U3",
            ],
        ),
    ),
    (
        "⚡P3",
        (
            &["⚡P2", "⚡P4", "⚡P0", "⚡R7", "⚡R8", "🐑U1"],
            &[
                "🎈S0", "🎈S1", "🎈S2", "🎈S3", "🎈T0", "🎈T1", "🎈T2", "🎈T3", "🎈U0", "🎈U1",
                "🎈U2", "🎈U3",
            ],
        ),
    ),
    (
        "⚡P4",
        (&["⚡P1", "⚡P2", "⚡P3", "⚡P5", "⚡P7", "⚡R7"], &[]),
    ),
    ("⚡P5", (&["⚡P1", "⚡P4", "⚡P6", "⚡P7"], &[])),
    (
        "⚡P6",
        (&["⚡P8", "⚡P1", "⚡P5", "⚡P7", "🥯S8", "🥯S1"], &[]),
    ),
    (
        "⚡P7",
        (
            &[
                "⚡P8", "⚡P4", "⚡P5", "⚡P6", "⚡Q7", "⚡Q5", "⚡Q4", "⚡Q3", "⚡R7",
            ],
            &[],
        ),
    ),
    ("⚡P8", (&["⚡P6", "⚡P7", "⚡Q3", "🥯S1"], &[])),
    (
        "⚡Q0",
        (
            &["⚡Q1", "⚡Q2", "⚡Q3"],
            &[
                "🤠P0", "🤠P1", "🤠P2", "🤠P3", "🤠Q0", "🤠Q1", "🤠Q2", "🤠Q3", "🤠R0", "🤠R1",
                "🤠R2", "🤠R3", "🐺Q0", "🐺Q1", "🐺Q2", "🐺Q3",
            ],
        ),
    ),
    (
        "⚡Q1",
        (
            &[
                "⚡Q2", "⚡Q4", "⚡Q5", "⚡Q6", "⚡Q0", "⚡S6", "⚡S8", "⚡T3",
            ],
            &[
                "🤠P0", "🤠P1", "🤠P2", "🤠P3", "🤠Q0", "🤠Q1", "🤠Q2", "🤠Q3", "🤠R0", "🤠R1",
                "🤠R2", "🤠R3", "🐺Q0", "🐺Q1", "🐺Q2", "🐺Q3",
            ],
        ),
    ),
    (
        "⚡Q2",
        (
            &["⚡Q1", "⚡Q3", "⚡Q4", "⚡Q0"],
            &[
                "🤠P0", "🤠P1", "🤠P2", "🤠P3", "🤠Q0", "🤠Q1", "🤠Q2", "🤠Q3", "🤠R0", "🤠R1",
                "🤠R2", "🤠R3", "🐺Q0", "🐺Q1", "🐺Q2", "🐺Q3",
            ],
        ),
    ),
    (
        "⚡Q3",
        (
            &["⚡Q2", "⚡Q4", "⚡Q0", "⚡P7", "⚡P8", "🥯S1"],
            &[
                "🤠P0", "🤠P1", "🤠P2", "🤠P3", "🤠Q0", "🤠Q1", "🤠Q2", "🤠Q3", "🤠R0", "🤠R1",
                "🤠R2", "🤠R3", "🐺Q0", "🐺Q1", "🐺Q2", "🐺Q3",
            ],
        ),
    ),
    ("⚡Q4", (&["⚡Q1", "⚡Q2", "⚡Q3", "⚡Q5", "⚡P7"], &[])),
    (
        "⚡Q5",
        (&["⚡Q1", "⚡Q4", "⚡Q6", "⚡Q7", "⚡Q8", "⚡P7"], &[]),
    ),
    ("⚡Q6", (&["⚡Q8", "⚡Q1", "⚡Q5", "⚡S6"], &[])),
    (
        "⚡Q7",
        (
            &["⚡Q8", "⚡Q5", "⚡P7", "⚡R7", "⚡R5", "⚡R4", "⚡R3"],
            &[],
        ),
    ),
    (
        "⚡Q8",
        (&["⚡Q5", "⚡Q6", "⚡Q7", "⚡R3", "⚡S1", "⚡S6"], &[]),
    ),
    ("⚡R0", (&["⚡R1", "⚡R2", "⚡R3"], &[])),
    (
        "⚡R1",
        (
            &["⚡R2", "⚡R4", "⚡R5", "⚡R6", "⚡R0", "🐑U8", "🐑S3"],
            &[],
        ),
    ),
    ("⚡R2", (&["⚡R1", "⚡R3", "⚡R4", "⚡R0"], &[])),
    (
        "⚡R3",
        (&["⚡R2", "⚡R4", "⚡R0", "⚡Q7", "⚡Q8", "⚡S1"], &[]),
    ),
    ("⚡R4", (&["⚡R1", "⚡R2", "⚡R3", "⚡R5", "⚡Q7"], &[])),
    (
        "⚡R5",
        (&["⚡R1", "⚡R4", "⚡R6", "⚡R7", "⚡R8", "⚡Q7"], &[]),
    ),
    ("⚡R6", (&["⚡R8", "⚡R1", "⚡R5", "🐑U8", "🐑U6"], &[])),
    (
        "⚡R7",
        (&["⚡R8", "⚡R5", "⚡Q7", "⚡P7", "⚡P4", "⚡P3"], &[]),
    ),
    (
        "⚡R8",
        (&["⚡R5", "⚡R6", "⚡R7", "⚡P3", "🐑U6", "🐑U1"], &[]),
    ),
    ("⚡S0", (&["⚡S1", "⚡S2", "⚡S3"], &[])),
    (
        "⚡S1",
        (&["⚡S2", "⚡S5", "⚡S6", "⚡S0", "⚡Q8", "⚡R3"], &[]),
    ),
    ("⚡S2", (&["⚡S1", "⚡S3", "⚡S4", "⚡S5", "⚡S0"], &[])),
    ("⚡S3", (&["⚡S2", "⚡S4", "⚡S0", "⚡U8"], &[])),
    (
        "⚡S4",
        (
            &["⚡S2", "⚡S3", "⚡S5", "⚡S7", "⚡S8", "⚡U7", "⚡U8"],
            &[],
        ),
    ),
    ("⚡S5", (&["⚡S1", "⚡S2", "⚡S4", "⚡S6", "⚡S8"], &[])),
    (
        "⚡S6",
        (&["⚡S8", "⚡S1", "⚡S5", "⚡Q8", "⚡Q6", "⚡Q1"], &[]),
    ),
    ("⚡S7", (&["⚡S8", "⚡S4", "⚡T4", "⚡U4", "⚡U7"], &[])),
    (
        "⚡S8",
        (
            &["⚡S4", "⚡S5", "⚡S6", "⚡S7", "⚡T4", "⚡T3", "⚡Q1"],
            &[],
        ),
    ),
    (
        "⚡T0",
        (
            &["⚡T1", "⚡T2", "⚡T3"],
            &[
                "🥯S0", "🥯S1", "🥯S2", "🥯S3", "🥯T0", "🥯T1", "🥯T2", "🥯T3", "🥯U0", "🥯U1",
                "🥯U2", "🥯U3",
            ],
        ),
    ),
    (
        "⚡T1",
        (
            &["⚡T8", "⚡T2", "⚡T6", "⚡T0", "🤠S8", "🤠S6", "🤠R3"],
            &[
                "🥯S0", "🥯S1", "🥯S2", "🥯S3", "🥯T0", "🥯T1", "🥯T2", "🥯T3", "🥯U0", "🥯U1",
                "🥯U2", "🥯U3",
            ],
        ),
    ),
    (
        "⚡T2",
        (
            &["⚡T1", "⚡T3", "⚡T5", "⚡T6", "⚡T0"],
            &[
                "🥯S0", "🥯S1", "🥯S2", "🥯S3", "🥯T0", "🥯T1", "🥯T2", "🥯T3", "🥯U0", "🥯U1",
                "🥯U2", "🥯U3",
            ],
        ),
    ),
    (
        "⚡T3",
        (
            &["⚡T2", "⚡T4", "⚡T5", "⚡T0", "⚡S8", "⚡Q1"],
            &[
                "🥯S0", "🥯S1", "🥯S2", "🥯S3", "🥯T0", "🥯T1", "🥯T2", "🥯T3", "🥯U0", "🥯U1",
                "🥯U2", "🥯U3",
            ],
        ),
    ),
    (
        "⚡T4",
        (&["⚡T3", "⚡T5", "⚡T7", "⚡S7", "⚡S8", "⚡U4"], &[]),
    ),
    ("⚡T5", (&["⚡T2", "⚡T3", "⚡T4", "⚡T6", "⚡T7"], &[])),
    ("⚡T6", (&["⚡T1", "⚡T2", "⚡T5", "⚡T7", "⚡T8"], &[])),
    ("⚡T7", (&["⚡T8", "⚡T4", "⚡T5", "⚡T6", "⚡U4"], &[])),
    ("⚡T8", (&["⚡T1", "⚡T6", "⚡T7", "🤠S6", "🤠S1"], &[])),
    (
        "⚡U0",
        (
            &[],
            &[
                "🍪Q0", "🍪Q1", "🍪Q2", "🍪Q3", "🍪R0", "🍪R1", "🍪R2", "🍪R3", "🍕Q0", "🍕Q1",
                "🍕Q2", "🍕Q3",
            ],
        ),
    ),
    ("⚡U4", (&["⚡U5", "⚡U7", "⚡T4", "⚡T7", "⚡S7"], &[])),
    ("⚡U5", (&["⚡U4", "⚡U6", "⚡U7"], &[])),
    ("⚡U6", (&["⚡U8", "⚡U5", "⚡U7", "🤖R7", "🤖R8"], &[])),
    (
        "⚡U7",
        (&["⚡U8", "⚡U4", "⚡U5", "⚡U6", "⚡S7", "⚡S4"], &[]),
    ),
    ("⚡U8", (&["⚡U6", "⚡U7", "⚡S4", "⚡S3", "🤖R7"], &[])),
    (
        "🤖Q0",
        (
            &[],
            &[
                "🐉P0", "🐉P1", "🐉P2", "🐉P3", "🐉Q0", "🐉Q1", "🐉Q2", "🐉Q3", "🐉R0", "🐉R1",
                "🐉R2", "🐉R3", "🍔Q0", "🍔Q1", "🍔Q2", "🍔Q3",
            ],
        ),
    ),
    ("🤖Q4", (&["🤖Q5", "🤖Q6"], &[])),
    ("🤖Q5", (&["🤖Q4", "🤖Q6", "🤖Q7"], &[])),
    ("🤖Q6", (&["🤖Q4", "🤖Q5", "🤖Q7", "🤖R6", "🤖R4"], &[])),
    ("🤖Q7", (&["🤖Q8", "🤖Q5", "🤖Q6", "🤖R4"], &[])),
    ("🤖Q8", (&["🤖Q7"], &[])),
    ("🤖R0", (&[], &[])),
    ("🤖R4", (&["🤖R5", "🤖R6", "🤖R7", "🤖Q6", "🤖Q7"], &[])),
    ("🤖R5", (&["🤖R4", "🤖R7"], &[])),
    ("🤖R6", (&["🤖R8", "🤖R4", "🤖R7", "🤖Q6"], &[])),
    (
        "🤖R7",
        (&["🤖R4", "🤖R5", "🤖R6", "🤖R8", "⚡U8", "⚡U6"], &[]),
    ),
    ("🤖R8", (&["🤖R6", "🤖R7", "⚡U6"], &[])),
    (
        "🐑P0",
        (
            &["🐑P1", "🐑P2", "🐑P3"],
            &[
                "🐉S0", "🐉S1", "🐉S2", "🐉S3", "🐉T0", "🐉T1", "🐉T2", "🐉T3", "🐉U0", "🐉U1",
                "🐉U2", "🐉U3",
            ],
        ),
    ),
    (
        "🐑P1",
        (
            &["🐑P8", "🐑P2", "🐑P6", "🐑P0", "⛰️S8", "⛰️S1", "⛰️R0"],
            &[
                "🐉S0", "🐉S1", "🐉S2", "🐉S3", "🐉T0", "🐉T1", "🐉T2", "🐉T3", "🐉U0", "🐉U1",
                "🐉U2", "🐉U3",
            ],
        ),
    ),
    (
        "🐑P2",
        (
            &["🐑P1", "🐑P3", "🐑P5", "🐑P6", "🐑P0"],
            &[
                "🐉S0", "🐉S1", "🐉S2", "🐉S3", "🐉T0", "🐉T1", "🐉T2", "🐉T3", "🐉U0", "🐉U1",
                "🐉U2", "🐉U3",
            ],
        ),
    ),
    (
        "🐑P3",
        (
            &["🐑P2", "🐑P4", "🐑P5", "🐑P0", "🐑R7", "🐑R8"],
            &[
                "🐉S0", "🐉S1", "🐉S2", "🐉S3", "🐉T0", "🐉T1", "🐉T2", "🐉T3", "🐉U0", "🐉U1",
                "🐉U2", "🐉U3",
            ],
        ),
    ),
    (
        "🐑P4",
        (
            &["🐑P3", "🐑P5", "🐑P7", "🐑Q7", "🐑Q5", "🐑R6", "🐑R7"],
            &[],
        ),
    ),
    ("🐑P5", (&["🐑P2", "🐑P3", "🐑P4", "🐑P6", "🐑P7"], &[])),
    ("🐑P6", (&["🐑P1", "🐑P2", "🐑P5", "🐑P7", "🐑P8"], &[])),
    (
        "🐑P7",
        (
            &["🐑P8", "🐑P4", "🐑P5", "🐑P6", "🐑Q5", "🐑Q4", "🐑Q3"],
            &[],
        ),
    ),
    ("🐑P8", (&["🐑P1", "🐑P6", "🐑P7", "🐑Q3", "⛰️S1"], &[])),
    (
        "🐑Q0",
        (
            &["🐑Q1", "🐑Q2", "🐑Q3"],
            &[
                "🎈P0", "🎈P1", "🎈P2", "🎈P3", "🎈Q0", "🎈Q1", "🎈Q2", "🎈Q3", "🎈R0", "🎈R1",
                "🎈R2", "🎈R3", "📚Q0", "📚Q1", "📚Q2", "📚Q3",
            ],
        ),
    ),
    (
        "🐑Q1",
        (
            &["🐑Q2", "🐑Q4", "🐑Q5", "🐑Q6", "🐑Q0", "🐑S8", "🐑T3"],
            &[
                "🎈P0", "🎈P1", "🎈P2", "🎈P3", "🎈Q0", "🎈Q1", "🎈Q2", "🎈Q3", "🎈R0", "🎈R1",
                "🎈R2", "🎈R3", "📚Q0", "📚Q1", "📚Q2", "📚Q3",
            ],
        ),
    ),
    (
        "🐑Q2",
        (
            &["🐑Q1", "🐑Q3", "🐑Q4", "🐑Q0"],
            &[
                "🎈P0", "🎈P1", "🎈P2", "🎈P3", "🎈Q0", "🎈Q1", "🎈Q2", "🎈Q3", "🎈R0", "🎈R1",
                "🎈R2", "🎈R3", "📚Q0", "📚Q1", "📚Q2", "📚Q3",
            ],
        ),
    ),
    (
        "🐑Q3",
        (
            &["🐑Q2", "🐑Q4", "🐑Q0", "🐑P7", "🐑P8", "⛰️S1"],
            &[
                "🎈P0", "🎈P1", "🎈P2", "🎈P3", "🎈Q0", "🎈Q1", "🎈Q2", "🎈Q3", "🎈R0", "🎈R1",
                "🎈R2", "🎈R3", "📚Q0", "📚Q1", "📚Q2", "📚Q3",
            ],
        ),
    ),
    ("🐑Q4", (&["🐑Q1", "🐑Q2", "🐑Q3", "🐑Q5", "🐑P7"], &[])),
    (
        "🐑Q5",
        (
            &["🐑Q1", "🐑Q4", "🐑Q6", "🐑Q7", "🐑Q8", "🐑P4", "🐑P7"],
            &[],
        ),
    ),
    ("🐑Q6", (&["🐑Q8", "🐑Q1", "🐑Q5", "🐑S6", "🐑S8"], &[])),
    ("🐑Q7", (&["🐑Q8", "🐑Q5", "🐑P4", "🐑R6", "🐑R4"], &[])),
    (
        "🐑Q8",
        (
            &["🐑Q5", "🐑Q6", "🐑Q7", "🐑R4", "🐑R3", "🐑S1", "🐑S6"],
            &[],
        ),
    ),
    ("🐑R0", (&["🐑R1", "🐑R2", "🐑R3"], &[])),
    ("🐑R1", (&["🐑R8", "🐑R2", "🐑R5", "🐑R7", "🐑R0"], &[])),
    ("🐑R2", (&["🐑R1", "🐑R3", "🐑R4", "🐑R5", "🐑R0"], &[])),
    ("🐑R3", (&["🐑R2", "🐑R4", "🐑R0", "🐑Q8", "🐑S1"], &[])),
    (
        "🐑R4",
        (&["🐑R2", "🐑R3", "🐑R5", "🐑R6", "🐑Q7", "🐑Q8"], &[]),
    ),
    ("🐑R5", (&["🐑R1", "🐑R2", "🐑R4", "🐑R6", "🐑R7"], &[])),
    ("🐑R6", (&["🐑R4", "🐑R5", "🐑R7", "🐑Q7", "🐑P4"], &[])),
    (
        "🐑R7",
        (&["🐑R8", "🐑R1", "🐑R5", "🐑R6", "🐑P4", "🐑P3"], &[]),
    ),
    ("🐑R8", (&["🐑R1", "🐑R7", "🐑P3"], &[])),
    ("🐑S0", (&["🐑S1", "🐑S2", "🐑S3"], &[])),
    (
        "🐑S1",
        (
            &["🐑S2", "🐑S4", "🐑S5", "🐑S6", "🐑S0", "🐑Q8", "🐑R3"],
            &[],
        ),
    ),
    ("🐑S2", (&["🐑S1", "🐑S3", "🐑S4", "🐑S0"], &[])),
    (
        "🐑S3",
        (&["🐑S2", "🐑S4", "🐑S0", "🐑U7", "🐑U8", "⚡R1"], &[]),
    ),
    (
        "🐑S4",
        (&["🐑S1", "🐑S2", "🐑S3", "🐑S5", "🐑S7", "🐑U7"], &[]),
    ),
    ("🐑S5", (&["🐑S1", "🐑S4", "🐑S6", "🐑S7"], &[])),
    (
        "🐑S6",
        (&["🐑S8", "🐑S1", "🐑S5", "🐑S7", "🐑Q8", "🐑Q6"], &[]),
    ),
    (
        "🐑S7",
        (
            &["🐑S8", "🐑S4", "🐑S5", "🐑S6", "🐑T6", "🐑T4", "🐑U7"],
            &[],
        ),
    ),
    (
        "🐑S8",
        (&["🐑S6", "🐑S7", "🐑T4", "🐑T3", "🐑Q6", "🐑Q1"], &[]),
    ),
    (
        "🐑T0",
        (
            &["🐑T1", "🐑T2", "🐑T3"],
            &[
                "⛰️S0", "⛰️S1", "⛰️S2", "⛰️S3", "⛰️T0", "⛰️T1", "⛰️T2", "⛰️T3", "⛰️U0", "⛰️U1",
                "⛰️U2", "⛰️U3", "⛰️S8", "⛰️S4", "⛰️T8", "⛰️S5", "⛰️S6",
            ],
        ),
    ),
    (
        "🐑T1",
        (
            &[
                "🐑T8", "🐑T2", "🐑T5", "🐑T7", "🐑T0", "🎈S8", "🎈S6", "🎈R3",
            ],
            &[
                "⛰️S0", "⛰️S1", "⛰️S2", "⛰️S3", "⛰️T0", "⛰️T1", "⛰️T2", "⛰️T3", "⛰️U0", "⛰️U1",
                "⛰️U2", "⛰️U3", "⛰️S8", "⛰️S4", "⛰️T8", "⛰️S5", "⛰️S6",
            ],
        ),
    ),
    (
        "🐑T2",
        (
            &["🐑T1", "🐑T3", "🐑T4", "🐑T5", "🐑T0"],
            &[
                "⛰️S0", "⛰️S1", "⛰️S2", "⛰️S3", "⛰️T0", "⛰️T1", "⛰️T2", "⛰️T3", "⛰️U0", "⛰️U1",
                "⛰️U2", "⛰️U3", "⛰️S8", "⛰️S4", "⛰️T8", "⛰️S5", "⛰️S6",
            ],
        ),
    ),
    (
        "🐑T3",
        (
            &["🐑T2", "🐑T4", "🐑T0", "🐑S8", "🐑Q1"],
            &[
                "⛰️S0", "⛰️S1", "⛰️S2", "⛰️S3", "⛰️T0", "⛰️T1", "⛰️T2", "⛰️T3", "⛰️U0", "⛰️U1",
                "⛰️U2", "⛰️U3", "⛰️S8", "⛰️S4", "⛰️T8", "⛰️S5", "⛰️S6",
            ],
        ),
    ),
    (
        "🐑T4",
        (&["🐑T2", "🐑T3", "🐑T5", "🐑T6", "🐑S7", "🐑S8"], &[]),
    ),
    ("🐑T5", (&["🐑T1", "🐑T2", "🐑T4", "🐑T6", "🐑T7"], &[])),
    (
        "🐑T6",
        (&["🐑T4", "🐑T5", "🐑T7", "🐑S7", "🐑U7", "🐑U5"], &[]),
    ),
    (
        "🐑T7",
        (
            &["🐑T8", "🐑T1", "🐑T5", "🐑T6", "🐑U5", "🐑U4", "🐑U3"],
            &[],
        ),
    ),
    ("🐑T8", (&["🐑T1", "🐑T7", "🐑U3", "🎈S6", "🎈S1"], &[])),
    (
        "🐑U0",
        (
            &["🐑U1", "🐑U2", "🐑U3"],
            &[
                "🥯P0", "🥯P1", "🥯P2", "🥯P3", "🥯Q0", "🥯Q1", "🥯Q2", "🥯Q3", "🥯R0", "🥯R1",
                "🥯R2", "🥯R3", "🍌Q0", "🍌Q1", "🍌Q2", "🍌Q3",
            ],
        ),
    ),
    (
        "🐑U1",
        (
            &["🐑U2", "🐑U4", "🐑U5", "🐑U6", "🐑U0", "⚡R8", "⚡P3"],
            &[
                "🥯P0", "🥯P1", "🥯P2", "🥯P3", "🥯Q0", "🥯Q1", "🥯Q2", "🥯Q3", "🥯R0", "🥯R1",
                "🥯R2", "🥯R3", "🍌Q0", "🍌Q1", "🍌Q2", "🍌Q3",
            ],
        ),
    ),
    (
        "🐑U2",
        (
            &["🐑U1", "🐑U3", "🐑U4", "🐑U0"],
            &[
                "🥯P0", "🥯P1", "🥯P2", "🥯P3", "🥯Q0", "🥯Q1", "🥯Q2", "🥯Q3", "🥯R0", "🥯R1",
                "🥯R2", "🥯R3", "🍌Q0", "🍌Q1", "🍌Q2", "🍌Q3",
            ],
        ),
    ),
    (
        "🐑U3",
        (
            &["🐑U2", "🐑U4", "🐑U0", "🐑T7", "🐑T8", "🎈S1"],
            &[
                "🥯P0", "🥯P1", "🥯P2", "🥯P3", "🥯Q0", "🥯Q1", "🥯Q2", "🥯Q3", "🥯R0", "🥯R1",
                "🥯R2", "🥯R3", "🍌Q0", "🍌Q1", "🍌Q2", "🍌Q3",
            ],
        ),
    ),
    ("🐑U4", (&["🐑U1", "🐑U2", "🐑U3", "🐑U5", "🐑T7"], &[])),
    (
        "🐑U5",
        (
            &["🐑U1", "🐑U4", "🐑U6", "🐑U7", "🐑U8", "🐑T6", "🐑T7"],
            &[],
        ),
    ),
    ("🐑U6", (&["🐑U8", "🐑U1", "🐑U5", "⚡R6", "⚡R8"], &[])),
    (
        "🐑U7",
        (&["🐑U8", "🐑U5", "🐑T6", "🐑S7", "🐑S4", "🐑S3"], &[]),
    ),
    (
        "🐑U8",
        (&["🐑U5", "🐑U6", "🐑U7", "🐑S3", "⚡R1", "⚡R6"], &[]),
    ),
    ("👻X0", (&["👻X1", "👻X2", "👻X3"], &[])),
    (
        "👻X1",
        (&["👻X2", "👻X5", "👻X6", "👻X0", "👻T7", "👻U8"], &[]),
    ),
    ("👻X2", (&["👻X1", "👻X3", "👻X4", "👻X5", "👻X0"], &[])),
    ("👻X3", (&["👻X2", "👻X4", "👻X0", "🇲🇺Q4", "🇲🇺R8"], &[])),
    ("👻X4", (&["👻X2", "👻X3", "👻X5", "👻X7", "👻X8"], &[])),
    ("👻X5", (&["👻X1", "👻X2", "👻X4", "👻X6", "👻X8"], &[])),
    ("👻X6", (&["👻X8", "👻X1", "👻X5", "👻T8", "👻T7"], &[])),
    ("👻X7", (&["👻X8", "👻X4", "🇲🇺U8", "🇲🇺U7", "🇲🇺T4"], &[])),
    (
        "👻X8",
        (
            &["👻X4", "👻X5", "👻X6", "👻X7", "👻T8", "👻S4", "🇲🇺U7"],
            &[],
        ),
    ),
    ("👻Y0", (&["👻Y1", "👻Y2"], &[])),
    ("👻Y1", (&["👻Y2", "👻Y5", "👻Y6", "👻Y0", "👻Z3"], &[])),
    ("👻Y2", (&["👻Y1", "👻Y5", "👻Y0"], &[])),
    ("👻Y5", (&["👻Y1", "👻Y2", "👻Y6", "👻Y7"], &[])),
    (
        "👻Y6",
        (&["👻Y8", "👻Y1", "👻Y5", "👻Y7", "👻Z3", "👻Z4"], &[]),
    ),
    ("👻Y7", (&["👻Y8", "👻Y5", "👻Y6"], &[])),
    ("👻Y8", (&["👻Y6", "👻Y7", "👻Z4", "👻Z6"], &[])),
    ("👻Z0", (&["👻Z1", "👻Z2", "👻Z3"], &[])),
    (
        "👻Z1",
        (&["👻Z2", "👻Z5", "👻Z7", "👻Z0", "👻V6", "👻V7"], &[]),
    ),
    ("👻Z2", (&["👻Z1", "👻Z3", "👻Z5", "👻Z0"], &[])),
    (
        "👻Z3",
        (&["👻Z2", "👻Z4", "👻Z5", "👻Z0", "👻Y1", "👻Y6"], &[]),
    ),
    (
        "👻Z4",
        (&["👻Z3", "👻Z5", "👻Z6", "👻Z7", "👻Y6", "👻Y8"], &[]),
    ),
    ("👻Z5", (&["👻Z1", "👻Z2", "👻Z3", "👻Z4", "👻Z7"], &[])),
    ("👻Z6", (&["👻Z8", "👻Z4", "👻Z7", "👻Y8"], &[])),
    (
        "👻Z7",
        (
            &["👻Z1", "👻Z4", "👻Z5", "👻Z6", "👻Z8", "👻V7", "👻V8"],
            &[],
        ),
    ),
    ("👻Z8", (&["👻Z6", "👻Z7", "👻V8", "👻W1"], &[])),
    ("👻V0", (&["👻V1", "👻V2", "👻V3"], &[])),
    (
        "👻V1",
        (
            &[
                "👻V8", "👻V2", "👻V5", "👻V7", "👻V0", "👻W8", "👻W6", "👻W1", "👻N6",
            ],
            &[],
        ),
    ),
    ("👻V2", (&["👻V1", "👻V3", "👻V4", "👻V5", "👻V0"], &[])),
    ("👻V3", (&["👻V2", "👻V4", "👻V0"], &[])),
    ("👻V4", (&["👻V2", "👻V3", "👻V5", "👻V6"], &[])),
    ("👻V5", (&["👻V1", "👻V2", "👻V4", "👻V6", "👻V7"], &[])),
    ("👻V6", (&["👻V4", "👻V5", "👻V7", "👻Z1"], &[])),
    (
        "👻V7",
        (&["👻V8", "👻V1", "👻V5", "👻V6", "👻Z1", "👻Z7"], &[]),
    ),
    ("👻V8", (&["👻V1", "👻V7", "👻Z7", "👻Z8", "👻W1"], &[])),
    (
        "👻W0",
        (
            &["👻W1", "👻W2", "👻W3"],
            &[
                "🧩R0", "🧩R1", "🧩R2", "🧩R3", "🧩S0", "🧩S1", "🧩S2", "🧩S3", "🧩T0", "🧩T1",
                "🧩T2", "🧩T3", "🧩U0", "🧩U1", "🧩U2", "🧩U3",
            ],
        ),
    ),
    (
        "👻W1",
        (
            &["👻W2", "👻W6", "👻W0", "👻V1", "👻V8", "👻Z8"],
            &[
                "🧩R0", "🧩R1", "🧩R2", "🧩R3", "🧩S0", "🧩S1", "🧩S2", "🧩S3", "🧩T0", "🧩T1",
                "🧩T2", "🧩T3", "🧩U0", "🧩U1", "🧩U2", "🧩U3",
            ],
        ),
    ),
    (
        "👻W2",
        (
            &["👻W1", "👻W3", "👻W4", "👻W5", "👻W6", "👻W0"],
            &[
                "🧩R0", "🧩R1", "🧩R2", "🧩R3", "🧩S0", "🧩S1", "🧩S2", "🧩S3", "🧩T0", "🧩T1",
                "🧩T2", "🧩T3", "🧩U0", "🧩U1", "🧩U2", "🧩U3",
            ],
        ),
    ),
    (
        "👻W3",
        (
            &["👻W2", "👻W4", "👻W0"],
            &[
                "🧩R0", "🧩R1", "🧩R2", "🧩R3", "🧩S0", "🧩S1", "🧩S2", "🧩S3", "🧩T0", "🧩T1",
                "🧩T2", "🧩T3", "🧩U0", "🧩U1", "🧩U2", "🧩U3",
            ],
        ),
    ),
    ("👻W4", (&["👻W2", "👻W3", "👻W5", "👻W7"], &[])),
    ("👻W5", (&["👻W2", "👻W4", "👻W6", "👻W7", "👻W8"], &[])),
    ("👻W6", (&["👻W8", "👻W1", "👻W2", "👻W5", "👻V1"], &[])),
    ("👻W7", (&["👻W8", "👻W4", "👻W5", "👻N8", "👻K8"], &[])),
    (
        "👻W8",
        (&["👻W5", "👻W6", "👻W7", "👻V1", "👻N6", "👻N8"], &[]),
    ),
    ("👻N0", (&["👻N1", "👻N2", "👻N3"], &[])),
    ("👻N1", (&["👻N2", "👻N5", "👻N7", "👻N0", "👻K7"], &[])),
    ("👻N2", (&["👻N1", "👻N3", "👻N5", "👻N0"], &[])),
    ("👻N3", (&["👻N2", "👻N4", "👻N5", "👻N0"], &[])),
    ("👻N4", (&["👻N3", "👻N5", "👻N6", "👻N7"], &[])),
    ("👻N5", (&["👻N1", "👻N2", "👻N3", "👻N4", "👻N7"], &[])),
    ("👻N6", (&["👻N8", "👻N4", "👻N7", "👻W8", "👻V1"], &[])),
    (
        "👻N7",
        (
            &["👻N1", "👻N4", "👻N5", "👻N6", "👻N8", "👻K7", "👻K8"],
            &[],
        ),
    ),
    ("👻N8", (&["👻N6", "👻N7", "👻W8", "👻W7", "👻K8"], &[])),
    ("👻K0", (&["👻K1", "👻K2", "👻K3"], &[])),
    ("👻K1", (&["👻K2", "👻K4", "👻K5", "👻K6", "👻K0"], &[])),
    ("👻K2", (&["👻K1", "👻K3", "👻K4", "👻K0"], &[])),
    ("👻K3", (&["👻K2", "👻K4", "👻K0"], &[])),
    ("👻K4", (&["👻K1", "👻K2", "👻K3", "👻K5", "👻K7"], &[])),
    ("👻K5", (&["👻K1", "👻K4", "👻K6", "👻K7"], &[])),
    ("👻K6", (&["👻K8", "👻K1", "👻K5", "👻K7"], &[])),
    (
        "👻K7",
        (&["👻K8", "👻K4", "👻K5", "👻K6", "👻N1", "👻N7"], &[]),
    ),
    ("👻K8", (&["👻K6", "👻K7", "👻N7", "👻N8", "👻W7"], &[])),
];
