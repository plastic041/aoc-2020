advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut numbers = input
        .lines()
        .map(|line| {
            line.parse()
                .unwrap_or_else(|_| panic!("not a number: {}", line))
        })
        .collect::<Vec<u32>>();

    numbers.sort_unstable();
    let numbers = numbers;

    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            let a = numbers[i];
            let b = numbers[j];
            let sum = a + b;
            if sum > 2020 {
                break;
            } else if sum == 2020 {
                return Some((a * b).try_into().unwrap());
            } else {
                continue;
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut numbers = input
        .lines()
        .map(|line| {
            line.parse()
                .unwrap_or_else(|_| panic!("not a number: {}", line))
        })
        .collect::<Vec<u32>>();

    numbers.sort_unstable();
    let numbers = numbers;

    for i in 0..numbers.len() {
        let a = numbers[i];

        for j in i + 1..numbers.len() {
            let b = numbers[j];
            if a + b > 2020 {
                break;
            }

            for k in j + 1..numbers.len() {
                let c = numbers[k];

                let sum = a + b + c;
                if sum > 2020 {
                    break;
                } else if sum == 2020 {
                    return Some((a * b * c).try_into().unwrap());
                } else {
                    continue;
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(result, Some(514579));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(result, Some(241861950));
    }
}
