mod cells;
mod puzzle;
mod searcher;

use crate::puzzle::*;
use crate::searcher::*;

fn main() {
    let searcher = PuzzleSearcher::new(
        3,
        100000000,
        1,
        Puzzle::base(4, 5, 2),
        SwapNPuzzleGenerator { swaps: 5 },
    );
    let puzzle = searcher.search();
    println!("Found\n{}", puzzle.to_str());
    let result = puzzle.solve();
    assert!(result.ok);
    println!("Moves {:?}", result.moves(&puzzle));
    let shrink = result.shrink_move(result.moves(&puzzle));
    println!("Shrink #{} {:?}", shrink.len(), shrink);
}
