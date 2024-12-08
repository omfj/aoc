use crate::utils::AdventDay;
use std::collections::{HashMap, HashSet};

fn collect_antennas(map: &Vec<Vec<char>>) -> Vec<(usize, usize, char)> {
    let mut antennas = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch.is_alphanumeric() {
                antennas.push((x, y, ch));
            }
        }
    }

    antennas
}

fn collect_antinodes(
    antennas: &Vec<(usize, usize, char)>,
    width: usize,
    height: usize,
) -> HashSet<(usize, usize)> {
    let mut antinodes = HashSet::new();
    let mut groups: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for &(x, y, freq) in antennas {
        groups.entry(freq).or_default().push((x, y));
    }

    for positions in groups.values() {
        let len = positions.len();
        for i in 0..len {
            let (x1, y1) = positions[i];

            for j in 0..len {
                if i == j {
                    continue;
                }

                let (x2, y2) = positions[j];

                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                antinodes.insert((x1, y1));
                antinodes.insert((x2, y2));

                let mut x_prev = x1 as isize;
                let mut y_prev = y1 as isize;
                while x_prev >= 0
                    && y_prev >= 0
                    && x_prev < width as isize
                    && y_prev < height as isize
                {
                    antinodes.insert((x_prev as usize, y_prev as usize));
                    x_prev -= dx;
                    y_prev -= dy;
                }

                let mut x_next = x1 as isize;
                let mut y_next = y1 as isize;
                while x_next >= 0
                    && y_next >= 0
                    && x_next < width as isize
                    && y_next < height as isize
                {
                    antinodes.insert((x_next as usize, y_next as usize));
                    x_next += dx;
                    y_next += dy;
                }
            }
        }
    }

    antinodes
}

fn is_valid_antinode(antennas: &Vec<(usize, usize, char)>, x: usize, y: usize) -> bool {
    for &(x1, y1, freq1) in antennas {
        for &(x2, y2, freq2) in antennas {
            if freq1 == freq2 && (x1, y1) != (x2, y2) {
                let dx1 = x as isize - x1 as isize;
                let dy1 = y as isize - y1 as isize;
                let dx2 = x2 as isize - x as isize;
                let dy2 = y2 as isize - y as isize;
                let is_colinear = dx1 * dy2 == dy1 * dx2;

                if is_colinear {
                    let dist1 = dx1.abs() + dy1.abs();
                    let dist2 = dx2.abs() + dy2.abs();

                    if dist1 == 2 * dist2 || dist2 == 2 * dist1 {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn get_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub struct Day08 {
    input: String,
}

impl AdventDay for Day08 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let map = get_map(&self.input);
        let (height, width) = (map.len(), map[0].len());

        let antennas = collect_antennas(&map);
        let antinodes = collect_antinodes(&antennas, width, height);

        let valid_antinodes: HashSet<(usize, usize)> = antinodes
            .into_iter()
            .filter(|&(x, y)| is_valid_antinode(&antennas, x, y))
            .collect();

        valid_antinodes.len().to_string()
    }

    fn part_two(&self) -> String {
        let map = get_map(&self.input);
        let (height, width) = (map.len(), map[0].len());

        let antennas = collect_antennas(&map);
        let antinodes = collect_antinodes(&antennas, width, height);

        antinodes.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn part_one() {
        let day08 = Day08::new(DATA.to_string());
        assert_eq!(day08.part_one(), "14");
    }

    #[test]
    fn part_two() {
        let day08 = Day08::new(DATA.to_string());
        assert_eq!(day08.part_two(), "34");
    }
}
