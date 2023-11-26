use crate::puzzle::*;

mod puzzle;

fn main() {
    let searcher = PuzzleSearcher::new(4, 1000);
    let puzzle = searcher.search::<SwapPuzzleGenerator>();
    println!("Found\n{}", puzzle.to_str());
    let result = puzzle.solve();
    assert!(result.ok);
    println!("Moves {:?}", result.moves(&puzzle));
    let shrink = result.shrink_move(result.moves(&puzzle));
    println!("Shrink #{} {:?}", shrink.len(), shrink);
}
