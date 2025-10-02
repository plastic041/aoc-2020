advent_of_code::solution!(3);

enum Cell {
    Square,
    Tree,
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '#' => Cell::Tree,
                    '.' => Cell::Square,
                    _ => panic!("{char}"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    const RIGHT: usize = 3;
    const DOWN: usize = 1;

    let mut x = 0;
    let mut y = 0;

    let mut tree_count = 0;

    loop {
        x += RIGHT;
        y += DOWN;

        if y == height {
            break;
        }

        x = x.rem_euclid(width);

        let cell = &grid[y][x];
        match cell {
            Cell::Square => (),
            Cell::Tree => tree_count += 1,
        }
    }

    Some(tree_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '#' => Cell::Tree,
                    '.' => Cell::Square,
                    _ => panic!("{char}"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    let mut tree_counts: Vec<u64> = vec![];

    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .for_each(|(right, down)| {
            let mut tree_count = 0;
            let mut x: usize = 0;
            let mut y: usize = 0;

            loop {
                x += right;
                y += down;

                if y >= height {
                    break;
                }

                x = x.rem_euclid(width);

                let cell = &grid[y][x];
                match cell {
                    Cell::Square => (),
                    Cell::Tree => tree_count += 1,
                }
            }

            tree_counts.push(tree_count);
        });

    let product = tree_counts.iter().copied().reduce(|a, b| (a * b)).unwrap();
    Some(product)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(336));
    }
}
