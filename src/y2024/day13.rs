use crate::utils::AdventDay;
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
struct Machine {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
}

impl Machine {
    /// Least amount of presses of button A and B to reach the prize
    /// If no solution is found, it will return (0, 0)
    fn cheapest_to_prize(&self) -> (isize, isize) {
        let (a1, a2) = self.a;
        let (b1, b2) = self.b;
        let (x, y) = self.prize;

        let det = a1 * b2 - a2 * b1;

        // Singular matrix, no solution
        if det == 0 {
            return (0, 0);
        }

        let m_num = x * b2 - y * b1;
        let n_num = -x * a2 + y * a1;

        if m_num % det != 0 || n_num % det != 0 {
            return (0, 0);
        }

        let m = m_num / det;
        let n = n_num / det;

        match m.is_positive() && n.is_positive() {
            true => (m, n),
            _ => (0, 0),
        }
    }
}

fn parse_button_line(line: &str) -> (isize, isize) {
    let line = line.split(": ").nth(1).unwrap();
    let (x_part, y_part) = line.split(", ").collect_tuple::<(&str, &str)>().unwrap();

    let x_str = x_part.split('+').nth(1).unwrap();
    let y_str = y_part.split('+').nth(1).unwrap();

    let x = x_str.parse::<isize>().unwrap();
    let y = y_str.parse::<isize>().unwrap();

    (x, y)
}

fn parse_prize_line(line: &str) -> (isize, isize) {
    let after_colon = line.split(": ").nth(1).unwrap();

    let (x_part, y_part) = after_colon
        .split(", ")
        .collect_tuple::<(&str, &str)>()
        .unwrap();

    let x_str = x_part.split('=').nth(1).unwrap();
    let y_str = y_part.split('=').nth(1).unwrap();

    let x = x_str.parse::<isize>().unwrap();
    let y = y_str.parse::<isize>().unwrap();

    (x, y)
}

fn collect_machines(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|chunk| {
            let lines: Vec<&str> = chunk.lines().collect();
            let button_a = parse_button_line(lines[0]);
            let button_b = parse_button_line(lines[1]);
            let prize = parse_prize_line(lines[2]);

            Machine {
                a: button_a,
                b: button_b,
                prize,
            }
        })
        .collect()
}

pub struct Day13 {
    input: String,
}

impl AdventDay for Day13 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let mut total = 0;
        let machines = collect_machines(&self.input);

        for machine in machines {
            let (a, b) = machine.cheapest_to_prize();
            total += (a * 3) + (b * 1)
        }

        total.to_string()
    }

    fn part_two(&self) -> String {
        let mut total = 0;
        let machines = collect_machines(&self.input)
            .iter()
            .map(|machine| {
                let mut machine = *machine;
                let (x, y) = machine.prize;
                machine.prize = (x + 10000000000000, y + 10000000000000);

                machine
            })
            .collect::<Vec<Machine>>();

        for machine in machines {
            let (a, b) = machine.cheapest_to_prize();
            total += (a * 3) + (b * 1)
        }

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    #[test]
    fn part_one_and_two() {
        let day13 = Day13::new(DATA.to_string());
        assert_eq!(day13.part_one(), "480");
    }
}
