use std::fs;
use std::time::Instant;

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
    let start = Instant::now();
    let output = f();
    let elapsed = start.elapsed();

    if output.is_empty() {
        println!("{} not implemented", part);
    } else {
        println!("{}: {} ({:?})", part, output, elapsed);
    }
}

pub fn read_input(year: i32, day: i32) -> String {
    fs::read_to_string(format!("data/inputs/{}/day{:02}.input.txt", year, day)).expect(&format!(
        "Could not read file for year {} day {}",
        year, day
    ))
}
