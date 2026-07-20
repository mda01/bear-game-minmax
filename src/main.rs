pub mod board;
pub mod minmax;

use std::{
    hash::{DefaultHasher, Hash, Hasher},
    io,
};

use crate::{
    board::{Board, Move, create_init_board, next_moves, play_move, print_board}, minmax::{NodeType, minmax},
};

struct BoardNode {
    board: Board,
    depth: usize,
}

impl NodeType for BoardNode {
    fn is_over(&self) -> bool {
        next_moves(&self.board).is_empty() || self.depth > 20
    }

    fn heuristic(&self) -> i32 {
        if self.is_over() {
            if self.board.player_turn {
                return 100;
            } else {
                return -100;
            }
        } else {
            return 0;
        }
    }

    fn get_children(&self) -> Vec<Self> {
        let mut children = vec![];
        let next_moves = next_moves(&self.board);
        for next_move in next_moves {
            let mut next_board = self.board.clone();
            play_move(&mut next_board, next_move);
            children.push(BoardNode {
                board: next_board,
                depth: self.depth + 1,
            });
        }
        return children;
    }

    fn hash(&self) -> i128 {
        let mut hasher = DefaultHasher::new();
        self.board.hash(&mut hasher);
        return hasher.finish().into();
    }
}

fn best_next_move(node: &BoardNode, depth: usize, player: bool) -> (Move, i32) {
    let mut best_move = Move {
        piece: 0,
        origin: 0,
        destination: 0,
    };
    let mut best_score = i32::MIN;

    for m in next_moves(&node.board) {
        let mut next_board = node.board.clone();
        play_move(&mut next_board, m);
        let move_score = minmax(
            BoardNode {
                board: next_board,
                depth: depth,
            },
            depth + 1,
            player,
        );
        if move_score > best_score {
            best_score = move_score;
            best_move = m;
        }
    }
    return (best_move, best_score);
}

#[allow(dead_code)]
fn readarr() -> Vec<i16> {
    let mut v = String::new();
    std::io::stdin().read_line(&mut v).unwrap();
    v.trim()
        .split_whitespace()
        .map(|s: &str| s.parse().unwrap())
        .collect()
}

fn main() {
    let root_board = create_init_board();
    let mut root_node: BoardNode = BoardNode {
        board: root_board,
        depth: 0,
    };

    while !root_node.is_over() {
        let bnm = best_next_move(&root_node, 4, root_node.board.player_turn);
        println!(
            "Score: {}, Move: {} {}",
            bnm.1, bnm.0.piece, bnm.0.destination
        );
        play_move(&mut root_node.board, bnm.0);
        print_board(&root_node.board);

        let player_inp = readarr();
        let player_move = Move {
            piece: player_inp[0].try_into().unwrap(),
            origin: player_inp[1],
            destination: player_inp[2],
        };
        play_move(&mut root_node.board, player_move);
        print_board(&root_node.board);
    }
}
