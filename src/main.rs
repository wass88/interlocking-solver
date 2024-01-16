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
        8,
        Puzzle::base(4, 5, 2, Some(50000)),
        10000,
        SwapNPuzzleGenerator { swaps: 6 },
        DupDropEvaluator {},
    );
    let luncher = Launcher::new(searcher, 2);
    let writer = launcher::PuzzleWriter::new("puzzles/puzzle_20240116_4_dup_drop".to_string());
    luncher.launch(&writer).unwrap();
}
