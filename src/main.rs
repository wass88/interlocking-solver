mod cells;
mod gen_all_puzzles;
mod iters;
mod launcher;
mod puzzle;
mod puzzle_num_format;
mod searcher;
mod v3;

use launcher::Launcher;
use puzzle::Puzzle;
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
fn main() {
    launch_gen_all_puzzles()
}
