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
        let base_puzzle = self.initial.clone();
        let mut puzzles = vec![(base_puzzle, 0); self.stack];
        let mut max_move = 0;
        for i in 0..self.tries {
            for k in 0..self.stack {
                let (puzzle, max_moves) = &puzzles[k];
                let new_puzzle = self.generator.generate(&puzzle);
                let result = new_puzzle.solve();
                if result.ok {
                    let moves = result.shrink_move(result.moves(&new_puzzle)).len();
                    if max_move < moves {
                        puzzles[k] = (new_puzzle, moves);
                        max_move = moves;
                    }
                }
            }
            let best_puzzle = puzzles.iter().max_by_key(|p| p.1).unwrap();
            println!(
                "try #{} moves: #{} puzzle:{}",
                i,
                best_puzzle.1,
                best_puzzle.0.to_str()
            );
        }
        puzzles.iter().max_by_key(|p| p.1).unwrap().0.clone()
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

            for a in 0..puzzle.size {
                if pieces[a].block.get(x, y, z) {
                    pieces[a].block.set(x, y, z, false);
                    let b = (a + rnd.gen_range(1..puzzle.size)) % puzzle.size;
                    pieces[b].block.set(x, y, z, true);
                    break;
                }
            }
            for piece in pieces.iter() {
                if !piece.block.is_connected() || piece.block.count() == 0 {
                    pieces = puzzle.pieces.clone();
                    continue 'retry;
                }
            }
            break;
        }
        Puzzle {
            pieces,
            size: puzzle.size,
            margin: puzzle.margin,
            space: puzzle.space,
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn puzzle_searcher() {
        let searcher = PuzzleSearcher::new(3, 10, 2, Puzzle::base(3, 1), SwapPuzzleGenerator {});
        let puzzle = searcher.search();
        println!("Found\n{}", puzzle.to_str());
        let result = puzzle.solve();
        assert!(result.ok);
        println!("Moves {:?}", result.moves(&puzzle));
        let shrink = result.shrink_move(result.moves(&puzzle));
        println!("Shrink #{} {:?}", shrink.len(), shrink);
    }
}
