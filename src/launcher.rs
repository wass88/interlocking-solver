use crate::{
    puzzle::{self, *},
    puzzle_num_format::PuzzleNumFormat,
    searcher::*,
    server::PuzzleJson,
};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub struct Launcher<G: PuzzleGenerator, E: Evaluator> {
    searcher: PuzzleSearcher<G, E>,
    parallel: usize,
}

trait PuzzleWriter {
    async fn write<V: EvalValue>(&self, puzzle: &Puzzle, result: &SolveResult, value: V);
    async fn write_config(&self, log: &str);
}

pub struct PuzzleFileWriter {
    dir: String,
}

impl PuzzleFileWriter {
    pub fn new(dir: String) -> Self {
        std::fs::create_dir_all(&dir).unwrap();
        Self { dir }
    }
}
impl PuzzleWriter for PuzzleFileWriter {
    async fn write<V: EvalValue>(&self, puzzle: &Puzzle, result: &SolveResult, value: V) {
        let date = chrono::Local::now();
        let date_path = date.format("%Y%m%dT%H%M%S").to_string();
        let path = format!("{}/{}_step_{}.pcad", self.dir, date_path, value.to_path());
        let moves = result.moves(puzzle);
        let shrink_moves = result.shrink_move(&moves);
        let pcad = format!("{}\n\n//{:?}", puzzle.to_pcad(), shrink_moves);
        println!("write to {}", path);
        std::fs::write(path, pcad).unwrap();
    }
    async fn write_config(&self, log: &str) {
        let path = format!("{}/config.log", self.dir);
        std::fs::write(path, log).unwrap();
    }
}

pub struct DBWriter {
    client: mongodb::Client,
    run: String,
}
impl DBWriter {
    pub async fn new(uri: &str, run: &str) -> Self {
        let client = mongodb::Client::with_uri_str(uri).await.unwrap();
        Self {
            client,
            run: run.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct RunMetaJson {
    #[serde(rename = "_id", skip_serializing)]
    id: Option<ObjectId>,
    run: String,
    date: String,
    config: String,
}
impl PuzzleWriter for DBWriter {
    async fn write<V: EvalValue>(&self, puzzle: &Puzzle, _result: &SolveResult, value: V) {
        let mut puzzle_json = PuzzleJson::normalized_from_puzzle(puzzle);
        puzzle_json.run = self.run.clone();
        let date = chrono::Local::now();
        let date_path = date.format("%Y%m%dT%H%M%S").to_string();
        let name = format!("{}_step_{}", date_path, value.to_path());
        puzzle_json.name = name;
        puzzle_json.date = date.to_rfc3339();

        let db = self.client.database("puzzle");
        let collection = db.collection("generated");
        collection.insert_one(puzzle_json, None).await.unwrap();
    }
    async fn write_config(&self, log: &str) {
        let meta = RunMetaJson {
            id: None,
            run: self.run.clone(),
            date: chrono::Local::now().to_rfc3339(),
            config: log.to_owned(),
        };
        let db = self.client.database("puzzle");
        let collection = db.collection("run_meta");
        collection.insert_one(meta, None).await.unwrap();
    }
}

impl<G: PuzzleGenerator, E: Evaluator> Launcher<G, E> {
    pub fn new(searcher: PuzzleSearcher<G, E>, parallel: usize) -> Self {
        Self { searcher, parallel }
    }
}

impl<G: PuzzleGenerator + 'static, E: Evaluator + 'static> Launcher<G, E> {
    pub async fn launch<W: PuzzleWriter>(&self, writer: &W) -> Result<(), String> {
        writer.write_config(&format!("{:#?}", self.searcher)).await;
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
                writer.write(&puzzle, &result, value).await;
            }
        }
        Ok(())
    }
}
