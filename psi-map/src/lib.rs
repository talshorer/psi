use std::{collections::HashMap, rc::Rc};

pub mod board;
pub mod layout;

use board::{Board, BoardKey, Land, LandKey};
use layout::Layout;

#[derive(Default)]
pub struct Map {
    boards: HashMap<BoardKey, Rc<Board>>,
}

impl Map {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_board(&mut self, key: BoardKey, layout: Rc<Layout>) -> Rc<Board> {
        let board = Board::new(key.clone(), layout);
        self.boards.insert(key, board.clone());
        board
    }

    pub fn boards(&self) -> impl Iterator<Item = Rc<Board>> + '_ {
        self.boards.values().cloned()
    }

    pub fn land(&self, key: &LandKey) -> Option<Rc<Land>> {
        self.boards.get(&key.0).and_then(|board| board.land(key.1))
    }
}
