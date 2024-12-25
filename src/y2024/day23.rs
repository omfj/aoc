use crate::utils::AdventDay;
use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let a = parts.next().unwrap().to_string();
            let b = parts.next().unwrap().to_string();
            (a, b)
        })
        .collect()
}

fn collect_adjecency_list(connections: &[(String, String)]) -> HashMap<String, HashSet<String>> {
    let mut adj_list: HashMap<String, HashSet<String>> = HashMap::new();
    for (a, b) in connections {
        adj_list.entry(a.clone()).or_default().insert(b.clone());
        adj_list.entry(b.clone()).or_default().insert(a.clone());
    }
    adj_list
}

fn collect_sets_of(connections: &[(String, String)], size: usize) -> Vec<Vec<String>> {
    if size == 0 {
        return Vec::new();
    }

    if size == 1 {
        let nodes: HashSet<String> = connections
            .iter()
            .flat_map(|(a, b)| vec![a.clone(), b.clone()])
            .collect();
        return nodes.into_iter().map(|node| vec![node]).collect();
    }

    let adj_list = collect_adjecency_list(connections);
    let nodes: Vec<String> = adj_list.keys().cloned().collect();
    let mut result = Vec::new();

    fn backtrack(
        current_clique: &mut Vec<String>,
        candidates: &[String],
        adj_list: &HashMap<String, HashSet<String>>,
        size: usize,
        result: &mut Vec<Vec<String>>,
    ) {
        if current_clique.len() == size {
            result.push(current_clique.clone());
            return;
        }

        for (i, node) in candidates.iter().enumerate() {
            let is_connected = current_clique.iter().all(|existing_node| {
                adj_list
                    .get(existing_node)
                    .map_or(false, |neighbors| neighbors.contains(node))
            });

            if is_connected {
                current_clique.push(node.clone());

                let new_candidates = &candidates[i + 1..];
                backtrack(current_clique, new_candidates, adj_list, size, result);

                current_clique.pop();
            }
        }
    }

    backtrack(&mut Vec::new(), &nodes, &adj_list, size, &mut result);

    result
}

fn find_largest_clique(connections: &[(String, String)]) -> Vec<String> {
    let adj_list = collect_adjecency_list(connections);
    let nodes: Vec<String> = adj_list.keys().cloned().collect();
    let mut max_clique: Vec<String> = Vec::new();
    let all_nodes: HashSet<String> = nodes.into_iter().collect();

    fn bron_kerbosch(
        r: &mut Vec<String>,
        p: &HashSet<String>,
        x: &HashSet<String>,
        adj_list: &HashMap<String, HashSet<String>>,
        max_clique: &mut Vec<String>,
    ) {
        if p.is_empty() && x.is_empty() {
            if r.len() > max_clique.len() {
                *max_clique = r.clone();
            }
            return;
        }

        let pivot = p.iter().chain(x.iter()).next().cloned();

        let pivot = match pivot {
            Some(node) => node,
            None => return,
        };

        let non_neighbors: HashSet<String> = adj_list
            .get(&pivot)
            .map(|neighbors| p.difference(neighbors).cloned().collect())
            .unwrap_or_default();

        for node in non_neighbors.clone() {
            r.push(node.clone());

            let neighbors = adj_list.get(&node).cloned().unwrap_or_else(HashSet::new);
            let new_p = p.intersection(&neighbors).cloned().collect::<HashSet<_>>();
            let new_x = x.intersection(&neighbors).cloned().collect::<HashSet<_>>();

            bron_kerbosch(r, &new_p, &new_x, adj_list, max_clique);

            r.pop();
        }
    }

    bron_kerbosch(
        &mut Vec::new(),
        &all_nodes,
        &HashSet::new(),
        &adj_list,
        &mut max_clique,
    );

    max_clique
}

pub struct Day23 {
    input: String,
}

impl AdventDay for Day23 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let connections = parse_input(&self.input);
        let sets = collect_sets_of(&connections, 3);
        sets.iter()
            .filter(|set| set.iter().any(|s| s.starts_with("t")))
            .count()
            .to_string()
    }

    fn part_two(&self) -> String {
        let connections = parse_input(&self.input);
        let mut max_clique = find_largest_clique(&connections);
        max_clique.sort();
        max_clique.join(",").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;

    #[test]
    fn part_one() {
        let day23 = Day23::new(DATA.to_string());
        assert_eq!(day23.part_one(), "7");
    }

    #[test]
    fn part_two() {
        let day23 = Day23::new(DATA.to_string());
        assert_eq!(day23.part_two(), "co,de,ka,ta");
    }
}
