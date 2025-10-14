use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Debug)]
struct Group<'a> {
    raw: &'a str,
    size: u8,
    // answers: [bool; 26]
}

impl Group<'_> {
    fn get_answers1(&self) -> [bool; 26] {
        let mut answers = [false; 26];
        self.raw.lines().for_each(|line| {
            line.chars().for_each(|char| {
                let ascii = u32::from(char) - 97;
                answers[ascii as usize] = true;
            });
        });

        answers
    }

    fn get_answers2(&self) -> u8 {
        let mut answers = [0; 26];
        self.raw.lines().for_each(|line| {
            line.chars().for_each(|char| {
                let ascii = u32::from(char) - 97;
                answers[ascii as usize] += 1;
            });
        });

        answers
            .iter()
            .filter(|answer| **answer == self.size)
            .count()
            .try_into()
            .unwrap()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let groups = input
        .split("\n\n")
        .map(|line| Group {
            raw: line,
            size: line.lines().count() as u8,
        })
        .collect_vec();

    let answers = groups
        .iter()
        .map(|group| group.get_answers1())
        .collect_vec();

    let sum = answers
        .into_iter()
        .map(|answer| answer.iter().filter(|b| **b == true).count() as u64)
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let groups = input
        .split("\n\n")
        .map(|line| Group {
            raw: line,
            size: line.lines().count() as u8,
        })
        .collect_vec();

    let answers = groups
        .iter()
        .map(|group| group.get_answers2())
        .collect_vec();

    let sum = answers.iter().map(|a| *a as u64).sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
