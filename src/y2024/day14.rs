use crate::utils::AdventDay;
use itertools::Itertools;
// use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

impl Robot {
    fn next(&mut self, max_width: i32, max_height: i32) {
        let (px, py) = self.p;
        let (vx, vy) = self.v;

        let nx = px + vx;
        let ny = py + vy;

        let nx = if nx < 0 {
            max_width + nx
        } else {
            nx % max_width
        };

        let ny = if ny < 0 {
            max_height + ny
        } else {
            ny % max_height
        };

        self.p = (nx, ny);
    }
}

pub struct Day14 {
    input: String,
}

impl AdventDay for Day14 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let mut robots = collect_robots(&self.input);
        let (max_width, max_height) = get_dimensions(&robots);

        for _ in 0..100 {
            for robot in robots.iter_mut() {
                robot.next(max_width, max_height);
            }
        }

        let quadrants = collect_robots_in_quadrants(&robots, max_width, max_height);
        let mut sum = 1;
        for quadrant in quadrants {
            sum *= quadrant.len();
        }

        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut robots = collect_robots(&self.input);
        let (max_width, max_height) = get_dimensions(&robots);

        let mut seconds = 0;
        'outer: loop {
            seconds += 1;

            // let positions = HashSet::new();

            for robot in robots.iter_mut() {
                robot.next(max_width, max_height);
                // positions.insert(robot.p);
            }

            // if positions.len() == robots.len() {
            //     break;
            // }

            let str = grid_str(&robots, max_width, max_height);
            for line in str.lines() {
                if line.contains("########") {
                    break 'outer;
                }
            }
        }

        seconds.to_string()
    }
}

fn get_dimensions(robots: &[Robot]) -> (i32, i32) {
    let max_height = robots.iter().map(|r| r.p.1).max().unwrap() + 1;
    let max_width = robots.iter().map(|r| r.p.0).max().unwrap() + 1;

    (max_width, max_height)
}

fn collect_robots(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");
            let p = parts
                .next()
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            let v = parts
                .next()
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();

            Robot { p, v }
        })
        .collect()
}

fn collect_robots_in_quadrants(
    robots: &[Robot],
    max_width: i32,
    max_height: i32,
) -> Vec<Vec<Robot>> {
    let mut quadrants = vec![Vec::new(); 4];

    for robot in robots {
        let (x, y) = robot.p;

        if x < max_width / 2 && y < max_height / 2 {
            quadrants[0].push(*robot);
        }

        if x > max_width / 2 && y < max_height / 2 {
            quadrants[1].push(*robot);
        }

        if x < max_width / 2 && y > max_height / 2 {
            quadrants[2].push(*robot);
        }

        if x > max_width / 2 && y > max_height / 2 {
            quadrants[3].push(*robot);
        }
    }

    quadrants
}

fn grid_str(robots: &[Robot], max_width: i32, max_height: i32) -> String {
    let mut grid = String::new();

    for y in 0..max_height {
        for x in 0..max_width {
            let is_robot = robots.iter().any(|r| r.p == (x, y));
            let char = if is_robot { '#' } else { '.' };
            grid.push(char);
        }
        grid.push('\n');
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    #[test]
    fn part_one() {
        let day14 = Day14::new(DATA.to_string());
        assert_eq!(day14.part_one(), "12");
    }
}
