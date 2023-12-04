mod cells;
mod puzzle;
mod searcher;

use crate::puzzle::*;
use crate::searcher::*;

fn main() {
    let searcher = PuzzleSearcher::new(
        3,
        10000,
        1,
        Puzzle::base(3, 1),
        SwapNPuzzleGenerator { swaps: 10 },
    );
    let puzzle = searcher.search();
    println!("Found\n{}", puzzle.to_str());
    let result = puzzle.solve();
    assert!(result.ok);
    println!("Moves {:?}", result.moves(&puzzle));
    let shrink = result.shrink_move(result.moves(&puzzle));
    println!("Shrink #{} {:?}", shrink.len(), shrink);
}
