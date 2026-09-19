use std::{
    collections::{HashMap, HashSet},
    error::Error,
    rc::Rc,
};

use psi_map::{
    LayoutCache, Map,
    board::{Board, BoardEdgeLinkError, BoardKey, Distance},
    layout::{Edge, ValidatedLayoutData},
};

mod data;

struct RegionData<'a> {
    key: &'a str,
    p: &'a ValidatedLayoutData<'a>,
    q: &'a ValidatedLayoutData<'a>,
    r: &'a ValidatedLayoutData<'a>,
    s: &'a ValidatedLayoutData<'a>,
    t: &'a ValidatedLayoutData<'a>,
    u: &'a ValidatedLayoutData<'a>,
}

struct ContinentData<'a> {
    rim: [RegionData<'a>; 6],
    spokes: [RegionData<'a>; 6],
    hub: [RegionData<'a>; 3],
}

impl<'a> RegionData<'a> {
    fn keys(&'a self) -> RegionBoardKeys<'a> {
        RegionBoardKeys(self)
    }
}

struct RegionBoardKeys<'a>(&'a RegionData<'a>);

impl<'a> RegionBoardKeys<'a> {
    fn key(&self, board: char) -> BoardKey {
        BoardKey::from(format!("{}{}", self.0.key, board).as_str())
    }

    fn p(&self) -> BoardKey {
        self.key('P')
    }

    fn q(&self) -> BoardKey {
        self.key('Q')
    }

    fn r(&self) -> BoardKey {
        self.key('R')
    }

    fn s(&self) -> BoardKey {
        self.key('S')
    }

    fn t(&self) -> BoardKey {
        self.key('T')
    }

    fn u(&self) -> BoardKey {
        self.key('U')
    }

    fn parity_uq(&self, parity: bool) -> BoardKey {
        match parity {
            false => self.u(),
            true => self.q(),
        }
    }

    fn parity_tp(&self, parity: bool) -> BoardKey {
        match parity {
            false => self.t(),
            true => self.p(),
        }
    }

    fn iter_pqr(&self) -> impl Iterator<Item = BoardKey> {
        [self.p(), self.q(), self.r()].into_iter()
    }

    fn iter_stu(&self) -> impl Iterator<Item = BoardKey> {
        [self.s(), self.t(), self.u()].into_iter()
    }

    fn iter_prstu(&self) -> impl Iterator<Item = BoardKey> {
        [self.p(), self.r(), self.s(), self.t(), self.u()].into_iter()
    }
}

struct Region {
    p: Rc<Board>,
    q: Rc<Board>,
    r: Rc<Board>,
    s: Rc<Board>,
    t: Rc<Board>,
    u: Rc<Board>,
}

fn load_region<'a>(
    map: &Rc<Map>,
    lc: &LayoutCache<'a>,
    region: &'a RegionData,
    link: impl FnOnce(Region) -> Result<(), BoardEdgeLinkError>,
) -> Result<(), BoardEdgeLinkError> {
    let p = map.add_board(region.keys().p(), lc.layout(region.p));
    let q = map.add_board(region.keys().q(), lc.layout(region.q));
    let r = map.add_board(region.keys().r(), lc.layout(region.r));
    let s = map.add_board(region.keys().s(), lc.layout(region.s));
    let t = map.add_board(region.keys().t(), lc.layout(region.t));
    let u = map.add_board(region.keys().u(), lc.layout(region.u));
    link(Region { p, q, r, s, t, u })
}

fn cycle_double_skip<'a, I>(it: I, skip: usize) -> impl Iterator<Item = &'a RegionData<'a>>
where
    I: Iterator<Item = &'a RegionData<'a>> + Clone,
{
    it.cycle()
        .flat_map(|i| std::iter::repeat_n(i, 2))
        .skip(skip)
}

#[test]
fn adjacencies_144p_final() -> Result<(), Box<dyn Error>> {
    let lc = LayoutCache::new();
    let map = Map::new();

    for continent in [&data::BLUE, &data::ORANGE] {
        for region in continent.rim.iter() {
            load_region(&map, &lc, region, |Region { p, q, r, s, t, u }| {
                p.edge(Edge::Clock9).link(&q.edge(Edge::Clock9))?;
                q.edge(Edge::Clock6).link(&r.edge(Edge::Clock6))?;
                r.edge(Edge::Clock9).link(&s.edge(Edge::Clock3))?;
                s.edge(Edge::Clock9).link(&t.edge(Edge::Clock3))?;
                t.edge(Edge::Clock9).link(&u.edge(Edge::Clock3))?;
                Ok(())
            })?;
        }
        for region in continent.spokes.iter() {
            load_region(&map, &lc, region, |Region { p, q, r, s, t, u }| {
                // the left coastline
                p.edge(Edge::Clock9).link(&q.edge(Edge::Clock3))?;
                q.edge(Edge::Clock9).link(&r.edge(Edge::Clock3))?;
                // the right coastline
                s.edge(Edge::Clock9).link(&t.edge(Edge::Clock3))?;
                t.edge(Edge::Clock9).link(&u.edge(Edge::Clock3))?;
                // and link them
                r.edge(Edge::Clock9).link(&s.edge(Edge::Clock6))?;
                Ok(())
            })?;
        }
        for region in continent.hub.iter() {
            load_region(&map, &lc, region, |Region { p, q, r, s, t, u }| {
                // the left standard-3
                p.edge(Edge::Clock6).link(&q.edge(Edge::Clock9))?;
                q.edge(Edge::Clock6).link(&r.edge(Edge::Clock9))?;
                r.edge(Edge::Clock6).link(&p.edge(Edge::Clock9))?;
                // the right standard-3
                s.edge(Edge::Clock6).link(&t.edge(Edge::Clock9))?;
                t.edge(Edge::Clock6).link(&u.edge(Edge::Clock9))?;
                u.edge(Edge::Clock6).link(&s.edge(Edge::Clock9))?;
                // and link them
                q.edge(Edge::Clock3).link(&s.edge(Edge::Clock3))?;
                Ok(())
            })?;
        }
        for ((rim1, rim2), spoke) in continent // 🧀, 🌙, 🐍
            .rim
            .iter()
            .zip(continent.rim.iter().cycle().skip(1))
            .zip(continent.spokes.iter())
        {
            let rim1p = map.board(&rim1.keys().p()).unwrap();
            let spokep = map.board(&spoke.keys().p()).unwrap();
            let rim2u = map.board(&rim2.keys().u()).unwrap();
            rim1p.edge(Edge::Clock6).link(&spokep.edge(Edge::Clock3))?;
            spokep.edge(Edge::Clock6).link(&rim2u.edge(Edge::Clock9))?;

            let rim2s = map.board(&rim2.keys().s()).unwrap();
            let spokeu = map.board(&spoke.keys().u()).unwrap();
            spokeu.edge(Edge::Clock9).link(&rim2s.edge(Edge::Clock6))?;
        }
        for ((spoke1, spoke2), hub) in continent // 🐍, ♾️, 🏝️
            .spokes
            .iter()
            .step_by(2)
            .zip(continent.spokes.iter().skip(1).step_by(2))
            .zip(continent.hub.iter())
        {
            let spoke1s = map.board(&spoke1.keys().s()).unwrap();
            let hubp = map.board(&hub.keys().p()).unwrap();
            spoke1s.edge(Edge::Clock3).link(&hubp.edge(Edge::Clock3))?;

            let spoke2s = map.board(&spoke2.keys().s()).unwrap();
            let hubt = map.board(&hub.keys().t()).unwrap();
            spoke2s.edge(Edge::Clock3).link(&hubt.edge(Edge::Clock3))?;
        }
        for (hub1, hub2) in continent // 🏝️, 💖
            .hub
            .iter()
            .zip(continent.hub.iter().cycle().skip(1))
        {
            let hub1u = map.board(&hub1.keys().u()).unwrap();
            let hub2r = map.board(&hub2.keys().r()).unwrap();
            hub1u.edge(Edge::Clock3).link(&hub2r.edge(Edge::Clock3))?;
        }
        for ((((spoke, hub1), hub2), rim), parity) in continent // 🐍, 😎, 🏝️, 🧀
            .spokes
            .iter()
            .zip(cycle_double_skip(continent.hub.iter(), 5))
            .zip(cycle_double_skip(continent.hub.iter(), 1))
            .zip(continent.rim.iter())
            .zip([false, true].into_iter().cycle())
        {
            let hub1uq = map.board(&hub1.keys().parity_uq(parity)).unwrap();
            for key in spoke.keys().iter_pqr().chain(Some(rim.keys().q())) {
                hub1uq.ocean().link(&map.board(&key).unwrap().ocean());
            }
            let hub2tp = map.board(&hub2.keys().parity_tp(parity)).unwrap();
            for key in spoke.keys().iter_stu() {
                hub2tp.ocean().link(&map.board(&key).unwrap().ocean());
            }
        }
    }
    for (rim1, rim2) in data::BLUE.rim.iter().zip(data::ORANGE.rim.iter()) {
        // 🧀, 🍌
        for (rim1k, rim2k) in rim1.keys().iter_prstu().zip(rim2.keys().iter_prstu()) {
            map.board(&rim1k)
                .unwrap()
                .ocean()
                .link(&map.board(&rim2k).unwrap().ocean());
        }
    }
    data::modifications(&map, &lc);

    for board in map.boards() {
        for land in board.lands() {
            let mut map = HashMap::<Distance, HashSet<String>>::new();
            for (land, distance) in land.links() {
                map.entry(distance)
                    .or_default()
                    .insert(land.key().to_string());
            }
            println!("{}: {:?}", land.key(), map)
        }
    }
    super::compare_adjacencies(&map, data::EXPECTED)
}
