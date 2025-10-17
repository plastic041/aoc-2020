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

    let mut diffs_map: HashMap<u8, u8> = HashMap::new();

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

fn dp(map: &mut HashMap<u8, u64>, numbers: &[u8], number: u8) {
    if *numbers.last().unwrap() == number {
        map.insert(number, 1);
    } else {
        let next1 = map.get(&(number + 1)).unwrap_or(&0);
        let next2 = map.get(&(number + 2)).unwrap_or(&0);
        let next3 = map.get(&(number + 3)).unwrap_or(&0);
        map.insert(number, next1 + next2 + next3);
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut adapters = input
        .lines()
        .map(|line| line.parse::<u8>().unwrap())
        .sorted()
        .collect_vec();
    adapters.insert(0, 0);
    let last = adapters.last().unwrap();
    adapters.push(*last + 3);

    let adapters = adapters;

    let mut map: HashMap<u8, u64> = HashMap::new();

    for i in (0..adapters.len()).rev() {
        dp(&mut map, &adapters, adapters[i]);
    }

    map.get(&0).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
