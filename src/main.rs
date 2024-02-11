mod cells;
mod gen_all_puzzles;
mod iters;
mod launcher;
mod puzzle;
mod puzzle_num_format;
mod searcher;
mod v3;

use launcher::Launcher;
use puzzle::{Piece, Puzzle};
use searcher::*;

fn launch_generate() {
    let constraints = MinPuzzleSizeConstraints {
        size: 2,
        next: TerminalPuzzleConstraints {},
    };
    let searcher = PuzzleSearcher::new(
        1000000,
        1,
        Puzzle::base(4, 8, 0, Some(50000)),
        30000,
        SwapNPuzzleGenerator {
            swaps: 6,
            constraints,
        },
        ShrinkStepEvaluator {},
    );
    let launcher = Launcher::new(searcher, 4);
    let writer = launcher::PuzzleWriter::new("puzzles/puzzle_20240122_4x4_8_coshirnk".to_string());
    launcher.launch(&writer).unwrap();
}

fn launch_gen_all_puzzles() {
    let mut writer = gen_all_puzzles::DebugWriter::new();
    gen_all_puzzles::GenAllPuzzles {
        size: 3,
        piece: 3,
        holes: 0,
    }
    .generate(&mut writer);
    println!("done");
    println!("{}", writer.codes.len());
}

fn solve_sample_puzzle() {
    let piece_a = Piece::from_str(
        4,
        "
x.xx|x...|x...|x...
x..x|...x|....|x...
x..x|....|....|xxxx
x..x|...x|...x|...x",
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
    let puzzle = Puzzle {
        size: 4,
        pieces: vec![piece_a, piece_b, piece_c, piece_d, piece_e],
        space: 16,
        margin: 4,
        reach_limit: None,
        multi: Some(1),
    };
    assert!(puzzle.check_puzzle());
    let result = puzzle.solve();
    let moves = result.moves(&puzzle);
    println!("{:?}", moves);
}

fn main() {
    // launch_generate()
    // launch_gen_all_puzzles()
    solve_sample_puzzle()
}
