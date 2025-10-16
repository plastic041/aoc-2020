use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let mut adapters = input
        .lines()
        .map(|line| line.parse::<u8>().unwrap())
        .sorted()
        .collect_vec();
    adapters.insert(0, 0);
    let last = adapters.last().unwrap();
    adapters.push(*last + 3);

    let adapters = adapters;

    let mut diffs_map: HashMap<u8, u32> = HashMap::new();

    for i in 0..adapters.len() - 1 {
        let a = adapters[i];
        let b = adapters[i + 1];
        let diff = b - a;
        let diff_count = diffs_map.get(&diff).unwrap_or(&0);
        diffs_map.insert(diff, *diff_count + 1);
    }

    let zeros = *diffs_map.get(&1).unwrap() as u32;
    let threes = *diffs_map.get(&3).unwrap() as u32;

    Some(zeros * threes)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(220));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
