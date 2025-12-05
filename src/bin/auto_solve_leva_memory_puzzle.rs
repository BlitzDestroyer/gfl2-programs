use clap::Parser;

use gfl2_programs::leva_memory_puzzle::{Attempts, LevaPuzzleClient, roll_gacha, solve_puzzle};

#[derive(Parser, Debug)]
#[command(version = "v1.0.0", about = "Solves Leva's Memory Puzzle automatically", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "", help = "Authentication token for the Leva Memory Puzzle")]
    auth_token: String,
    #[arg(default_value_t = Attempts::One, help = "Number of attempts to solve the puzzle")]
    attempts: Attempts,
    #[arg(short, long, alias = "gacha", default_value_t = Attempts::None, help = "Number of attempts to roll the gacha")]
    gacha_attempts: Attempts,
}

fn main() {
    let args = Args::parse();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let client = LevaPuzzleClient::new(&args.auth_token);
    let client = match client {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error creating LevaPuzzleClient: {}", e);
            return;
        }
    };

    match runtime.block_on(solve_puzzle(&client, args.attempts)) {
        Ok(_) => {
            if !matches!(args.attempts, Attempts::None) {
                println!("Puzzle(s) solved successfully!");
            }
        },
        Err(e) => eprintln!("Error solving puzzle(s): {}", e),
    }

    match runtime.block_on(roll_gacha(&client, args.gacha_attempts)) {
        Ok(_) => {
            if !matches!(args.gacha_attempts, Attempts::None) {
                println!("Gacha rolling completed successfully!")
            }
        },
        Err(e) => eprintln!("Error rolling gacha: {}", e),
    }
}