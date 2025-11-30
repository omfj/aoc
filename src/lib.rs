use std::fs;

pub mod y2015;
pub mod y2022;
pub mod y2023;
pub mod y2024;
pub mod y2025;

pub fn read_input(year: i32, day: i32) -> String {
    fs::read_to_string(format!("data/inputs/{}/day{:02}.input.txt", year, day))
        .unwrap_or_else(|_| panic!("Could not read file for year {} day {}", year, day))
}

pub struct Runner;

impl Runner {
    pub fn run(year: i32, day: i32) {
        let input = read_input(year, day);

        match (year, day) {
            // Advent of Code 2015
            (2015, 1) => y2015::day01::Day01::new(input).run(),
            (2015, 2) => y2015::day02::Day02::new(input).run(),
            (2015, 3) => y2015::day03::Day03::new(input).run(),
            (2015, 4) => y2015::day04::Day04::new(input).run(),
            (2015, 5) => y2015::day05::Day05::new(input).run(),
            (2015, 6) => y2015::day06::Day06::new(input).run(),

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
            (2015, 7) => y2015::day07::Day07::new(input).run(),
            (2015, 8) => y2015::day08::Day08::new(input).run(),
            (2024, 24) => y2024::day24::Day24::new(input).run(),
            (2024, 25) => y2024::day25::Day25::new(input).run(),

            // Advent of Code 2025
            (2025, 1) => y2025::day01::Day01::new(input).run(),

            _ => println!("No implementation for year {} day {}", year, day),
        }
    }

    pub fn generate(year: i32, day: i32) {
        let day_file = format!("src/y{}/day{:02}.rs", year, day);
        let mod_file = format!("src/y{}/mod.rs", year);
        let run_file = "src/run.rs";

        let day_template = format!(
            r##"use crate::AdventDay;

pub struct Day{day:02} {{
    input: String,
}}

impl AdventDay for Day{day:02} {{
    fn new(input: String) -> Self {{
        Self {{ input }}
    }}

    fn part_one(&self) -> String {{
        todo!()
    }}

    fn part_two(&self) -> String {{
        todo!()
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    const DATA: &str = r#""#;

    #[test]
    fn part_one() {{
        let day{day:02} = Day{day:02}::new(DATA.to_string());
        assert_eq!(day{day:02}.part_one(), "");
    }}

    #[test]
    #[ignore]
    fn part_two() {{
        let day{day:02} = Day{day:02}::new(DATA.to_string());
        assert_eq!(day{day:02}.part_two(), "");
    }}
}}
"##,
        );

        if std::path::Path::new(&day_file).exists() {
            eprintln!("Day file {} already exists!", day_file);
            return;
        }

        fs::create_dir_all(format!("src/y{}", year)).expect("Failed to create year directory");
        fs::write(&day_file, day_template).expect("Failed to write day file");
        println!("Created {}", day_file);

        let mod_entry = format!("pub mod day{:02};", day);
        let mut mod_content = fs::read_to_string(&mod_file).unwrap_or_else(|_| String::new());
        if !mod_content.contains(&mod_entry) {
            mod_content.push_str(&format!("{}\n", mod_entry));
            fs::write(&mod_file, mod_content).expect("Failed to update mod.rs");
            println!("Updated {}", mod_file);
        }

        let new_match_arm = format!(
            r#"        ({}, {}) => y{}::day{:02}::Day{:02}::new(input).run(),"#,
            year, day, year, day, day
        );

        let mut main_content = fs::read_to_string(run_file).expect("Failed to read main.rs");
        let insertion_marker =
            "        _ => println!(\"No implementation for year {} day {}\", year, day),";
        if let Some(pos) = main_content.find(insertion_marker) {
            main_content.insert_str(pos, &format!("{}\n", new_match_arm));
            fs::write(run_file, main_content).expect("Failed to update main.rs");
            println!("Updated {}", run_file);
        } else {
            eprintln!("Could not find insertion marker in main.rs!");
        }

        println!("Day {:02} setup complete! File created: {}", day, day_file);
    }
}

pub trait AdventDay {
    fn new(input: String) -> Self;
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
    fn run(&self) {
        run_part("Part 1", || self.part_one());
        run_part("Part 2", || self.part_two());
    }
}

fn run_part(part: &str, f: impl FnOnce() -> String) {
    let start = std::time::Instant::now();
    let output = f();
    let elapsed = start.elapsed();

    if output.is_empty() {
        println!("{} not implemented", part);
    } else {
        println!("{}: {} ({:?})", part, output, elapsed);
    }
}
