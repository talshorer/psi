use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    ops::Deref,
    rc::{Rc, Weak},
    str::FromStr,
};

use enum_map::EnumMap;

use crate::{
    Map,
    layout::{
        CLOCKWISE, COUNTER_CLOCKWISE, Corner, Edge, LandNum, Layout, LayoutEdge, Rotate, Terrain,
    },
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Distance(pub usize);

struct LandLink {
    land: Weak<Land>,
    distance: Distance,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LandKey(pub BoardKey, pub LandNum);

impl FromStr for LandKey {
    type Err = <usize as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (board, land) = s.split_at(s.len() - 1);
        land.parse()
            .map(|land| Self(BoardKey::from(board), LandNum(land)))
    }
}

pub struct Land {
    key: LandKey,
    links: RefCell<HashMap<LandKey, LandLink>>,
    terrain: Terrain,
    board: Weak<Board>,
}

impl Land {
    fn link_one_way(self: &Rc<Self>, other: &Rc<Self>, distance: Distance) {
        self.links
            .borrow_mut()
            .entry(other.key.clone())
            .or_insert(LandLink {
                land: Rc::downgrade(other),
                distance,
            });
    }

    fn link(self: &Rc<Self>, other: &Rc<Self>, distance: Distance) {
        self.link_one_way(other, distance);
        other.link_one_way(self, distance);
    }

    fn links_inner(&self) -> <Vec<(Rc<Land>, Distance)> as IntoIterator>::IntoIter {
        self.links
            .borrow()
            .values()
            .filter_map(|link| link.land.upgrade().map(|rc| (rc, link.distance)))
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn links(&self) -> impl Iterator<Item = (Rc<Land>, Distance)> {
        self.links_inner()
    }

    pub fn key(&self) -> LandKey {
        self.key.clone()
    }

    pub fn terrain(&self) -> Terrain {
        self.terrain
    }

    fn unlink(&self, key: &LandKey) {
        self.links.borrow_mut().remove(key);
    }

    pub fn cast_down(&self) {
        for (land, _) in self.links() {
            land.unlink(&self.key);
            self.unlink(&land.key);
        }
        if let Some(board) = self.board.upgrade() {
            board.inner.borrow_mut().lands.remove(&self.key.1);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BoardKey(Rc<str>);

impl From<&str> for BoardKey {
    fn from(s: &str) -> Self {
        BoardKey(Rc::from(s))
    }
}

struct BoardInner {
    lands: HashMap<LandNum, Rc<Land>>,
    neighbours: EnumMap<Edge, Option<(Weak<Board>, Edge)>>,
}

impl BoardInner {
    fn land(&self, num: LandNum) -> Option<Rc<Land>> {
        self.lands.get(&num).cloned()
    }
}

pub struct Board {
    inner: RefCell<BoardInner>,
    layout: Rc<Layout>,
    map: Weak<Map>,
}

impl Board {
    pub(crate) fn new(map: Weak<Map>, key: BoardKey, layout: Rc<Layout>) -> Rc<Self> {
        let rc = Rc::new(Self {
            inner: RefCell::new(BoardInner {
                lands: HashMap::new(),
                neighbours: EnumMap::default(),
            }),
            layout: layout.clone(),
            map,
        });
        let weak = Rc::downgrade(&rc);
        let mut inner = rc.inner.borrow_mut();

        for (land, terrain) in layout.terrains.iter() {
            inner.lands.insert(
                *land,
                Rc::new(Land {
                    key: LandKey(key.clone(), *land),
                    links: Default::default(),
                    terrain: *terrain,
                    board: weak.clone(),
                }),
            );
        }

        for (land, adjacencies) in layout.internal_adjacencies.iter() {
            let land = inner.lands.get(land).expect("Invalid Layout");
            for other_land in adjacencies {
                land.link(
                    inner.lands.get(other_land).expect("Invalid Layout"),
                    Distance(1),
                );
            }
        }

        drop(inner);
        rc
    }

    pub fn edge(self: &Rc<Self>, edge: Edge) -> BoardEdge {
        BoardEdge {
            board: self.clone(),
            edge,
        }
    }

    pub fn lands(&self) -> impl Iterator<Item = Rc<Land>> {
        self.inner
            .borrow()
            .lands
            .values()
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn land(&self, num: LandNum) -> Option<Rc<Land>> {
        self.inner.borrow().land(num)
    }

    fn corner(&self, corner: Corner) -> Option<Rc<Land>> {
        self.inner.borrow().land(self.layout.corner(corner))
    }

    fn neighbour(&self, edge: Edge) -> Option<BoardEdge> {
        self.inner.borrow().neighbours[edge]
            .as_ref()
            .and_then(|(weak_board, edge)| weak_board.upgrade().map(|board| board.edge(*edge)))
    }

    pub fn ocean(self: &Rc<Self>) -> BoardOcean {
        BoardOcean(self.clone())
    }

    fn coastals(&self) -> impl Iterator<Item = Rc<Land>> {
        let ocean = self.land(LandNum(0)).expect("A board must have an ocean");
        let ocean_key = ocean.key.0.clone();
        ocean
            .links_inner()
            .filter_map(move |(land, distance)| {
                (land.key.0 == ocean_key && distance == Distance(1)).then_some(land)
            })
            .chain(Some(ocean))
    }
}

#[derive(Clone)]
pub struct BoardEdge {
    board: Rc<Board>,
    edge: Edge,
}

#[derive(Debug, thiserror::Error)]
pub enum BoardEdgeLinkError {
    #[error("Map mismatch")]
    MapMismatch,
    #[error("Self slot occupied")]
    OccupiedSelf,
    #[error("Other slot occupied")]
    OccupiedOther,
}

impl BoardEdge {
    pub fn link(&self, other: &Self) -> Result<(), BoardEdgeLinkError> {
        let true = self.board.map.ptr_eq(&other.board.map) else {
            return Err(BoardEdgeLinkError::MapMismatch);
        };

        let self_entry = BoardEdgeEntry::new(self).ok_or(BoardEdgeLinkError::OccupiedSelf)?;
        let other_entry = BoardEdgeEntry::new(other).ok_or(BoardEdgeLinkError::OccupiedOther)?;
        self_entry.link(other_entry);

        let self_layout = BoardEdgeLayoutRef::new(self);
        let other_layout = BoardEdgeLayoutRef::new(other);

        for (self_land_num, other_land_num) in self_layout.link_iter(&other_layout) {
            if let Some(self_land) = self.board.inner.borrow().lands.get(&self_land_num)
                && let Some(other_land) = other.board.inner.borrow().lands.get(&other_land_num)
            {
                self_land.link(other_land, Distance(1));
            }
        }

        self.link_corners_one_way(other);
        other.link_corners_one_way(self);

        Ok(())
    }

    fn link_corners_one_way(&self, other: &Self) {
        self.link_corners_one_way_inner(other, &CLOCKWISE);
        self.link_corners_one_way_inner(other, &COUNTER_CLOCKWISE);
    }

    fn link_corners_one_way_inner(&self, other: &Self, rotate: &Rotate) -> Option<()> {
        let land = other.corner(rotate.reverse())?;
        let mut cur = self.clone();
        loop {
            let rotated_edge = rotate.to_edge(cur.edge)?;
            let other_edge = cur.board.neighbour(rotated_edge)?;
            let other_corner = other_edge.corner(rotate)?;
            if other_corner.key == land.key {
                return None;
            }
            land.link(&other_corner, Distance(1));
            cur = other_edge;
        }
    }

    fn corner(&self, rotate: &Rotate) -> Option<Rc<Land>> {
        self.board.corner(rotate.to_corner(self.edge))
    }
}

struct BoardEdgeEntry<'a> {
    inner: &'a BoardEdge,
    borrow: RefMut<'a, BoardInner>,
}

impl<'a> BoardEdgeEntry<'a> {
    fn new(inner: &'a BoardEdge) -> Option<Self> {
        let borrow = inner.board.inner.borrow_mut();
        borrow.neighbours[inner.edge]
            .is_none()
            .then(|| Self { inner, borrow })
    }

    fn link_one_way(&mut self, other: &Self) {
        self.borrow.neighbours[self.inner.edge] =
            Some((Rc::downgrade(&other.inner.board), other.inner.edge));
    }

    fn link(mut self, mut other: Self) {
        self.link_one_way(&other);
        other.link_one_way(&self);
    }
}

struct BoardEdgeLayoutRef {
    layout: Rc<Layout>,
    edge: Edge,
}

impl BoardEdgeLayoutRef {
    fn new(edge: &BoardEdge) -> Self {
        Self {
            layout: edge.board.layout.clone(),
            edge: edge.edge,
        }
    }
}

impl Deref for BoardEdgeLayoutRef {
    type Target = LayoutEdge;

    fn deref(&self) -> &Self::Target {
        &self.layout.edges[self.edge]
    }
}

pub struct BoardOcean(Rc<Board>);

impl BoardOcean {
    pub fn link(&self, other: &Self) {
        for self_land in self.0.coastals() {
            for other_land in other.0.coastals() {
                self_land.link(&other_land, Distance(2));
            }
        }
    }
}
