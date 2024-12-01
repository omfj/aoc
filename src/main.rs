use crate::utils::read_input;
use crate::utils::AdventDay;
use clap::Parser;

pub mod utils;
pub mod y2024;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    year: i32,

    #[arg(short, long)]
    day: i32,
}

fn main() {
    let args = Args::parse();
    let (year, day) = (args.year, args.day);
    let input = read_input(year, day);
    match (year, day) {
        // Advent of Code 2024
        (2024, 1) => y2024::day01::Day01::new(input).run(),
        // End of match
        _ => println!("No implementation for year {} day {}", year, day),
    }
}
