use crate::{
    iters::V3Iter,
    puzzle::{Move, Piece, Puzzle, SolveResult},
    puzzle_num_format::PuzzleNumFormat,
    v3::{V3, V3I},
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use itertools::Itertools;
use mongodb::{bson::oid::ObjectId, options::FindOptions, Client, Cursor};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PuzzleJson {
    #[serde(rename = "_id", skip_serializing)]
    id: Option<ObjectId>,
    pub code: String,
    pub name: String,
    pub run: String,
    pub solution: SolutionJson,
    pub date: String,
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
    translate: Option<CoordJson>,
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct PuzzlesQuery {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
impl Default for PuzzlesQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(10),
        }
    }
}
pub async fn puzzles(
    State(client): State<Client>,
    query: Query<PuzzlesQuery>,
) -> (StatusCode, Json<Vec<PuzzleJson>>) {
    let generated = client.database("puzzle").collection("generated");
    let page = query.page.unwrap() as u64;
    let limit = query.limit.unwrap() as u64;
    let option = FindOptions::builder()
        .skip((page - 1) * limit)
        .limit(limit as i64)
        .build();
    let puzzles = generated.find(None, option).await.unwrap();
    use futures::stream::TryStreamExt;
    let puzzles: Vec<PuzzleJson> = puzzles.try_collect().await.unwrap();

    (StatusCode::OK, Json(puzzles))
}

impl PuzzleJson {
    pub fn from_result(puzzle: &Puzzle, result: &SolveResult) -> PuzzleJson {
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
                    pieces: p.to_owned(),
                    translate: Some(CoordJson::from_v3i(*x)),
                }),
                Move::Remove(p, _) => Some(MoveJson {
                    pieces: vec![*p],
                    translate: None,
                }),
            })
            .collect_vec();
        PuzzleJson {
            id: None,
            code: code.to_block_code(),
            name,
            run: "none".to_owned(),
            solution: SolutionJson { pieces, moves },
            date: "".to_owned(),
        }
    }

    pub fn normalized_from_puzzle(puzzle: &Puzzle) -> PuzzleJson {
        let puzzle_code = PuzzleNumFormat::from_puzzle(puzzle);
        let normalized = puzzle_code.normalize().to_puzzle();
        let result = normalized.solve();
        PuzzleJson::from_result(&normalized, &result)
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

fn sample_puzzle2() -> Puzzle {
    let pieces = Piece::vec_from_str(
        4,
        r#"
        burr_plate([[
            "....|...x|...x|x..x",
            ".xx.|....|....|xx.x",
            "..x.|....|....|xx.x",
            "x.xx|x.x.|xxxx|.x.x",
            ],[
            "xxxx|xxx.|..x.|.xx.",
            "x..x|.x..|....|..x.",
            "...x|...x|....|..x.",
            "....|...x|....|..x.",
            ],[
            "....|....|.x..|....",
            "....|..x.|.xx.|....",
            "....|....|....|....",
            "....|....|....|....",
            ],[
            "....|....|....|....",
            "....|...x|...x|....",
            "....|.x..|.xxx|....",
            "....|....|....|....",
            ],[
            "....|....|x...|....",
            "....|x...|x...|....",
            "xx..|x...|x...|....",
            ".x..|.x..|....|....",
            ]]);
        "#,
    );

    Puzzle {
        size: 4,
        pieces,
        space: 16,
        margin: 4,
        reach_limit: None,
        multi: None,
    }
}
