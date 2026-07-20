// The board can be viewed as follow
/*

           0 1
    14 15       2 3
13                   4
12                   5
    11 10       7 6
           9 8

for the outside and similarly from 16 to 23 for the inside, and 24 at the center

So for n <= 15, we can move to n +- 1 % 16 and n//2 + 16
for 16 <= n <= 23, we can move to 24 and 2(n-16) and 2(n-16) + 1
for n == 24, we can move from 16 to 23

*/

/*

0 means empty, 1 is the bear, 2 3 and 4 are the hunters (may change)

*/

use std::hash::{Hash, Hasher};

#[derive(Eq, Clone)]
pub struct Board {
    pub board: Vec<i16>, // stores pieces position
    pub player_turn: bool,
    pub turns: i16,
}

#[derive(Clone, Copy)]
pub struct Move {
    pub piece: usize,
    pub origin: i16,
    pub destination: i16,
}

pub fn create_init_board() -> Board {
    return Board {
        board: vec![24, 8, 9, 20],
        player_turn: true,
        turns: 0,
    };
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i16(self.board[0] * 100 + self.board[1] + self.board[2] + self.board[3]);
        if self.player_turn {
            state.write_i8(1);
        } else {
            state.write_i8(0);
        }
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.board[0] * 100 + self.board[1] + self.board[2] + self.board[3]
            == other.board[0] * 100 + other.board[1] + other.board[2] + other.board[3]
            && self.player_turn == other.player_turn
    }
}

fn adjacent_pos(pos: i16) -> Vec<i16> {
    let mut adj_pos: Vec<i16> = vec![];
    if pos <= 15 {
        adj_pos.push((pos + 1) % 16);
        adj_pos.push((pos - 1) % 16);
        adj_pos.push((pos / 2) + 16);
    } else if pos <= 23 {
        adj_pos.push(24);
        adj_pos.push(2 * (pos - 16));
        adj_pos.push(2 * (pos - 16) + 1);
    } else {
        for i in 16..24 {
            adj_pos.push(i);
        }
    }
    return adj_pos;
}

pub fn next_moves(b: &Board) -> Vec<Move> {
    let mut moves = vec![];

    if b.player_turn {
        let adj_pos = adjacent_pos(b.board[0]);
        for p in adj_pos {
            if !b.board.contains(&p) {
                moves.push(Move {
                    piece: 0,
                    origin: b.board[0],
                    destination: p,
                });
            }
        }
    } else {
        for i in 1..4 {
            let adj_pos = adjacent_pos(b.board[i]);
            for p in adj_pos {
                if !b.board.contains(&p) {
                    moves.push(Move {
                        piece: i,
                        origin: b.board[i],
                        destination: p,
                    });
                }
            }
        }
    }

    return moves;
}

pub fn play_move(b: &mut Board, m: Move) {
    b.board[m.piece] = m.destination;
    b.player_turn = !b.player_turn;
}

pub fn print_board(b: &Board) {
    println!(
        "{} {} {} {}",
        b.board[0], b.board[1], b.board[2], b.board[3]
    );
}
