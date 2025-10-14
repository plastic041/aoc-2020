use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1, take_while_m_n},
    character::complete::{alpha1, char, digit1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use std::collections::HashMap;

advent_of_code::solution!(4);

fn parse_pair(input: &str) -> IResult<&str, (String, String)> {
    let (input, pair) =
        separated_pair(alpha1, char(':'), take_while1(|c: char| !c.is_whitespace()))
            .parse(input)?;

    Ok((input, (pair.0.to_string(), pair.1.to_string())))
}

type Passport = HashMap<String, String>;

fn parse_passport(input: &str) -> IResult<&str, Passport> {
    let (input, passport) = separated_list1(
        alt((map(char(' '), |_| ()), map(char('\n'), |_| ()))),
        parse_pair,
    )
    .parse(input)?;

    Ok((input, passport.into_iter().collect()))
}

fn parse_passports(input: &str) -> IResult<&str, Vec<Passport>> {
    separated_list1(tag("\n\n"), parse_passport).parse(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, passports) = parse_passports(input).unwrap();

    let count = passports
        .iter()
        .filter(|passport| {
            !["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .any(|k| !passport.keys().contains(&k.to_string()))
        })
        .count();

    Some(count.try_into().unwrap())
}

fn check_byr(input: &str) -> bool {
    let byr = input.parse::<u32>();

    match byr {
        Ok(num) => num >= 1920 && num <= 2002,
        Err(_) => false,
    }
}

fn check_iyr(input: &str) -> bool {
    let iyr = input.parse::<u32>();

    match iyr {
        Ok(num) => num >= 2010 && num <= 2020,
        Err(_) => false,
    }
}

fn check_eyr(input: &str) -> bool {
    let eyr = input.parse::<u32>();

    match eyr {
        Ok(num) => num >= 2020 && num <= 2030,
        Err(_) => false,
    }
}

fn parse_hgt(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, hgt) = digit1(input)?;
    let (input, unit) = alt((tag("cm"), tag("in"))).parse(input)?;

    Ok((input, (hgt, unit)))
}

fn check_hgt(input: &str) -> bool {
    match parse_hgt(input) {
        Ok((_, (hgt_str, unit))) => match hgt_str.parse::<u32>() {
            Ok(hgt) => match unit {
                "cm" => hgt >= 150 && hgt <= 193,
                "in" => hgt >= 59 && hgt <= 76,
                _ => false,
            },
            Err(_) => false,
        },
        Err(_) => false,
    }
}

fn parse_hcl(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("#")(input)?;
    let (input, hex) = take_while_m_n(6, 6, |c: char| c.is_digit(16)).parse(input)?;

    Ok((input, hex))
}

fn check_hcl(input: &str) -> bool {
    match parse_hcl(input) {
        Ok((_, _hex)) => true,
        Err(_) => false,
    }
}

fn check_ecl(input: &str) -> bool {
    match input {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}

fn parse_pid(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

fn check_pid(input: &str) -> bool {
    match parse_pid(input) {
        Ok((_, pid)) => pid.len() == 9,
        Err(_) => false,
    }
}

fn check_value((key, value): (&str, &str)) -> bool {
    match key {
        "byr" => check_byr(value),
        "iyr" => check_iyr(value),
        "eyr" => check_eyr(value),
        "hgt" => check_hgt(value),
        "hcl" => check_hcl(value),
        "ecl" => check_ecl(value),
        "pid" => check_pid(value),
        _ => true,
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, passports) = parse_passports(input).unwrap();

    let valid_passports = passports
        .iter()
        .filter(|passport| {
            !["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .any(|k| !passport.keys().contains(&k.to_string()))
        })
        .filter(|passport| {
            passport
                .iter()
                .enumerate()
                .all(|(_, (key, value))| check_value((key, value)))
        });

    Some(valid_passports.count().try_into().unwrap())
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(3));
    }
}
