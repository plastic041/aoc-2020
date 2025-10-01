use std::ops::Index;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::{
        anychar,
        complete::{alpha1, digit1},
    },
    combinator::map_res,
    IResult, Parser,
};

advent_of_code::solution!(2);

#[derive(Debug)]
struct Policy {
    pattern: char,
    min: usize,
    max: usize,
}

#[derive(Debug)]
struct Pair {
    password: String,
    policy: Policy,
}

impl Pair {
    fn is_password_valid1(&self) -> bool {
        let match_count = self
            .password
            .chars()
            .filter(|char| *char == self.policy.pattern)
            .count();

        match_count <= self.policy.max && match_count >= self.policy.min
    }

    fn is_password_valid2(&self) -> bool {
        let chars = self.password.chars().collect_vec();
        let first_match = *chars.index(self.policy.min - 1) == self.policy.pattern;
        let second_match = *chars.index(self.policy.max - 1) == self.policy.pattern;

        if first_match && second_match {
            false
        } else if !first_match && !second_match {
            false
        } else {
            true
        }

        // !(!first_match && !second_match)
    }
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse).parse(input)
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    let (input, min) = parse_usize(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, max) = parse_usize(input)?;

    let (input, _) = tag(" ")(input)?;

    let (input, pattern) = anychar(input)?;
    let (input, _) = tag(": ")(input)?;

    let (input, password) = alpha1(input)?;

    Ok((
        input,
        Pair {
            password: password.to_string(),
            policy: Policy { max, min, pattern },
        },
    ))
}

pub fn part_one(input: &str) -> Option<u64> {
    let pairs = input.lines().map(|line| {
        parse_pair(line)
            .unwrap_or_else(|_| panic!("can't parse: {}", line))
            .1
    });

    let valid_count = pairs.filter(|pair| pair.is_password_valid1()).count();

    Some(valid_count.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let pairs = input.lines().map(|line| {
        parse_pair(line)
            .unwrap_or_else(|_| panic!("can't parse: {}", line))
            .1
    });

    let valid_count = pairs.filter(|pair| pair.is_password_valid2()).count();

    Some(valid_count.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
