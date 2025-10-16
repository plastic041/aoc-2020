use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(11);

#[derive(Debug, Clone, PartialEq)]
enum SeatState {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    fn neighbors(&self) -> Vec<Self> {
        let ix = self.x as i8;
        let iy = self.y as i8;

        vec![
            (ix - 1, iy - 1),
            (ix, iy - 1),
            (ix + 1, iy - 1),
            (ix - 1, iy),
            (ix + 1, iy),
            (ix - 1, iy + 1),
            (ix, iy + 1),
            (ix + 1, iy + 1),
        ]
        .iter()
        .map(|&(x, y)| Position::new(x as usize, y as usize))
        .collect_vec()
    }
}

#[derive(Debug, Clone)]
struct Grid {
    seats: HashMap<Position, SeatState>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut map: HashMap<Position, SeatState> = HashMap::new();

        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, char)| {
                match char {
                    '.' => map.insert(Position::new(x, y), SeatState::Floor),
                    'L' => map.insert(Position::new(x, y), SeatState::Empty),
                    '#' => map.insert(Position::new(x, y), SeatState::Occupied),
                    _ => panic!(),
                };
            });
        });

        Grid { seats: map }
    }

    fn get(&self, position: &Position) -> Option<SeatState> {
        self.seats.get(position).cloned()
    }

    fn neighbors(&self, position: &Position) -> Vec<SeatState> {
        position
            .neighbors()
            .iter()
            .filter_map(|pos| self.get(pos))
            .collect_vec()
    }

    fn rule(&self, current: &SeatState, neighbors: &[SeatState]) -> SeatState {
        match current {
            SeatState::Floor => SeatState::Floor,
            SeatState::Empty => {
                if neighbors
                    .iter()
                    .all(|neighbor| *neighbor != SeatState::Occupied)
                {
                    SeatState::Occupied
                } else {
                    SeatState::Empty
                }
            }
            SeatState::Occupied => {
                if neighbors
                    .iter()
                    .filter(|neighbor| **neighbor == SeatState::Occupied)
                    .count()
                    >= 4
                {
                    SeatState::Empty
                } else {
                    SeatState::Occupied
                }
            }
        }
    }

    fn next(&self) -> HashMap<Position, SeatState> {
        let mut seats_cloned = self.seats.clone();
        self.seats.iter().for_each(|(pos, seat)| {
            let neighbors = self.neighbors(pos);
            seats_cloned.insert(pos.clone(), self.rule(seat, &neighbors));
        });
        seats_cloned
    }

    fn count_by(&self, seat_state: &SeatState) -> u32 {
        self.seats
            .iter()
            .filter(|(_, seat)| *seat == seat_state)
            .count() as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::new(input);

    loop {
        let next_seats = grid.next();
        if next_seats == grid.seats {
            return Some(grid.count_by(&SeatState::Occupied));
        } else {
            grid.seats = next_seats
        }
    }
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
        assert_eq!(result, Some(37));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
