use aoc::Runner;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    /// run a day
    Run {
        #[arg(short, long)]
        year: i32,

        #[arg(short, long)]
        day: i32,
    },

    /// generates a new day
    Generate {
        #[arg(short, long)]
        year: i32,

        #[arg(short, long)]
        day: i32,
    },
}

fn main() {
    let Args { command } = Args::parse();

    match command {
        Commands::Run { year, day } => Runner::run(year, day),
        Commands::Generate { year, day } => Runner::generate(year, day),
    }
}
