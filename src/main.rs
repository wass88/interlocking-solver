mod cells;
mod luncher;
mod puzzle;
mod searcher;

use luncher::Launcher;
use puzzle::Puzzle;
use searcher::*;

fn main() {
    let searcher = PuzzleSearcher::new(
        3,
        1000000,
        1,
        Puzzle::base(4, 5, 2),
        10000,
        SwapNPuzzleGenerator { swaps: 6 },
        ShrinkStepEvaluator {},
    );
    let luncher = Launcher::new(searcher, 8);
    let writer = luncher::PuzzleWriter::new("puzzles".to_string());
    luncher.launch(&writer).unwrap();
}
