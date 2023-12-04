use crate::puzzle::*;

mod puzzle;

fn main() {
    let searcher = PuzzleSearcher::new(3, 1000000, 1);
    let puzzle = searcher.search::<Swap5PuzzleGenerator>();
    println!("Found\n{}", puzzle.to_str());
    let result = puzzle.solve();
    assert!(result.ok);
    println!("Moves {:?}", result.moves(&puzzle));
    let shrink = result.shrink_move(result.moves(&puzzle));
    println!("Shrink #{} {:?}", shrink.len(), shrink);
}
