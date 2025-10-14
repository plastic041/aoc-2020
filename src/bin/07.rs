use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1, space1},
    combinator::{map, map_res, opt},
    multi::separated_list1,
    IResult, Parser,
};

advent_of_code::solution!(7);

#[derive(Debug, Clone)]
struct Bag {
    color: String,
    contains: HashMap<String, u32>,
    contained_by: HashMap<String, u32>,
}

/// (number) (color name) bag[s]
fn bag_with_count(input: &str) -> IResult<&str, (u32, &str)> {
    let (input, (count, _, color, _, _)) = (
        map_res(digit1, |s: &str| s.parse::<u32>()),
        space1,
        take_until(" bag"),
        tag(" bag"),
        opt(char('s')),
    )
        .parse(input)?;

    Ok((input, (count, color)))
}

/// color name bags contain number color name bag[s][, number color name bag[s]].
fn bag(input: &str) -> IResult<&str, Bag> {
    let (input, color) = take_until(" bags contain ")(input)?;
    let (input, _) = tag(" bags contain ")(input)?;
    let (input, contains) = alt((
        map(tag("no other bags"), |_| Vec::new()),
        separated_list1(tag(", "), bag_with_count),
    ))
    .parse(input)?;

    let mut map = HashMap::new();
    contains.iter().for_each(|&(count, color)| {
        map.insert(color.to_string(), count);
    });

    Ok((
        input,
        Bag {
            color: color.to_owned(),
            contains: map,
            contained_by: HashMap::new(),
        },
    ))
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut bags_map: HashMap<String, Bag> = HashMap::new();

    input.lines().for_each(|line| {
        let (_, b) = bag(line).unwrap();
        bags_map.insert(b.color.clone(), b);
    });

    for bag in bags_map.clone() {
        for contained_bag in bag.1.contains {
            let b = bags_map.get_mut(&contained_bag.0).unwrap();
            b.contained_by
                .insert(bag.1.color.to_owned(), contained_bag.1);
        }
    }

    let mut set = HashSet::new();
    let mut queue = vec!["shiny gold"];
    while !queue.is_empty() {
        let next = queue.pop().unwrap();
        match bags_map.get(next) {
            Some(next_bag) => {
                next_bag.contained_by.iter().for_each(|contained_by| {
                    queue.push(contained_by.0);
                    set.insert(contained_by.0);
                });
            }
            None => (),
        }
    }

    Some(set.iter().count() as u64)
}

fn calc_recursive(map: &HashMap<String, Bag>, bag: &Bag) -> u32 {
    if bag.contains.is_empty() {
        1
    } else {
        bag.contains
            .iter()
            .map(|b| calc_recursive(map, map.get(b.0).unwrap()) * b.1)
            .sum::<u32>()
            + 1
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut bags_map: HashMap<String, Bag> = HashMap::new();

    input.lines().for_each(|line| {
        let (_, b) = bag(line).unwrap();
        bags_map.insert(b.color.clone(), b);
    });

    let sum = calc_recursive(&bags_map, bags_map.get("shiny gold").unwrap()) - 1;

    Some(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32));
    }
}
