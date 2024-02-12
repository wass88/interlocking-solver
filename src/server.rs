use axum::{http::StatusCode, Json};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    iters::V3Iter,
    puzzle::{Move, Piece, Puzzle, SolveResult},
    puzzle_num_format::PuzzleNumFormat,
    v3::{V3, V3I},
};

#[derive(Serialize, Deserialize)]
pub struct PuzzleJson {
    code: String,
    name: String,
    solution: SolutionJson,
}
#[derive(Serialize, Deserialize)]
struct SolutionJson {
    pieces: Vec<PieceJson>,
    moves: Vec<MoveJson>,
}
#[derive(Serialize, Deserialize)]
struct PieceJson {
    blocks: Vec<CoordJson>,
}
#[derive(Serialize, Deserialize)]
struct CoordJson {
    x: isize,
    y: isize,
    z: isize,
}
#[derive(Serialize, Deserialize)]
struct MoveJson {
    pieces: Vec<usize>,
    translate: CoordJson,
}

pub async fn puzzles() -> (StatusCode, Json<Vec<PuzzleJson>>) {
    let puzzle = sample_puzzle();
    let result = puzzle.solve();
    let json = PuzzleJson::from_result(&puzzle, &result);
    let puzzles = vec![json];
    (StatusCode::OK, Json(puzzles))
}

impl PuzzleJson {
    fn from_result(puzzle: &Puzzle, result: &SolveResult) -> PuzzleJson {
        let code = PuzzleNumFormat::from_puzzle(puzzle);
        let date = chrono::Local::now();
        let name = format!("Untitled_{}", date.format("%Y%m%dT%H%M%S"));
        let pieces = puzzle
            .pieces
            .iter()
            .map(|piece| PieceJson::from_piece(piece))
            .collect_vec();
        let moves = result.moves(puzzle);
        let moves = moves
            .iter()
            .filter_map(|mov| match mov {
                Move::Shift(p, x) => Some(MoveJson {
                    pieces: vec![*p],
                    translate: CoordJson::from_v3i(*x),
                }),
                _ => None,
            })
            .collect_vec();
        PuzzleJson {
            code: code.to_block_code(),
            name,
            solution: SolutionJson { pieces, moves },
        }
    }
}
impl PieceJson {
    fn from_piece(piece: &Piece) -> PieceJson {
        let mut blocks = vec![];
        for x in V3Iter::cube(piece.size) {
            if piece.block.getv(x) {
                blocks.push(CoordJson::from_v3(x))
            }
        }
        PieceJson { blocks }
    }
}
impl CoordJson {
    fn from_v3(x: V3) -> CoordJson {
        Self::from_v3i(V3I::from(x))
    }
    fn from_v3i(x: V3I) -> CoordJson {
        let V3I(x, y, z) = x;
        CoordJson { x, y, z }
    }
}

pub fn sample_puzzle() -> Puzzle {
    let piece_a = Piece::from_str(
        4,
        "
x.xx|x...|x...|x...
x..x|...x|....|x...
x..x|....|....|xxxx
x..x|x..x|...x|...x",
    );
    let piece_b = Piece::from_str(
        4,
        "
.x..|..xx|.xx.|.xx.
.x..|..x.|....|..xx
.xx.|..x.|....|....
..x.|....|....|....",
    );
    let piece_c = Piece::from_str(
        4,
        "
....|....|...x|...x
....|....|xxxx|....
....|....|x...|....
....|....|x...|x...",
    );
    let piece_d = Piece::from_str(
        4,
        "
....|.x..|....|....
....|xx..|....|....
....|.x..|....|....
.x..|.xx.|..x.|..x.",
    );
    let piece_e = Piece::from_str(
        4,
        "
....|....|....|....
....|....|....|....
....|...x|.xxx|....
....|....|.x..|.x..",
    );
    Puzzle {
        size: 4,
        pieces: vec![piece_a, piece_b, piece_c, piece_d, piece_e],
        space: 16,
        margin: 4,
        reach_limit: None,
        multi: Some(1),
    }
}
