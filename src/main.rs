use clap::{Parser, Subcommand};
use unicode_segmentation::UnicodeSegmentation;
use unicode_normalization::UnicodeNormalization;
use std::io::{self, Read, Write};
use std::fs::File;
use std::net::SocketAddr;

#[derive(Parser)]
#[command(name = "flip")]
#[command(version = "1.0.0")]
struct Cli {
    #[arg(short, long, global = true)]
    flipe: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Text { text: Vec<String> },
    Serve { #[arg(short, long, default_value = "127.0.0.1:8080")] addr: String },
}

fn flip_text(s: &str, emoji: bool) -> String {
    if emoji {
        let n = s.nfc().collect::<String>();
        n.graphemes(true).rev().collect()
    } else {
        s.chars().rev().collect()
    }
}

fn run_cli(flipe: bool, text: Vec<String>) {
    let input = if text.is_empty() {
        let mut b = String::new();
        let _ = io::stdin().read_to_string(&mut b);
        b
    } else {
        text.join(" ")
    };
    let flipped = flip_text(&input, flipe);
    println!("{flipped}");
    print!("Save to flipped-thing.txt? (y/N): ");
    io::stdout().flush().ok();
    let mut a = String::new();
    io::stdin().read_line(&mut a).ok();
    let a = a.trim().to_lowercase();
    if a == "y" || a == "yes" {
        if let Ok(mut f) = File::create("flipped-thing.txt") {
            let _ = f.write_all(flipped.as_bytes());
        }
    }
}

async fn run_server(addr: &str, flipe_flag: bool) {
    use axum::{routing::post, Router, extract::Query, body::Bytes};
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Params { flipe: Option<u8> }

    async fn handler(Query(p): Query<Params>, body: Bytes, f: bool) -> String {
        let t = String::from_utf8_lossy(&body);
        let e = p.flipe == Some(1) || f;
        flip_text(&t, e)
    }

    let app = Router::new().route("/flip", post(move |q, b| handler(q, b, flipe_flag)));
    let socket: SocketAddr = addr.parse().expect("bad addr");
    println!("Serving on http://{addr}");
    let listener = tokio::net::TcpListener::bind(socket).await.expect("bind fail");
    axum::serve(listener, app).await.unwrap();
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Text { text } => run_cli(cli.flipe, text),
        Commands::Serve { addr } => run_server(&addr, cli.flipe).await,
    }
}
