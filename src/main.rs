use std::time::Instant;

use crate::utils::read_input;
use crate::utils::AdventDay;

use clap::Parser;

pub mod utils;
pub mod y2022;
pub mod y2023;
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
    let start_time = Instant::now();
    let (year, day) = (args.year, args.day);
    let input = read_input(year, day);
    match (year, day) {
        // Advent of Code 2022
        (2022, 1) => y2022::day01::Day01::new(input).run(),
        (2022, 2) => y2022::day02::Day02::new(input).run(),
        (2022, 3) => y2022::day03::Day03::new(input).run(),
        (2022, 4) => y2022::day04::Day04::new(input).run(),

        // Advent of Code 2023
        (2023, 1) => y2023::day01::Day01::new(input).run(),
        (2023, 2) => y2023::day02::Day02::new(input).run(),

        // Advent of Code 2024
        (2024, 1) => y2024::day01::Day01::new(input).run(),
        (2024, 2) => y2024::day02::Day02::new(input).run(),
        (2024, 3) => y2024::day03::Day03::new(input).run(),
        (2024, 4) => y2024::day04::Day04::new(input).run(),
        (2024, 5) => y2024::day05::Day05::new(input).run(),
        (2024, 6) => y2024::day06::Day06::new(input).run(),
        (2024, 7) => y2024::day07::Day07::new(input).run(),
        (2024, 8) => y2024::day08::Day08::new(input).run(),
        (2024, 9) => y2024::day09::Day09::new(input).run(),
        _ => println!("No implementation for year {} day {}", year, day),
    }

    let elapsed = start_time.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);
}
