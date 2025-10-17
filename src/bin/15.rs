use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u64> {
    let numbers = input
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    let mut map: HashMap<u64, usize> = HashMap::new();
    numbers.iter().enumerate().for_each(|(index, number)| {
        map.insert(*number, index);
    });

    let mut last: u64 = *numbers.last().unwrap();

    println!("{:?}, {}", map, last);

    for i in (0 + numbers.len())..10 {
        let cache = map.get(&last);
        match cache {
            Some(last_index) => {
                let index_diff = (i - last_index) as u64;
                map.insert(last, i);
                last = index_diff;
            }
            None => {
                last = 0;
            }
        }
        println!("{} {}", last, i);
    }

    Some(last)
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
        assert_eq!(result, Some(436));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
