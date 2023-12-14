use core::panic;
use std::{cell::RefCell, collections::HashMap, fmt::Display, str::Lines, sync::Arc};

use itertools::Itertools;

use crate::Solver;
pub struct Solver14;

#[derive(PartialEq, Debug, PartialOrd, Ord, Eq, Copy, Clone, Hash)]
enum Rock {
    Square = 0,
    Round = 1,
    None = 2,
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rock::Square => write!(f, "#"),
            Rock::Round => write!(f, "O"),
            Rock::None => write!(f, "."),
        }
    }
}

type Location = Arc<RefCell<Rock>>;

enum Dir {
    North,
    East,
    South,
    West,
}

impl Solver for Solver14 {
    fn day_number(&self) -> u32 {
        14
    }

    fn part1(&self, input_lines: Lines) -> String {
        let mut rows_of_rocks: Vec<Vec<Location>> = vec![];
        let mut cols_of_rocks: Vec<Vec<Location>> = vec![];

        for (y, line) in input_lines.enumerate() {
            if rows_of_rocks.get(y).is_none() {
                rows_of_rocks.push(vec![]);
            }
            for (x, c) in line.chars().enumerate() {
                if cols_of_rocks.get(x).is_none() {
                    cols_of_rocks.push(vec![]);
                }
                let location = Arc::new(RefCell::new(match c {
                    '.' => Rock::None,
                    '#' => Rock::Square,
                    'O' => Rock::Round,
                    _ => panic!("Invalid input"),
                }));

                rows_of_rocks[y].push(location.clone());
                cols_of_rocks[x].push(location);
            }
        }

        roll_balls(&mut rows_of_rocks, &mut cols_of_rocks, Dir::North);

        calc_north_weight(&rows_of_rocks).to_string()
    }

    fn part2(&self, input_lines: Lines) -> String {
        let mut rows_of_rocks: Vec<Vec<Location>> = vec![];
        let mut cols_of_rocks: Vec<Vec<Location>> = vec![];

        for (y, line) in input_lines.enumerate() {
            if rows_of_rocks.get(y).is_none() {
                rows_of_rocks.push(vec![]);
            }
            for (x, c) in line.chars().enumerate() {
                if cols_of_rocks.get(x).is_none() {
                    cols_of_rocks.push(vec![]);
                }
                let location = Arc::new(RefCell::new(match c {
                    '.' => Rock::None,
                    '#' => Rock::Square,
                    'O' => Rock::Round,
                    _ => panic!("Invalid input"),
                }));

                rows_of_rocks[y].push(location.clone());
                cols_of_rocks[x].push(location);
            }
        }

        let mut map_history: HashMap<String, u32> = HashMap::new();

        for ii in 0..1000000000 {
            spin(&mut rows_of_rocks, &mut cols_of_rocks);

            let start_of_loop = map_history.insert(rows_to_string(&rows_of_rocks), ii);

            if let Some(start_of_loop_ix) = start_of_loop {
                let remaining = (1000000000 - ii - 1) % (ii - start_of_loop_ix);
                for _ in 0..remaining {
                    spin(&mut rows_of_rocks, &mut cols_of_rocks);
                }
                break;
            }
        }

        calc_north_weight(&rows_of_rocks).to_string()
    }
}

fn spin(rows_of_rocks: &mut Vec<Vec<Location>>, cols_of_rocks: &mut Vec<Vec<Location>>) {
    roll_balls(rows_of_rocks, cols_of_rocks, Dir::North);
    roll_balls(rows_of_rocks, cols_of_rocks, Dir::West);
    roll_balls(rows_of_rocks, cols_of_rocks, Dir::South);
    roll_balls(rows_of_rocks, cols_of_rocks, Dir::East);
}

fn rows_to_string(rows: &Vec<Vec<Location>>) -> String {
    rows.iter()
        .map(|row| {
            row.iter()
                .map(|l| l.borrow().to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn calc_north_weight(rows: &Vec<Vec<Location>>) -> u32 {
    let num_rows = rows.len();
    rows.iter()
        .enumerate()
        .map(|(row_ix, row)| {
            row.iter().filter(|l| *l.borrow() == Rock::Round).count() as u32
                * (num_rows - row_ix) as u32
        })
        .sum()
}

fn roll_balls(rows: &mut Vec<Vec<Location>>, cols: &mut Vec<Vec<Location>>, dir: Dir) {
    let (working_map, reverse) = match dir {
        Dir::North => (&cols, false),
        Dir::East => (&rows, true),
        Dir::South => (&cols, true),
        Dir::West => (&rows, false),
    };

    for line in working_map.iter() {
        let new_line: Vec<Rock> = line
            .iter()
            .map(|l| *l.borrow())
            .group_by(|r| *r == Rock::Square)
            .into_iter()
            .map(|(_, g)| {
                let mut g = g.collect::<Vec<Rock>>();
                g.sort();
                if reverse {
                    g.reverse();
                }
                g
            })
            .flatten()
            .collect();

        for (loc_ix, rock) in new_line.iter().enumerate() {
            *line[loc_ix].borrow_mut() = *rock;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
        let sample_input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(super::Solver14.part1(sample_input.lines()), "136");
    }

    #[test]
    fn part2() {
        let sample_input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(super::Solver14.part2(sample_input.lines()), "64");
    }
}
