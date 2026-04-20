use crate::AdventDay;

type Shape = [[bool; 3]; 3];

fn count_cells(shape: &Shape) -> usize {
    shape
        .iter()
        .flatten()
        .filter(|&&cell| cell)
        .count()
}

fn rotate_90(tile: &Shape) -> Shape {
    let mut result = [[false; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[j][2 - i] = tile[i][j];
        }
    }
    result
}

fn get_transformations(tile: &Shape) -> Vec<Shape> {
    let mut variants = Vec::new();
    let mut current = *tile;

    for _ in 0..4 {
        variants.push(current);
        current = rotate_90(&current);
    }

    // Flip and rotate
    current = current.map(|row| [row[2], row[1], row[0]]);
    for _ in 0..4 {
        variants.push(current);
        current = rotate_90(&current);
    }

    variants.sort_unstable();
    variants.dedup();
    variants
}

fn can_place(grid: &[Vec<bool>], shape: &Shape, x: usize, y: usize) -> bool {
    shape.iter().enumerate().all(|(i, row)| {
        row.iter().enumerate().all(|(j, &cell)| {
            !cell
                || (x + j < grid[0].len()
                    && y + i < grid.len()
                    && !grid[y + i][x + j])
        })
    })
}

fn place(grid: &mut [Vec<bool>], shape: &Shape, x: usize, y: usize, val: bool) {
    for (i, row) in shape.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell {
                grid[y + i][x + j] = val;
            }
        }
    }
}

fn try_fit(
    grid: &mut [Vec<bool>],
    transforms: &[Vec<Shape>],
    counts: &[(usize, usize)],
    idx: usize,
) -> bool {
    if idx >= counts.len() || counts[idx].1 == 0 {
        return idx >= counts.len() || try_fit(grid, transforms, counts, idx + 1);
    }

    for variant in &transforms[counts[idx].0] {
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if can_place(grid, variant, x, y) {
                    place(grid, variant, x, y, true);
                    let mut next = counts.to_vec();
                    next[idx].1 -= 1;
                    if try_fit(grid, transforms, &next, idx) {
                        place(grid, variant, x, y, false);
                        return true;
                    }
                    place(grid, variant, x, y, false);
                }
            }
        }
    }
    false
}

fn parse_input(s: &str) -> (Vec<Shape>, Vec<((usize, usize), Vec<usize>)>) {
    let sections: Vec<&str> = s.split("\n\n").collect();

    let shapes = sections[..sections.len() - 1]
        .iter()
        .map(|sec| {
            let mut tile = [[false; 3]; 3];
            for (i, line) in sec.lines().skip(1).enumerate() {
                for (j, ch) in line.chars().enumerate() {
                    tile[i][j] = ch == '#';
                }
            }
            tile
        })
        .collect();

    let regions = sections[sections.len() - 1]
        .lines()
        .map(|line| {
            let (size, nums) = line.split_once(':').unwrap();
            let (w, h) = size.trim().split_once('x').unwrap();
            let counts = nums
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            ((w.parse().unwrap(), h.parse().unwrap()), counts)
        })
        .collect();

    (shapes, regions)
}

pub struct Day12 {
    input: String,
}

impl AdventDay for Day12 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let (shapes, regions) = parse_input(&self.input);
        let sizes: Vec<_> = shapes.iter().map(count_cells).collect();
        let transforms: Vec<_> = shapes.iter().map(get_transformations).collect();

        regions
            .iter()
            .filter(|((w, h), counts)| {
                let area = w * h;
                let total: usize = counts.iter().sum();
                let cells: usize = counts
                    .iter()
                    .enumerate()
                    .map(|(i, &c)| sizes[i] * c)
                    .sum();

                area >= 9 * total
                    || (area >= cells && {
                        let mut grid = vec![vec![false; *w]; *h];
                        let shaped: Vec<_> = counts
                            .iter()
                            .enumerate()
                            .map(|(i, &c)| (i, c))
                            .collect();
                        try_fit(&mut grid, &transforms, &shaped, 0)
                    })
            })
            .count()
            .to_string()
    }

    fn part_two(&self) -> String {
        "does not exist".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#;

    #[test]
    fn part_one() {
        let day12 = Day12::new(DATA.to_string());
        assert_eq!(day12.part_one(), "2");
    }
}
