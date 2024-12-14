use crate::utils::AdventDay;
use std::collections::HashSet;

pub struct Day06 {
    input: String,
}

impl AdventDay for Day06 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let mut start = 0;
        let size = 4;

        for i in 0..self.input.len() - 1 {
            let chars = self
                .input
                .chars()
                .skip(i)
                .take(size)
                .collect::<HashSet<_>>();
            if chars.len() == size {
                start = i + size;
                break;
            }
        }

        start.to_string()
    }

    fn part_two(&self) -> String {
        let mut start = 0;
        let size = 14;

        for i in 0..self.input.len() - 1 {
            let chars = self
                .input
                .chars()
                .skip(i)
                .take(size)
                .collect::<HashSet<_>>();
            if chars.len() == size {
                start = i + size;
                break;
            }
        }

        start.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let day06_1 = Day06::new("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string());
        assert_eq!(day06_1.part_one(), "7");

        let day06_2 = Day06::new("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string());
        assert_eq!(day06_2.part_one(), "5");

        let day06 = Day06::new("nppdvjthqldpwncqszvftbrmjlhg".to_string());
        assert_eq!(day06.part_one(), "6");

        let day06_4 = Day06::new("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string());
        assert_eq!(day06_4.part_one(), "10");

        let day06_5 = Day06::new("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string());
        assert_eq!(day06_5.part_one(), "11");
    }

    #[test]
    fn part_two() {
        let day06 = Day06::new("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string());
        assert_eq!(day06.part_two(), "19");
    }
}
