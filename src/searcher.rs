use crate::puzzle::*;
use itertools::Itertools;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct PuzzleSearcher<G: PuzzleGenerator, E: Evaluator> {
    tries: usize,
    stack: usize,
    give_up: usize,
    initial: Puzzle,
    generator: G,
    pub evaluator: E,
}
impl<G: PuzzleGenerator, E: Evaluator> PuzzleSearcher<G, E> {
    pub fn new(
        tries: usize,
        stack: usize,
        initial: Puzzle,
        give_up: usize,
        generator: G,
        evaluator: E,
    ) -> PuzzleSearcher<G, E> {
        PuzzleSearcher {
            tries,
            stack,
            initial,
            give_up,
            generator,
            evaluator,
        }
    }
    pub fn search(&self) -> Puzzle {
        let init_puzzle = self.initial.clone();
        let mut best_puzzles = vec![(init_puzzle, E::Value::default(), vec![], 0); self.stack];
        for i in 0..self.tries {
            for k in 0..self.stack {
                {
                    let best = &mut best_puzzles[k];
                    best.3 += 1;
                    if best.3 > self.give_up {
                        return best_puzzles[0].0.to_owned();
                    }
                }

                let (puzzle, best_value, _, _) = &best_puzzles[k];
                let new_puzzle = self.generator.generate(&puzzle);
                //println!("to_solve\n{}", new_puzzle.to_str());
                let result = new_puzzle.solve();
                if result.ok {
                    let value = self.evaluator.evaluate(&new_puzzle, &result);
                    println!("#{} value: {}", i, value.to_str());
                    if best_value <= &value {
                        let shrink_moves = result.shrink_move(&result.moves(&new_puzzle));
                        let count = if best_value < &value {
                            println!("#{} updated", i);
                            0
                        } else {
                            best_puzzles[k].3
                        };
                        best_puzzles[k] = (new_puzzle, value, shrink_moves, count);
                    }
                }
                if i % 1 == 0 {
                    println!(
                        "try #{}-({}<{}) ({})",
                        i,
                        best_puzzles[k].3,
                        self.give_up,
                        best_puzzles[k].1.to_str(),
                    );
                }
            }
        }
        best_puzzles[0].0.to_owned()
    }
}

pub trait EvalValue: Ord + Copy + Clone + Default + Send + Sync {
    fn to_str(&self) -> String;
    fn to_path(&self) -> String;
}

pub trait Evaluator: Clone + Send + Sync + Debug {
    type Value: EvalValue;
    fn evaluate(&self, puzzle: &Puzzle, result: &SolveResult) -> Self::Value;
}

#[derive(Debug, Clone)]
pub struct ShrinkStepEvaluator {}
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct ShrinkStepValue(usize, usize, usize);
impl EvalValue for ShrinkStepValue {
    fn to_str(&self) -> String {
        format!("first={} shrink={} all={}", self.0, self.1, self.2)
    }
    fn to_path(&self) -> String {
        format!("F{}S{}A{}", self.0, self.1, self.2)
    }
}

impl Evaluator for ShrinkStepEvaluator {
    type Value = ShrinkStepValue;
    fn evaluate(&self, puzzle: &Puzzle, result: &SolveResult) -> Self::Value {
        let moves = result.moves(puzzle);
        let shrink_moves = result.shrink_move(&moves);
        let first = first_remove(&shrink_moves);
        ShrinkStepValue(first, shrink_moves.len(), moves.len())
    }
}

#[derive(Debug, Clone)]
pub struct DupDropEvaluator {}
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct DupDropValue(usize, usize, usize, usize);
impl EvalValue for DupDropValue {
    fn to_str(&self) -> String {
        format!(
            "first={} dup={} shrink={} all={}",
            self.0, self.1, self.2, self.3
        )
    }
    fn to_path(&self) -> String {
        format!("F{}D{}S{}A{}", self.0, self.1, self.2, self.3)
    }
}
impl Evaluator for DupDropEvaluator {
    type Value = DupDropValue;
    fn evaluate(&self, puzzle: &Puzzle, result: &SolveResult) -> Self::Value {
        let moves = result.moves(puzzle);
        let shrink_moves = result.shrink_move(&moves);
        let first = first_remove(&shrink_moves);
        let drop_count = drop_count(&shrink_moves);
        DupDropValue(first, drop_count, shrink_moves.len(), moves.len())
    }
}

pub trait PuzzleGenerator: Clone + Send + Sync + Debug {
    fn generate(&self, puzzle: &Puzzle) -> Puzzle;
}

#[derive(Clone, Debug)]
pub struct SwapPuzzleGenerator<C: PuzzleConstraints> {
    pub constraints: C,
}
impl<C: PuzzleConstraints> PuzzleGenerator for SwapPuzzleGenerator<C> {
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

            if !self.constraints.is_ok(&pieces) {
                continue 'retry;
            }
            break;
        }
        let mut puzzle = puzzle.clone();
        puzzle.pieces = pieces.clone();
        puzzle
    }
}

#[derive(Clone, Debug)]
pub struct SwapNPuzzleGenerator<C: PuzzleConstraints> {
    pub swaps: usize,
    pub constraints: C,
}
impl<C: PuzzleConstraints> PuzzleGenerator for SwapNPuzzleGenerator<C> {
    fn generate(&self, puzzle: &Puzzle) -> Puzzle {
        let mut puzzle = puzzle.clone();
        for _ in 0..self.swaps {
            puzzle = SwapPuzzleGenerator {
                constraints: self.constraints.clone(),
            }
            .generate(&puzzle);
        }
        puzzle
    }
}

trait PuzzleConstraints: Clone + Send + Sync + Debug {
    fn is_ok(&self, pieces: &[Piece]) -> bool;
}
#[derive(Clone, Debug)]
pub struct MinPuzzleSizeConstraints<C: PuzzleConstraints> {
    pub size: usize,
    pub next: C,
}
impl<C: PuzzleConstraints> PuzzleConstraints for MinPuzzleSizeConstraints<C> {
    fn is_ok(&self, pieces: &[Piece]) -> bool {
        pieces.iter().all(|piece| piece.block.count() >= self.size) && self.next.is_ok(pieces)
    }
}
#[derive(Clone, Debug)]
pub struct TerminalPuzzleConstraints {}
impl PuzzleConstraints for TerminalPuzzleConstraints {
    fn is_ok(&self, _pieces: &[Piece]) -> bool {
        true
    }
}

fn first_remove(moves: &[ShrinkMove]) -> usize {
    for i in 0..moves.len() {
        if let ShrinkMove::Remove(_, _) = moves[i] {
            return i;
        }
    }
    unreachable!("puzzle is not solved")
}

fn drop_count(moves: &[ShrinkMove]) -> usize {
    let mut drops = 0;
    let mut touch_pieces: Vec<usize> = vec![];
    let mut touch_count = 0;
    for i in 0..moves.len() {
        match &moves[i] {
            ShrinkMove::Remove(_, _) => {
                let pieces_count = touch_pieces.iter().unique().count();
                drops += pieces_count * pieces_count * touch_count;
                touch_pieces.clear();
                touch_count = 0;
            }
            ShrinkMove::Shift(p, _) => {
                touch_pieces.extend(p);
                touch_count += 1;
            }
        }
    }
    drops
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn puzzle_searcher() {
        let searcher = PuzzleSearcher::new(
            10,
            1,
            Puzzle::base(3, 4, 1, None),
            10000,
            SwapPuzzleGenerator {
                constraints: TerminalPuzzleConstraints {},
            },
            ShrinkStepEvaluator {},
        );
        let puzzle = searcher.search();
        println!("Found\n{}", puzzle.to_str());
        let result = puzzle.solve();
        assert!(result.ok);
        println!("Moves {:?}", result.moves(&puzzle));
        let shrink = result.shrink_move(&result.moves(&puzzle));
        println!("Shrink #{} {:?}", shrink.len(), shrink);
    }

    #[test]
    fn puzzle_generator() {
        let holes = 5;
        let mut puzzle = Puzzle::base(3, 4, holes, None);
        let puzzle_generator = SwapPuzzleGenerator {
            constraints: TerminalPuzzleConstraints {},
        };
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

    #[test]
    fn test_drop_count() {
        use ShrinkMove::*;
        let moves = vec![
            Shift(vec![0], vec![(0, 0, 0)]),
            Remove(0, (0, 0, 0)),
            Shift(vec![1], vec![(0, 0, 0)]),
            Shift(vec![2], vec![(0, 0, 0)]),
            Remove(1, (0, 0, 0)),
        ];
        let count = drop_count(&moves);
        assert_eq!(count, 1 + 2 * 2 * 2);
    }
}
