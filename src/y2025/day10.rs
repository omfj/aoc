use crate::AdventDay;
use std::collections::{HashSet, VecDeque};
use z3::ast::{Ast, Int};
use z3::{Config, Context, Optimize, SatResult};

pub struct Day10 {
    input: String,
}

type Machine = (Vec<char>, Vec<Vec<usize>>, Vec<usize>);

// 0: indicator light diagram - lisf of # and ., where # is on and . is off
// 1: wiring schemantic - list of positions to toggle
// 2: joltage requirements
fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let pattern: Vec<char> = parts[0].trim_matches(&['[', ']'][..]).chars().collect();
            let sequences: Vec<Vec<usize>> = parts[1..parts.len() - 1]
                .iter()
                .map(|s| {
                    s.trim_matches(&['(', ')'][..])
                        .split(',')
                        .filter_map(|n| n.parse::<usize>().ok())
                        .collect()
                })
                .collect();
            let counts: Vec<usize> = parts[parts.len() - 1]
                .trim_matches(&['{', '}'][..])
                .split(',')
                .filter_map(|n| n.parse::<usize>().ok())
                .collect();
            (pattern, sequences, counts)
        })
        .collect()
}

impl AdventDay for Day10 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let mut sum = 0;

        let machines = parse_input(&self.input);

        for (target_state, buttons, _) in machines.iter() {
            let mut min_presses = usize::MAX;
            let n = target_state.len();

            let initial_state = vec![false; n];
            let target_state: Vec<bool> = target_state.iter().map(|&c| c == '#').collect();

            let mut q = VecDeque::new();
            let mut seen = HashSet::new();

            q.push_back((initial_state.clone(), 0));
            seen.insert(initial_state);

            while let Some((prev_state, presses)) = q.pop_front() {
                for button in buttons {
                    let mut curr_state = prev_state.clone();

                    // toggle buttons
                    for &pos in button.iter() {
                        curr_state[pos] = !curr_state[pos];
                    }

                    // check if we've seen this state before
                    if seen.contains(&curr_state) {
                        continue;
                    }
                    seen.insert(curr_state.clone());

                    // check if we've reached the target state
                    if curr_state == target_state {
                        min_presses = min_presses.min(presses + 1);
                        break;
                    } else {
                        // add the new state to the queue
                        q.push_back((curr_state, presses + 1));
                    }
                }

                if min_presses != usize::MAX {
                    break;
                }
            }

            if min_presses != usize::MAX {
                sum += min_presses;
            }
        }

        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0;

        let machines = parse_input(&self.input);

        for (_, buttons, target_constraints) in machines.iter() {
            let cfg = Config::new();
            let ctx = Context::new(&cfg);
            let opt = Optimize::new(&ctx);

            let button_vars: Vec<Int> = (0..buttons.len())
                .map(|i| Int::new_const(&ctx, format!("button_{}", i)))
                .collect();

            for var in button_vars.iter() {
                opt.assert(&var.ge(&Int::from_i64(&ctx, 0)));
            }

            // for each joltage constraint, create sum of button presses that affect that light
            for (light_ix, &target) in target_constraints.iter().enumerate() {
                let mut sum_terms: Vec<&Int> = Vec::new();
                for (button, var) in buttons.iter().zip(&button_vars) {
                    if button.contains(&light_ix) {
                        sum_terms.push(var);
                    }
                }

                if !sum_terms.is_empty() {
                    let sum_expr = Int::add(&ctx, &sum_terms);
                    opt.assert(&sum_expr._eq(&Int::from_i64(&ctx, target as i64)));
                }
            }

            // minimize for total button presses
            let button_refs: Vec<&Int> = button_vars.iter().collect();
            let total_presses = Int::add(&ctx, &button_refs);
            opt.minimize(&total_presses);

            // solve
            if opt.check(&[]) == SatResult::Sat {
                if let Some(model) = opt.get_model() {
                    if let Some(result) = model.eval(&total_presses, true) {
                        if let Some(value) = result.as_i64() {
                            sum += value as usize;
                        }
                    }
                }
            }
        }

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

    #[test]
    fn part_one() {
        let day10 = Day10::new(DATA.to_string());
        assert_eq!(day10.part_one(), "7");
    }

    #[test]
    fn part_two() {
        let day10 = Day10::new(DATA.to_string());
        assert_eq!(day10.part_two(), "33");
    }
}
