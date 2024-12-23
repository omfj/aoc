use crate::utils::AdventDay;

fn parse_input(input: &str) -> Vec<(i32, i32, i32)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split('x');
            let l = parts.next().unwrap().parse().unwrap();
            let w = parts.next().unwrap().parse().unwrap();
            let h = parts.next().unwrap().parse().unwrap();
            (l, w, h)
        })
        .collect()
}

pub struct Day02 {
    input: String,
}

impl AdventDay for Day02 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let boxes = parse_input(&self.input);

        boxes
            .iter()
            .map(|(l, w, h)| {
                let lw = l * w;
                let wh = w * h;
                let hl = h * l;

                let sides = [lw, wh, hl];
                let slack = sides.iter().min().unwrap();

                2 * lw + 2 * wh + 2 * hl + slack
            })
            .sum::<i32>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let boxes = parse_input(&self.input);

        boxes
            .iter()
            .map(|(l, w, h)| {
                let ribbon = 2 * [*l + *w, *w + *h, *h + *l].iter().min().unwrap();
                let bow = l * w * h;
                ribbon + bow
            })
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let day02 = Day02::new("2x3x4".to_string());
        assert_eq!(day02.part_one(), "58");
    }

    #[test]
    fn part_two() {
        let day02 = Day02::new("2x3x4".to_string());
        assert_eq!(day02.part_two(), "34");

        let day02 = Day02::new("1x1x10".to_string());
        assert_eq!(day02.part_two(), "14");
    }
}
