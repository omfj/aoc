use std::fs;
use std::time::Instant;

pub trait AdventDay {
    fn new(input: String) -> Self;
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
    fn run(&self) {
        let start_one = Instant::now();
        let part_one = self.part_one();
        let elapsed_one = start_one.elapsed();

        if part_one.is_empty() {
            println!("Part one not implemented");
        } else {
            println!("Part one: {}", part_one);
            println!("Part one took {:?}", elapsed_one);
        }

        let start_two = Instant::now();
        let part_two = self.part_two();
        let elapsed_two = start_two.elapsed();

        if part_two.is_empty() {
            println!("Part two not implemented");
        } else {
            println!("Part two: {}", part_two);
            println!("Part two took {:?}", elapsed_two);
        }
    }
}

pub fn read_input(year: i32, day: i32) -> String {
    fs::read_to_string(format!("data/inputs/{}/day{:02}.input.txt", year, day))
        .unwrap_or_else(|_| panic!("Could not read file for year {} day {}", year, day))
}
