use clap::Parser;

use gfl2_programs::leva_memory_puzzle::solve_puzzle;

#[derive(Parser, Debug)]
#[command(version = "v1.0.0", about = "Solves Leva's Memory Puzzle automatically", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "", help = "Authentication token for the Leva Memory Puzzle")]
    auth_token: String,
}

fn main() {
    let args = Args::parse();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    match runtime.block_on(solve_puzzle(&args.auth_token)) {
        Ok(_) => println!("Puzzle solved successfully!"),
        Err(e) => eprintln!("Error solving puzzle: {}", e),
    }
}