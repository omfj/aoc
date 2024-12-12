use crate::utils::AdventDay;
use std::collections::HashSet;

/// Get the perimeter of a region
fn calculate_perimeter(region: HashSet<(usize, usize)>) -> usize {
    let mut perimeter = 0;

    for (i, j) in region.iter() {
        if !region.contains(&(i.wrapping_add(1), *j)) {
            perimeter += 1;
        }

        if !region.contains(&(i.wrapping_sub(1), *j)) {
            perimeter += 1;
        }

        if !region.contains(&(*i, j.wrapping_add(1))) {
            perimeter += 1;
        }

        if !region.contains(&(*i, j.wrapping_sub(1))) {
            perimeter += 1;
        }
    }

    perimeter
}

fn cell_in_region(region: &HashSet<(usize, usize)>, i: isize, j: isize) -> bool {
    if i < 0 || j < 0 {
        false
    } else {
        region.contains(&(i as usize, j as usize))
    }
}

fn count_corners(i: usize, j: usize, region: &HashSet<(usize, usize)>) -> usize {
    let mut count = 0;
    let corners = [
        ((0, 1), (1, 0), (1, 1)),
        ((1, 0), (0, -1), (1, -1)),
        ((0, -1), (-1, 0), (-1, -1)),
        ((-1, 0), (0, 1), (-1, 1)),
    ];

    let (ix, jy) = (i as isize, j as isize);

    for ((dx0, dy0), (dx1, dy1), (dx2, dy2)) in &corners {
        let c0 = cell_in_region(region, ix + dx0, jy + dy0);
        let c1 = cell_in_region(region, ix + dx1, jy + dy1);
        let c2 = cell_in_region(region, ix + dx2, jy + dy2);

        if !c0 && !c1 {
            count += 1;
        }

        if c0 && c1 && !c2 {
            count += 1;
        }
    }

    count
}

fn calculate_sides(region: HashSet<(usize, usize)>) -> usize {
    let mut sides = 0;

    for &(i, j) in region.iter() {
        sides += count_corners(i, j, &region);
    }

    sides
}

fn collect_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn collect_region(
    map: &[Vec<char>],
    visited: &mut Vec<(usize, usize)>,
    i: usize,
    j: usize,
) -> HashSet<(usize, usize)> {
    let char = map[i][j];
    let mut region = HashSet::new();
    let mut stack = vec![(i, j)];

    while let Some((i, j)) = stack.pop() {
        if i >= map.len() || j >= map[i].len() {
            continue;
        }

        if map[i][j] != char {
            continue;
        }

        if region.contains(&(i, j)) {
            continue;
        }

        if visited.contains(&(i, j)) {
            continue;
        }

        region.insert((i, j));
        visited.push((i, j));
        stack.push((i.wrapping_add(1), j));
        stack.push((i.wrapping_sub(1), j));
        stack.push((i, j.wrapping_add(1)));
        stack.push((i, j.wrapping_sub(1)));
    }

    region
}

/// Collect the prices of each region
fn collect_region_prices<F>(map: &[Vec<char>], calc_fn: F) -> Vec<usize>
where
    F: Fn(&HashSet<(usize, usize)>) -> usize,
{
    let mut visited: Vec<(usize, usize)> = Vec::new();
    let mut prices = Vec::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if visited.contains(&(i, j)) {
                continue;
            }

            let region = collect_region(map, &mut visited, i, j);
            prices.push(calc_fn(&region));
        }
    }

    prices
}

pub struct Day12 {
    input: String,
}

impl AdventDay for Day12 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let map = collect_map(&self.input);
        let prices = collect_region_prices(&map, |region| {
            region.len() * calculate_perimeter(region.clone())
        });
        prices.iter().sum::<usize>().to_string()
    }

    fn part_two(&self) -> String {
        let map = collect_map(&self.input);
        let prices = collect_region_prices(&map, |region| {
            let area = region.len();
            let sides = calculate_sides(region.clone());
            area * sides
        });
        prices.iter().sum::<usize>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_1: &str = r#"AAAA
BBCD
BBCC
EEEC"#;

    const DATA_2: &str = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;

    const DATA_3: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    const DATA_4: &str = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;

    #[test]
    fn part_one_example_one() {
        let day12 = Day12::new(DATA_1.to_string());
        assert_eq!(day12.part_one(), "140");
    }

    #[test]
    fn part_one_example_two() {
        let day12 = Day12::new(DATA_2.to_string());
        assert_eq!(day12.part_one(), "772");
    }

    #[test]
    fn part_one_example_three() {
        let day12 = Day12::new(DATA_3.to_string());
        assert_eq!(day12.part_one(), "1930");
    }

    #[test]
    fn part_two_example_one() {
        let day12 = Day12::new(DATA_1.to_string());
        assert_eq!(day12.part_two(), "80");
    }

    #[test]
    fn part_two_example_two() {
        let day12 = Day12::new(DATA_2.to_string());
        assert_eq!(day12.part_two(), "436");
    }

    #[test]
    fn part_two_example_three() {
        let day12 = Day12::new(DATA_4.to_string());
        assert_eq!(day12.part_two(), "236");
    }
}
