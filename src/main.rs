mod cells;
mod gen_all_puzzles;
mod iters;
mod launcher;
mod puzzle;
mod puzzle_num_format;
mod searcher;
mod server;
mod v3;

use std::env;

use axum::{routing::get, Router};
use launcher::Launcher;
use puzzle::Puzzle;
use searcher::*;

use crate::server::sample_puzzle;

async fn launch_generate_db() {
    let constraints = MinPuzzleSizeConstraints {
        size: 2,
        next: TerminalPuzzleConstraints {},
    };
    let searcher = PuzzleSearcher::new(
        1000000,
        1,
        Puzzle::base(4, 5, 2, Some(1000)),
        500000,
        SwapNPuzzleGenerator {
            swaps: 3,
            constraints,
        },
        ShrinkStepEvaluator {},
    );
    let launcher = Launcher::new(searcher, 4);
    let writer = launcher::DBWriter::new(&get_mongo_uri(), &"500k_retry").await;
    launcher.launch(&writer).await.unwrap();
}

async fn launch_generate_file() {
    let constraints = MinPuzzleSizeConstraints {
        size: 2,
        next: TerminalPuzzleConstraints {},
    };
    let searcher = PuzzleSearcher::new(
        1000000,
        1,
        Puzzle::base(4, 5, 2, Some(1000)),
        100000,
        SwapNPuzzleGenerator {
            swaps: 3,
            constraints,
        },
        ShrinkStepEvaluator {},
    );
    let launcher = Launcher::new(searcher, 4);
    let writer =
        launcher::PuzzleFileWriter::new("puzzles/puzzle_20240122_4x4_5_swap3ok".to_string());
    launcher.launch(&writer).await.unwrap();
}

fn launch_gen_all_puzzles() {
    let mut writer = gen_all_puzzles::DebugWriter::new();
    gen_all_puzzles::GenAllPuzzles {
        size: 3,
        piece: 3,
        holes: 0,
    }
    .generate(&mut writer);
    println!("done");
    println!("{}", writer.codes.len());
}

fn solve_sample_puzzle() {
    let puzzle = sample_puzzle();
    assert!(puzzle.check_puzzle());
    let result = puzzle.solve();
    let moves = result.moves(&puzzle);
    println!("{:?}", moves);
}

pub fn get_mongo_uri() -> String {
    let mongo_uri = env::var("MONGO_URI").unwrap();
    assert!(
        mongo_uri.starts_with("mongodb://"),
        "MONGO_URI must start with mongodb:// ({})",
        mongo_uri
    );
    mongo_uri
}
async fn launch_server() {
    tracing_subscriber::fmt::init();
    let mongo_uri = get_mongo_uri();
    let client = mongodb::Client::with_uri_str(&mongo_uri).await.unwrap();
    let app = Router::new()
        .route("/api/hello", get(hello))
        .route("/api/puzzles", get(server::puzzles))
        .with_state(client);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:13013")
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let cmd = env::args().collect::<Vec<_>>();
    match &cmd.get(1).unwrap_or(&"".to_owned())[..] {
        "generate" => launch_generate_db().await,
        "generate_file" => launch_generate_file().await,
        "gen_all" => launch_gen_all_puzzles(),
        "solve_sample" => solve_sample_puzzle(),
        _ => launch_server().await,
    }
}
