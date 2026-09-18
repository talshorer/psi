use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub mod board;
pub mod layout;

use board::{Board, BoardKey, Land, LandKey};
use layout::Layout;

#[derive(Default)]
struct MapInner {
    boards: HashMap<BoardKey, Rc<Board>>,
}

pub struct Map(RefCell<MapInner>);

impl Map {
    pub fn new() -> Rc<Self> {
        Rc::new(Self(Default::default()))
    }

    pub fn add_board(self: &Rc<Self>, key: BoardKey, layout: Rc<Layout>) -> Rc<Board> {
        let board = Board::new(key.clone(), layout);
        self.0.borrow_mut().boards.insert(key, board.clone());
        board
    }

    pub fn boards(&self) -> impl Iterator<Item = Rc<Board>> {
        self.0.borrow().boards.clone().into_values()
    }

    pub fn land(&self, key: &LandKey) -> Option<Rc<Land>> {
        self.0
            .borrow()
            .boards
            .get(&key.0)
            .and_then(|board| board.land(key.1))
    }
}
