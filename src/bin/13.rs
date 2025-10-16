use itertools::Itertools;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let (num_str, buses_str) = input.lines().collect_tuple().unwrap();
    let start_timestamp = num_str.parse::<u32>().unwrap();
    let buses = buses_str
        .split(",")
        .filter_map(|s| s.parse::<u32>().ok())
        .collect_vec();

    let a = buses
        .iter()
        .map(|bus| {
            let x = start_timestamp.div_ceil(*bus);
            let multiplied = x * bus;

            (bus, multiplied - start_timestamp)
        })
        .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
        .collect_vec();

    Some(a[0].0 * a[0].1)
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
        assert_eq!(result, Some(295));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
