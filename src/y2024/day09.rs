use crate::utils::AdventDay;
use std::collections::HashMap;

// (taken, free)
type DiskMap = Vec<(u32, u32)>;

type ExpandedDiskMap = Vec<Option<usize>>;

// Replace all empty spaces with the right-most non-empty space.
fn stabilize(disk_map: &ExpandedDiskMap) -> ExpandedDiskMap {
    let mut stabilized = disk_map.to_vec();

    for i in 0..stabilized.len() {
        if stabilized[i].is_none() {
            if let Some(to_swap) = stabilized.iter().rposition(|v| v.is_some()) {
                stabilized.swap(i, to_swap);
            }
        }
    }

    stabilized.iter().filter(|v| v.is_some()).cloned().collect()
}

fn find_leftmost_suitable_run(
    end_index: usize,
    length_needed: usize,
    map: &ExpandedDiskMap,
) -> Option<usize> {
    let mut runs = vec![];
    let mut run_start = None;
    let mut run_length = 0;

    for (idx, value) in map.iter().enumerate().take(end_index) {
        if value.is_none() {
            if run_start.is_none() {
                run_start = Some(idx);
                run_length = 1;
            } else {
                run_length += 1;
            }
        } else {
            if let Some(start) = run_start {
                runs.push((start, run_length));
            }
            run_start = None;
            run_length = 0;
        }
    }
    if let Some(start) = run_start {
        runs.push((start, run_length));
    }

    runs.iter()
        .filter(|(_, length)| *length >= length_needed)
        .min_by_key(|(start, _)| *start)
        .map(|(s, _)| *s)
}

fn rearrange_disk_map(disk_map: &ExpandedDiskMap) -> ExpandedDiskMap {
    let mut rearranged = disk_map.clone();

    let mut file_positions: HashMap<usize, (usize, usize)> = HashMap::new();
    for (i, &val) in rearranged.iter().enumerate() {
        if let Some(fid) = val {
            file_positions
                .entry(fid)
                .and_modify(|(_start, count)| *count += 1)
                .or_insert((i, 1));
        }
    }

    let mut files_vec: Vec<(usize, usize, usize)> = file_positions
        .into_iter()
        .map(|(fid, (start, count))| (fid, start, count))
        .collect();
    files_vec.sort_by_key(|x| x.0);
    files_vec.reverse();

    for &(fid, _fstart, fcount) in &files_vec {
        let file_positions: Vec<usize> = rearranged
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if v == Some(fid) { Some(i) } else { None })
            .collect();
        let current_start = *file_positions.first().unwrap();

        if let Some(target_start) = find_leftmost_suitable_run(current_start, fcount, &rearranged) {
            for &pos in &file_positions {
                rearranged[pos] = None;
            }

            for i in 0..fcount {
                rearranged[target_start + i] = Some(fid);
            }
        }
    }

    rearranged
}

fn expand_disk_map(disk_map: &DiskMap) -> ExpandedDiskMap {
    let mut expanded: Vec<Option<usize>> = Vec::new();

    for (id, (taken, free)) in disk_map.iter().enumerate() {
        for _ in 0..*taken {
            expanded.push(Some(id));
        }

        for _ in 0..*free {
            expanded.push(None);
        }
    }

    expanded
}

fn calculate_checksum(disk_map: &ExpandedDiskMap) -> usize {
    let mut checksum = 0;

    for (i, value) in disk_map.iter().enumerate() {
        if let Some(value) = value {
            checksum += i * value;
        }
    }

    checksum
}

// Parse the input data into a vector of tuples.
// The first element represents taken space, the second represents free space.
fn collect_disk_map(data: &str) -> DiskMap {
    let digits: Vec<u32> = data.chars().map(|c| c.to_digit(10).unwrap()).collect();
    digits
        .chunks(2)
        .map(|chunk| (*chunk.first().unwrap_or(&0), *chunk.get(1).unwrap_or(&0)))
        .collect()
}

pub struct Day09 {
    input: String,
}

impl AdventDay for Day09 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let disk_map = collect_disk_map(&self.input);
        let expanded_disk_map = expand_disk_map(&disk_map);
        let stabilized_disk_map = stabilize(&expanded_disk_map);
        let checksum = calculate_checksum(&stabilized_disk_map);
        checksum.to_string()
    }

    fn part_two(&self) -> String {
        let disk_map = collect_disk_map(&self.input);
        let expanded_disk_map = expand_disk_map(&disk_map);
        let rearranged_disk_map = rearrange_disk_map(&expanded_disk_map);
        let checksum = calculate_checksum(&rearranged_disk_map);
        checksum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "2333133121414131402";

    #[test]
    fn part_one() {
        let day09 = Day09::new(DATA.to_string());
        assert_eq!(day09.part_one(), "1928");
    }

    #[test]
    fn part_two() {
        let day09 = Day09::new(DATA.to_string());
        assert_eq!(day09.part_two(), "2858");
    }

    #[test]
    fn rearrange_disk_map_test() {
        let disk_map = collect_disk_map(DATA);
        let expanded_disk_map = expand_disk_map(&disk_map);
        let rearranged_disk_map = rearrange_disk_map(&expanded_disk_map);

        let actual: String = rearranged_disk_map
            .iter()
            .map(|v| match v {
                Some(v) => v.to_string(),
                None => ".".to_string(),
            })
            .collect::<Vec<String>>()
            .join("");
        let expected = "00992111777.44.333....5555.6666.....8888..";

        assert_eq!(actual, expected);
    }
}
