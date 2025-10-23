use std::collections::VecDeque;

use itertools::Itertools;

advent_of_code::solution!(22);

#[derive(Debug, Clone)]
struct Player {
    deck: VecDeque<u32>,
}

fn parse_player(input: &str) -> Player {
    let mut lines = input.lines();
    lines.next();
    Player {
        deck: lines.rev().map(|line| line.parse().unwrap()).collect(),
    }
}

fn round(p1: &mut Player, p2: &mut Player) {
    let p1_top = p1.deck.pop_back().unwrap();
    let p2_top = p2.deck.pop_back().unwrap();
    match p1_top.cmp(&p2_top) {
        std::cmp::Ordering::Less => {
            p2.deck.push_front(p2_top);
            p2.deck.push_front(p1_top);
        }
        std::cmp::Ordering::Equal => panic!(),
        std::cmp::Ordering::Greater => {
            p1.deck.push_front(p1_top);
            p1.deck.push_front(p2_top);
        }
    }
}

impl Player {
    fn score(&self) -> u32 {
        self.deck.iter().enumerate().fold(0, |acc, (index, card)| {
            println!("{} {}", card, index + 1);
            return acc + (index + 1) as u32 * card;
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut p1, mut p2) = input
        .split("\n\n")
        .map(|player_str| parse_player(player_str))
        .collect_tuple()
        .unwrap();

    loop {
        round(&mut p1, &mut p2);

        if p1.deck.len() == 0 {
            return Some(p2.score());
        }

        if p2.deck.len() == 0 {
            return Some(p1.score());
        }
    }

    None
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
        assert_eq!(result, Some(306));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
