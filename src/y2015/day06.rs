use crate::utils::AdventDay;

type Point = (usize, usize);

#[derive(Copy, Clone)]
enum Instruction {
    TurnOn(Point, Point),
    TurnOff(Point, Point),
    Toggle(Point, Point),
}

fn parse_point(s: &str) -> Point {
    let parts: Vec<&str> = s.split(',').collect();
    (parts[0].parse().unwrap(), parts[1].parse().unwrap())
}

fn parse_instruction(s: &str) -> Instruction {
    let words: Vec<&str> = s.split_whitespace().collect();
    match words.as_slice() {
        ["turn", "on", start, "through", end] => {
            let start = parse_point(start);
            let end = parse_point(end);
            Instruction::TurnOn(start, end)
        }
        ["turn", "off", start, "through", end] => {
            let start = parse_point(start);
            let end = parse_point(end);
            Instruction::TurnOff(start, end)
        }
        ["toggle", start, "through", end] => {
            let start = parse_point(start);
            let end = parse_point(end);
            Instruction::Toggle(start, end)
        }
        _ => panic!("Invalid instruction: {}", s),
    }
}

fn apply_instruction(lights: &mut [Vec<u8>], instruction: Instruction) {
    match instruction {
        Instruction::TurnOn((x1, y1), (x2, y2)) => {
            for lights in lights.iter_mut().take(x2 + 1).skip(x1) {
                for light in lights.iter_mut().take(y2 + 1).skip(y1) {
                    *light = light.saturating_add(1);
                }
            }
        }
        Instruction::TurnOff((x1, y1), (x2, y2)) => {
            for lights in lights.iter_mut().take(x2 + 1).skip(x1) {
                for light in lights.iter_mut().take(y2 + 1).skip(y1) {
                    *light = light.saturating_sub(1);
                }
            }
        }
        Instruction::Toggle((x1, y1), (x2, y2)) => {
            for lights in lights.iter_mut().take(x2 + 1).skip(x1) {
                for light in lights.iter_mut().take(y2 + 1).skip(y1) {
                    *light = light.saturating_add(2);
                }
            }
        }
    }
}

fn apply_all_instructions(lights: &mut [Vec<u8>], instructions: &[Instruction]) {
    for instruction in instructions {
        apply_instruction(lights, *instruction);
    }
}

fn count_lights_on(lights: &[Vec<u8>]) -> usize {
    lights.iter().flatten().filter(|&&b| b > 0).count()
}

fn sum_brightness(lights: &[Vec<u8>]) -> usize {
    lights.iter().flatten().map(|&b| b as usize).sum()
}

pub struct Day06 {
    input: String,
}

impl AdventDay for Day06 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let instructions: Vec<_> = self.input.lines().map(parse_instruction).collect();
        let mut lights = vec![vec![0; 1000]; 1000];
        apply_all_instructions(&mut lights, &instructions);
        count_lights_on(&lights).to_string()
    }

    fn part_two(&self) -> String {
        let instructions: Vec<_> = self.input.lines().map(parse_instruction).collect();
        let mut lights = vec![vec![0; 1000]; 1000];
        apply_all_instructions(&mut lights, &instructions);
        sum_brightness(&lights).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"turn on 489,959 through 759,964
turn off 820,516 through 871,914
turn off 427,423 through 929,502
toggle 692,845 through 866,994"#;

    #[test]
    fn part_one() {
        let day06 = Day06::new(DATA.to_string());
        assert_eq!(day06.part_one(), "27468");
    }

    #[test]
    fn part_two() {
        let day06 = Day06::new(DATA.to_string());
        assert_eq!(day06.part_two(), "54126");
    }
}
