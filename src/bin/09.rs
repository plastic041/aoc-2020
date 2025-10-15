use itertools::Itertools;

advent_of_code::solution!(9);

fn check_sum(numbers: &[u64], target: u64) -> bool {
    numbers
        .iter()
        .combinations(2)
        .any(|nums| nums.iter().map(|&a| a).sum::<u64>() == target)
}

pub fn part_one(input: &str) -> Option<u64> {
    let numbers = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect_vec();

    let preamble_size = 25;

    for i in 0..numbers.len() - preamble_size {
        let nums = &numbers[i..i + preamble_size];
        if !check_sum(nums, numbers[i + preamble_size]) {
            return Some(numbers[i + preamble_size]);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let number = 177777905;
    // let number = 127;
    let numbers = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect_vec();

    let mut first = 0;
    let mut last = 0;

    loop {
        let nums = &numbers[first..=last];
        let sum = nums.iter().sum::<u64>();

        match sum.cmp(&number) {
            std::cmp::Ordering::Less => last += 1,
            std::cmp::Ordering::Equal => {
                let min = nums.iter().min().unwrap();
                let max = nums.iter().max().unwrap();
                // let first = numbers[from];
                // let last = numbers[end];

                return Some(min + max);
            }
            std::cmp::Ordering::Greater => first += 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(127));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }
}
