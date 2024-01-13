mod cells;
mod launcher;
mod puzzle;
mod searcher;

use launcher::Launcher;
use puzzle::Puzzle;
use searcher::*;

fn main() {
    let searcher = PuzzleSearcher::new(
        1000000,
        1,
        Puzzle::base(5, 6, 2, Some(50000)),
        10000,
        SwapNPuzzleGenerator { swaps: 6 },
        ShrinkStepEvaluator {},
    );
    let luncher = Launcher::new(searcher, 2);
    let writer = launcher::PuzzleWriter::new("puzzles/puzzles_5".to_string());
    luncher.launch(&writer).unwrap();
}
