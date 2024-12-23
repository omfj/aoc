use crate::generate::generate;
use crate::run::run;
use clap::{Parser, Subcommand};

pub mod generate;
pub mod run;
pub mod utils;
pub mod y2022;
pub mod y2023;
pub mod y2024;

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
        Commands::Run { year, day } => run(year, day),
        Commands::Generate { year, day } => generate(year, day),
    }
}
