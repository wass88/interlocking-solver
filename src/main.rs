mod cells;
mod launcher;
mod puzzle;
mod searcher;

use launcher::Launcher;
use puzzle::Puzzle;
use searcher::*;

fn main() {
    let constraints = MinPuzzleSizeConstraints {
        size: 4,
        next: TerminalPuzzleConstraints {},
    };
    let searcher = PuzzleSearcher::new(
        1000000,
        1,
        Puzzle::base(4, 9, 2, Some(50000)),
        10000,
        SwapNPuzzleGenerator {
            swaps: 6,
            constraints,
        },
        DupDropEvaluator {},
    );
    let launcher = Launcher::new(searcher, 2);
    let writer = launcher::PuzzleWriter::new("puzzles/puzzle_20240120_4x4_9_dup_drop".to_string());
    launcher.launch(&writer).unwrap();
}
