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
    /// Run a day
    Run {
        #[arg(short, long)]
        year: i32,

        #[arg(short, long)]
        day: i32,
    },

    /// Generates a new day
    Generate {
        #[arg(short, long)]
        year: i32,

        #[arg(short, long)]
        day: i32,
    },

    /// Fetch the input for a day
    /// Requires SESSION_TOKEN env variable to be set
    Fetch {
        #[arg(short, long)]
        year: i32,

        #[arg(short, long)]
        day: i32,
    },
}

fn main() {
    dotenvy::dotenv().ok();
    let cli = Args::parse();

    match cli.command {
        Commands::Run { year, day } => Runner::run(year, day),
        Commands::Generate { year, day } => Runner::generate(year, day),
        Commands::Fetch { year, day } => Runner::fetch_input(year, day),
    }
}
