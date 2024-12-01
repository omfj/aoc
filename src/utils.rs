use std::fs;

pub trait AdventDay {
    fn new(input: String) -> Self;
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
    fn run(&self) {
        let part_one = self.part_one();
        let part_two = self.part_two();

        if part_one.is_empty() {
            println!("Part one not implemented");
        } else {
            println!("Part one: {}", part_one);
        }

        if part_two.is_empty() {
            println!("Part two not implemented");
        } else {
            println!("Part two: {}", part_two);
        }
    }
}

pub fn read_input(year: i32, day: i32) -> String {
    fs::read_to_string(format!("data/inputs/{}/day{:02}.input.txt", year, day))
        .expect(format!("Could not read file for year {} day {}", year, day).as_str())
}
