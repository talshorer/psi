use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    ops::Deref,
    rc::{Rc, Weak},
    str::FromStr,
};

use enum_map::EnumMap;

use crate::layout::{
    CLOCKWISE, COUNTER_CLOCKWISE, Corner, Edge, LandNum, Layout, LayoutEdge, Rotate, Terrain,
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

struct LandInner {
    key: LandKey,
    links: HashMap<LandKey, LandLink>,
    terrain: Terrain,
    board: Weak<Board>,
}

pub struct Land(RefCell<LandInner>);

impl Land {
    fn link_one_way(self: &Rc<Self>, other: &Rc<Self>, distance: Distance) {
        self.0
            .borrow_mut()
            .links
            .entry(other.0.borrow().key.clone())
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
        self.0
            .borrow()
            .links
            .values()
            .filter_map(|link| link.land.upgrade().map(|rc| (rc, link.distance)))
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn links(&self) -> impl Iterator<Item = (Rc<Land>, Distance)> {
        self.links_inner()
    }

    pub fn key(&self) -> LandKey {
        self.0.borrow().key.clone()
    }

    pub fn terrain(&self) -> Terrain {
        self.0.borrow().terrain
    }

    fn unlink(&self, key: &LandKey) {
        self.0.borrow_mut().links.remove(key);
    }

    pub fn cast_down(&self) {
        let key = self.key();
        for (land, _) in self.links() {
            land.unlink(&key);
            self.unlink(&land.key());
        }
        if let Some(board) = self.0.borrow().board.upgrade() {
            board.0.borrow_mut().lands.remove(&key.1);
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
    layout: Rc<Layout>,
    neighbours: EnumMap<Edge, Option<(Weak<Board>, Edge)>>,
}

impl BoardInner {
    fn land(&self, num: LandNum) -> Option<Rc<Land>> {
        self.lands.get(&num).cloned()
    }
}

pub struct Board(RefCell<BoardInner>);

impl Board {
    pub(crate) fn new(key: BoardKey, layout: Rc<Layout>) -> Rc<Self> {
        let rc = Rc::new(Self(RefCell::new(BoardInner {
            lands: HashMap::new(),
            layout: layout.clone(),
            neighbours: EnumMap::default(),
        })));
        let weak = Rc::downgrade(&rc);
        let mut inner = rc.0.borrow_mut();

        for (land, terrain) in layout.terrains.iter() {
            inner.lands.insert(
                *land,
                Rc::new(Land(RefCell::new(LandInner {
                    key: LandKey(key.clone(), *land),
                    links: HashMap::new(),
                    terrain: *terrain,
                    board: weak.clone(),
                }))),
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
        self.0
            .borrow()
            .lands
            .values()
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn land(&self, num: LandNum) -> Option<Rc<Land>> {
        self.0.borrow().land(num)
    }

    fn corner(&self, corner: Corner) -> Option<Rc<Land>> {
        let inner = self.0.borrow();
        inner.land(inner.layout.corner(corner))
    }

    fn neighbour(&self, edge: Edge) -> Option<BoardEdge> {
        self.0.borrow().neighbours[edge]
            .as_ref()
            .and_then(|(weak_board, edge)| weak_board.upgrade().map(|board| board.edge(*edge)))
    }

    pub fn ocean(self: &Rc<Self>) -> BoardOcean {
        BoardOcean(self.clone())
    }

    fn coastals(&self) -> impl Iterator<Item = Rc<Land>> {
        let ocean = self.land(LandNum(0)).expect("A board must have an ocean");
        let key = ocean.key().0;
        ocean
            .links_inner()
            .filter_map(move |(land, distance)| {
                (land.key().0 == key && distance == Distance(1)).then_some(land)
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
        let self_entry = BoardEdgeEntry::new(self).ok_or(BoardEdgeLinkError::OccupiedSelf)?;
        let other_entry = BoardEdgeEntry::new(other).ok_or(BoardEdgeLinkError::OccupiedOther)?;
        self_entry.link(other_entry);

        let self_layout = BoardEdgeLayoutRef::new(self);
        let other_layout = BoardEdgeLayoutRef::new(other);

        for (self_land_num, other_land_num) in self_layout.link_iter(&other_layout) {
            if let Some(self_land) = self.board.0.borrow().lands.get(&self_land_num)
                && let Some(other_land) = other.board.0.borrow().lands.get(&other_land_num)
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
        let sentinel = land.key();
        let mut cur = self.clone();
        loop {
            let rotated_edge = rotate.to_edge(cur.edge)?;
            let other_edge = cur.board.neighbour(rotated_edge)?;
            let other_corner = other_edge.corner(rotate)?;
            if other_corner.key() == sentinel {
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
        let borrow = inner.board.0.borrow_mut();
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
        let layout = edge.board.0.borrow().layout.clone();
        Self {
            layout,
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
