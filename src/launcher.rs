use crate::{puzzle::*, searcher::*};

pub struct Launcher<G: PuzzleGenerator, E: Evaluator> {
    searcher: PuzzleSearcher<G, E>,
    parallel: usize,
}

pub struct PuzzleWriter {
    dir: String,
}
impl PuzzleWriter {
    pub fn new(dir: String) -> Self {
        std::fs::create_dir_all(&dir).unwrap();
        Self { dir }
    }
    pub fn write<V: EvalValue>(&self, puzzle: &Puzzle, result: &SolveResult, value: V) {
        let date = chrono::Local::now();
        let date_path = date.format("%Y%m%dT%H%M%S").to_string();
        let path = format!("{}/{}_step_{}.pcad", self.dir, date_path, value.to_path());
        let moves = result.moves(puzzle);
        let shrink_moves = result.shrink_move(&moves);
        let pcad = format!("{}\n\n//{:?}", puzzle.to_pcad(), shrink_moves);
        println!("write to {}", path);
        std::fs::write(path, pcad).unwrap();
    }
}

impl<G: PuzzleGenerator, E: Evaluator> Launcher<G, E> {
    pub fn new(searcher: PuzzleSearcher<G, E>, parallel: usize) -> Self {
        Self { searcher, parallel }
    }
}

impl<G: PuzzleGenerator + 'static, E: Evaluator + 'static> Launcher<G, E> {
    pub fn launch(&self, writer: &PuzzleWriter) -> Result<(), String> {
        let (tx, rx) = std::sync::mpsc::channel();
        for _ in 0..self.parallel {
            let tx = tx.clone();
            let searcher = self.searcher.clone();
            std::thread::spawn(move || loop {
                let puzzle = searcher.search();
                tx.send(puzzle).unwrap();
            });
        }
        for puzzle in rx {
            let result = puzzle.solve();
            if result.ok {
                let value = self.searcher.evaluator.evaluate(&puzzle, &result);
                writer.write(&puzzle, &result, value);
            }
        }
        Ok(())
    }
}
