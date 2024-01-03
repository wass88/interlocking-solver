use crate::puzzle::*;

pub struct PuzzleSearcher<T: PuzzleGenerator> {
    size: usize,
    tries: usize,
    stack: usize,
    initial: Puzzle,
    generator: T,
}
impl<T: PuzzleGenerator> PuzzleSearcher<T> {
    pub fn new(
        size: usize,
        tries: usize,
        stack: usize,
        initial: Puzzle,
        generator: T,
    ) -> PuzzleSearcher<T> {
        PuzzleSearcher {
            size,
            tries,
            stack,
            initial,
            generator,
        }
    }
    pub fn search(&self) -> Puzzle {
        let init_puzzle = self.initial.clone();
        let mut best_puzzles = vec![(init_puzzle, (0, 0), vec![]); self.stack];
        for i in 0..self.tries {
            for k in 0..self.stack {
                let (puzzle, result, moves) = &best_puzzles[k];
                let new_puzzle = self.generator.generate(&puzzle);
                println!("to_solve\n{}", new_puzzle.to_str());
                let result = new_puzzle.solve();
                if result.ok {
                    let moves = result.shrink_move(result.moves(&new_puzzle));
                    let first = first_remove(&moves);
                    let result = (first, moves.len());
                    if best_puzzles[k].1 <= result {
                        best_puzzles[k] = (new_puzzle, result, moves);
                    }
                }
                if i % 1 == 0 {
                    println!(
                        "try #{} (first={}, all={})\nmoves: {:?}\npuzzle:\n{}\n|||",
                        i,
                        best_puzzles[k].1 .0,
                        best_puzzles[k].1 .1,
                        best_puzzles[k].2,
                        best_puzzles[k].0.to_str()
                    );
                }
            }
        }
        best_puzzles[0].0.to_owned()
    }
}

pub trait PuzzleGenerator {
    fn generate(&self, puzzle: &Puzzle) -> Puzzle;
}

pub struct SwapPuzzleGenerator {}
impl PuzzleGenerator for SwapPuzzleGenerator {
    fn generate(&self, puzzle: &Puzzle) -> Puzzle {
        let mut pieces = puzzle.pieces.clone();

        use rand::Rng;
        let mut rnd = rand::thread_rng();

        'retry: loop {
            let x = rnd.gen_range(0..puzzle.size);
            let y = rnd.gen_range(0..puzzle.size);
            let z = rnd.gen_range(0..puzzle.size);

            let mut found = false;
            for a in 0..puzzle.pieces.len() {
                if pieces[a].block.get(x, y, z) {
                    pieces[a].block.set(x, y, z, false);
                    let b = (a + rnd.gen_range(1..puzzle.pieces.len())) % puzzle.pieces.len();
                    pieces[b].block.set(x, y, z, true);
                    found = true;
                    break;
                }
            }
            if !found {
                let a = rnd.gen_range(0..puzzle.pieces.len());
                let (nx, ny, nz) = (
                    rnd.gen_range(0..puzzle.size),
                    rnd.gen_range(0..puzzle.size),
                    rnd.gen_range(0..puzzle.size),
                );
                if !pieces[a].block.get(nx, ny, nz) {
                    continue 'retry;
                }
                assert!(!pieces[a].block.get(x, y, z));
                assert!(pieces[a].block.get(nx, ny, nz));
                pieces[a].block.set(x, y, z, true);
                pieces[a].block.set(nx, ny, nz, false);
            }

            for piece in pieces.iter() {
                if !piece.block.is_connected() || piece.block.count() == 0 {
                    pieces = puzzle.pieces.clone();
                    continue 'retry;
                }
            }
            break;
        }
        let mut puzzle = puzzle.clone();
        puzzle.pieces = pieces.clone();
        puzzle
    }
}

pub struct SwapNPuzzleGenerator {
    pub swaps: usize,
}
impl PuzzleGenerator for SwapNPuzzleGenerator {
    fn generate(&self, puzzle: &Puzzle) -> Puzzle {
        let mut puzzle = puzzle.clone();
        for _ in 0..self.swaps {
            puzzle = SwapPuzzleGenerator {}.generate(&puzzle);
        }
        puzzle
    }
}

fn first_remove(moves: &[Move]) -> usize {
    for i in 0..moves.len() {
        if let Move::Remove(_, _) = moves[i] {
            return i;
        }
    }
    unreachable!("puzzle is not solved")
}

#[cfg(test)]
mod tests {
    use crate::puzzle;

    use super::*;
    #[test]
    fn puzzle_searcher() {
        let searcher = PuzzleSearcher::new(3, 10, 1, Puzzle::base(3, 4, 1), SwapPuzzleGenerator {});
        let puzzle = searcher.search();
        println!("Found\n{}", puzzle.to_str());
        let result = puzzle.solve();
        assert!(result.ok);
        println!("Moves {:?}", result.moves(&puzzle));
        let shrink = result.shrink_move(result.moves(&puzzle));
        println!("Shrink #{} {:?}", shrink.len(), shrink);
    }

    #[test]
    fn puzzle_generator() {
        let holes = 5;
        let mut puzzle = Puzzle::base(3, 4, holes);
        let puzzle_generator = SwapPuzzleGenerator {};
        for _ in 0..100 {
            puzzle = puzzle_generator.generate(&puzzle);
            let mut count = 0;
            puzzle.pieces.iter().for_each(|piece| {
                assert!(piece.block.is_connected());
                assert!(piece.block.count() > 0);
                count += piece.block.count();
            });
            assert_eq!(count, puzzle.size * puzzle.size * puzzle.size - holes);
        }
    }
}
