use nom::{character::complete::anychar, IResult};

advent_of_code::solution!(12);

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn(self, by: i32) -> Self {
        let index = match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        };

        let turned_index = (by + index).rem_euclid(4);

        match turned_index {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Ship {
    east: i32,
    north: i32,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Waypoint {
    east: i64,
    north: i64,
}

#[derive(Debug, Clone)]
struct Ship2 {
    east: i64,
    north: i64,
    waypoint: Waypoint,
}

fn parse_command(input: &str) -> IResult<&str, (char, i32)> {
    let (value, action) = anychar(input)?;
    Ok(("", (action, value.parse::<i32>().unwrap())))
}

impl Ship {
    fn command(&mut self, input: &str) {
        let (_, (action, value)) = parse_command(input).unwrap();
        match action {
            'N' => self.north += value,
            'S' => self.north -= value,
            'E' => self.east += value,
            'W' => self.east -= value,
            'L' => self.direction = self.direction.clone().turn(-value / 90),
            'R' => self.direction = self.direction.clone().turn(value / 90),
            'F' => match self.direction {
                Direction::North => self.north += value,
                Direction::South => self.north -= value,
                Direction::East => self.east += value,
                Direction::West => self.east -= value,
            },
            _ => panic!(),
        }
    }
}

impl Ship2 {
    fn command(&mut self, input: &str) {
        let (_, (action, value)) = parse_command(input).unwrap();
        match action {
            'N' => self.waypoint.north += value as i64,
            'S' => self.waypoint.north -= value as i64,
            'E' => self.waypoint.east += value as i64,
            'W' => self.waypoint.east -= value as i64,
            'L' => {
                let sin = (value as f32).sin().round() as i64;
                let cos = (value as f32).cos().round() as i64;
                let east_rotated = self.waypoint.east * cos - self.waypoint.north * sin;
                let north_rotated = self.waypoint.east * sin + self.waypoint.north * cos;
                self.waypoint.east = east_rotated;
                self.waypoint.north = north_rotated;
            }
            'R' => {
                let sin = (-value as f32).sin().round() as i64;
                let cos = (-value as f32).cos().round() as i64;
                let east_rotated = self.waypoint.east * cos - self.waypoint.north * sin;
                let north_rotated = self.waypoint.east * sin + self.waypoint.north * cos;
                self.waypoint.east = east_rotated;
                self.waypoint.north = north_rotated;
            }
            'F' => {
                for _ in 0..value {
                    self.east += self.waypoint.east;
                    self.north += self.waypoint.north;
                }
            }
            _ => panic!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut ship = Ship {
        direction: Direction::East,
        east: 0,
        north: 0,
    };
    input.lines().for_each(|line| {
        ship.command(line);
    });

    Some(ship.east.abs() + ship.north.abs())
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut ship = Ship2 {
        waypoint: Waypoint { east: 10, north: 1 },
        east: 0,
        north: 0,
    };
    input.lines().for_each(|line| {
        ship.command(line);
    });

    Some(ship.east.abs() + ship.north.abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(286));
    }
}
