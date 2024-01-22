mod cells;
mod launcher;
mod puzzle;
mod searcher;

use launcher::Launcher;
use puzzle::Puzzle;
use searcher::*;

fn main() {
    let constraints = MinPuzzleSizeConstraints {
        size: 2,
        next: TerminalPuzzleConstraints {},
    };
    let searcher = PuzzleSearcher::new(
        1000000,
        1,
        Puzzle::base(4, 5, 2, Some(50000)),
        50000,
        SwapNPuzzleGenerator {
            swaps: 6,
            constraints,
        },
        ShrinkStepEvaluator {},
    );
    let launcher = Launcher::new(searcher, 2);
    let writer = launcher::PuzzleWriter::new("puzzles/puzzle_20240122_4x4_5_coshirnk".to_string());
    launcher.launch(&writer).unwrap();
}
