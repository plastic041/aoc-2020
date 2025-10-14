use itertools::Itertools;
use std::io::{stdout, Write};

advent_of_code::solution!(5);

#[derive(Debug)]
enum Direction {
    Upper,
    Lower,
}

fn binary((min, max): (u8, u8), dir: &Direction) -> (u8, u8) {
    let (i, a) = match dir {
        Direction::Lower => (min, (max + min - 1) / 2),
        Direction::Upper => ((max + min + 1) / 2, max),
    };

    (i, a)
}

#[derive(Debug)]
struct BoardingPass {
    characters: [Direction; 10],
}

impl BoardingPass {
    fn find(&self) -> (u8, u8) {
        let mut min = 0;
        let mut max = 127;

        for i in 0..7 {
            let direction = &self.characters[i];
            (min, max) = binary((min, max), direction);
        }

        assert!(min == max);

        let row = min;

        let mut min = 0;
        let mut max = 7;

        for i in 7..10 {
            let direction = &self.characters[i];
            (min, max) = binary((min, max), direction);
        }

        assert!(min == max);

        let col = min;

        (row, col)
    }

    fn seat_id(&self) -> u32 {
        let (row, col) = self.find();
        row as u32 * 8 + col as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let passes = input
        .lines()
        .map(|line| BoardingPass {
            characters: line
                .chars()
                .map(|char| match char {
                    'F' => Direction::Lower,
                    'B' => Direction::Upper,
                    'L' => Direction::Lower,
                    'R' => Direction::Upper,
                    _ => panic!(),
                })
                .collect_vec()
                .try_into()
                .unwrap(),
        })
        .collect_vec();

    let counts = passes.iter().map(|pass| pass.seat_id()).collect_vec();

    counts.iter().max().copied()
}

#[derive(Copy, Clone)]
enum Seat {
    Empty,
    NotEmpty,
}

pub fn part_two(input: &str) -> Option<u64> {
    let passes = input
        .lines()
        .map(|line| BoardingPass {
            characters: line
                .chars()
                .map(|char| match char {
                    'F' => Direction::Lower,
                    'B' => Direction::Upper,
                    'L' => Direction::Lower,
                    'R' => Direction::Upper,
                    _ => panic!(),
                })
                .collect_vec()
                .try_into()
                .unwrap(),
        })
        .collect_vec();

    let seats = passes.iter().map(|pass| pass.find()).collect_vec();

    let mut grid: [[Seat; 128]; 8] = [[Seat::Empty; 128]; 8];

    seats.iter().for_each(|&(row, col)| {
        grid[col as usize][row as usize] = Seat::NotEmpty;
    });
    let mut lock = stdout().lock();
    grid.iter().for_each(|row| {
        row.iter().for_each(|seat| {
            write!(
                lock,
                "{}",
                match seat {
                    Seat::Empty => '.',
                    Seat::NotEmpty => '#',
                }
            )
            .unwrap();
        });
        write!(lock, "\n",).unwrap();
    });

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(820));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
