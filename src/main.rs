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

fn launch_generate() {
    let constraints = MinPuzzleSizeConstraints {
        size: 2,
        next: TerminalPuzzleConstraints {},
    };
    let searcher = PuzzleSearcher::new(
        1000000,
        1,
        Puzzle::base(4, 8, 0, Some(50000)),
        30000,
        SwapNPuzzleGenerator {
            swaps: 6,
            constraints,
        },
        ShrinkStepEvaluator {},
    );
    let launcher = Launcher::new(searcher, 4);
    let writer = launcher::PuzzleWriter::new("puzzles/puzzle_20240122_4x4_8_coshirnk".to_string());
    launcher.launch(&writer).unwrap();
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

async fn launch_server() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/api/hello", get(hello))
        .route("/api/puzzles", get(server::puzzles));
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
        "generate" => launch_generate(),
        "gen_all" => launch_gen_all_puzzles(),
        "solve_sample" => solve_sample_puzzle(),
        _ => launch_server().await,
    }
}
