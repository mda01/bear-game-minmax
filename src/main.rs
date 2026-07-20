pub mod board;
pub mod minmax;

use std::collections::HashSet;

use crate::{board::Board, minmax::NodeType};

struct BoardNode {
    board: Board,
    children: HashSet<Board>,
    depth: i16
}

impl NodeType for BoardNode {
    fn is_over(&self) -> bool {
        todo!()
    }

    fn heuristic(&self) -> i32 {
        todo!()
    }

    fn get_children(&self) -> Vec<Self> {
        todo!()
    }

    fn hash(&self) -> i128 {
        todo!()
    }
}



fn main() {
    
}
