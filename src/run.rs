use crate::utils::AdventDay;
use crate::y2015;
use crate::y2022;
use crate::y2023;
use crate::y2024;
use std::fs;

pub fn read_input(year: i32, day: i32) -> String {
    fs::read_to_string(format!("data/inputs/{}/day{:02}.input.txt", year, day))
        .unwrap_or_else(|_| panic!("Could not read file for year {} day {}", year, day))
}

pub fn run(year: i32, day: i32) {
    let input = read_input(year, day);

    match (year, day) {
        // Advent of Code 2015
        (2015, 1) => y2015::day01::Day01::new(input).run(),
        (2015, 2) => y2015::day02::Day02::new(input).run(),

        // Advent of Code 2022
        (2022, 1) => y2022::day01::Day01::new(input).run(),
        (2022, 2) => y2022::day02::Day02::new(input).run(),
        (2022, 3) => y2022::day03::Day03::new(input).run(),
        (2022, 4) => y2022::day04::Day04::new(input).run(),
        (2022, 5) => y2022::day05::Day05::new(input).run(),
        (2022, 6) => y2022::day06::Day06::new(input).run(),

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
        (2024, 10) => y2024::day10::Day10::new(input).run(),
        (2024, 11) => y2024::day11::Day11::new(input).run(),
        (2024, 12) => y2024::day12::Day12::new(input).run(),
        (2024, 13) => y2024::day13::Day13::new(input).run(),
        (2024, 14) => y2024::day14::Day14::new(input).run(),
        (2024, 15) => y2024::day15::Day15::new(input).run(),
        (2024, 16) => y2024::day16::Day16::new(input).run(),
        (2024, 17) => y2024::day17::Day17::new(input).run(),
        (2024, 18) => y2024::day18::Day18::new(input).run(),
        (2024, 19) => y2024::day19::Day19::new(input).run(),
        (2024, 20) => y2024::day20::Day20::new(input).run(),
        (2024, 21) => y2024::day21::Day21::new(input).run(),
        (2024, 22) => y2024::day22::Day22::new(input).run(),
        (2024, 23) => y2024::day23::Day23::new(input).run(),
        _ => println!("No implementation for year {} day {}", year, day),
    }
}
