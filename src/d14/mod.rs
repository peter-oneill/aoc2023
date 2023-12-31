use core::panic;
use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc, str::Lines};

use itertools::Itertools;

use crate::Solver;
pub struct Solver14;

#[derive(PartialEq, Debug, PartialOrd, Ord, Eq, Copy, Clone, Hash)]
enum Rock {
    Square = 0,
    Round = 1,
    None = 2,
}

impl Rock {
    fn to_char(self) -> char {
        match self {
            Rock::Square => '#',
            Rock::Round => 'O',
            Rock::None => '.',
        }
    }
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

type Location = Rc<RefCell<Rock>>;

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
                let location = Rc::new(RefCell::new(match c {
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
                let location = Rc::new(RefCell::new(match c {
                    '.' => Rock::None,
                    '#' => Rock::Square,
                    'O' => Rock::Round,
                    _ => panic!("Invalid input"),
                }));

                rows_of_rocks[y].push(location.clone());
                cols_of_rocks[x].push(location);
            }
        }

        let mut map_history_hashmap: HashMap<String, u32> = HashMap::new();
        let mut map_history_vec: Vec<String> = vec![];

        for ii in 0..1000000000 {
            spin(&mut rows_of_rocks, &mut cols_of_rocks);

            let str_representation = rows_to_string(&rows_of_rocks);
            let start_of_loop = map_history_hashmap.insert(str_representation.clone(), ii);
            map_history_vec.push(str_representation);

            if let Some(start_of_loop_ix) = start_of_loop {
                let remaining = (1000000000 - ii - 1) % (ii - start_of_loop_ix);
                let end_version_ix = start_of_loop_ix + remaining;
                let previously_found_string = &map_history_vec[end_version_ix as usize];
                let previously_found_map = previously_found_string
                    .chars()
                    .chunks(rows_of_rocks[0].len())
                    .into_iter()
                    .map(|row| {
                        row.map(|c| {
                            Rc::new(RefCell::new(match c {
                                '.' => Rock::None,
                                '#' => Rock::Square,
                                'O' => Rock::Round,
                                _ => panic!("Invalid input"),
                            }))
                        })
                        .collect::<Vec<Rc<RefCell<Rock>>>>()
                    })
                    .collect::<Vec<Vec<Rc<RefCell<Rock>>>>>();
                return calc_north_weight(&previously_found_map).to_string();
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

fn rows_to_string(rows: &[Vec<Location>]) -> String {
    rows.iter()
        .flat_map(|row| row.iter().map(|l| l.borrow().to_char()))
        .collect::<String>()
}

fn calc_north_weight(rows: &[Vec<Location>]) -> u32 {
    let num_rows = rows.len();
    rows.iter()
        .enumerate()
        .map(|(row_ix, row)| {
            row.iter().filter(|l| *l.borrow() == Rock::Round).count() as u32
                * (num_rows - row_ix) as u32
        })
        .sum()
}

fn sort_rocks(
    rocks: &Vec<Rc<RefCell<Rock>>>,
    roll_back_to: usize,
    first_rock: (Rock, usize),
    second_rock: (Rock, usize),
) {
    update_rocks(rocks, roll_back_to, first_rock.1, first_rock.0);
    update_rocks(
        rocks,
        roll_back_to + first_rock.1,
        second_rock.1,
        second_rock.0,
    );
}

fn update_rocks(rocks: &Vec<Rc<RefCell<Rock>>>, start_ix: usize, num: usize, rock_type: Rock) {
    let start_ix = start_ix;
    let finish_ix = start_ix + num;
    for rock in &rocks[start_ix..finish_ix] {
        *(**rock).borrow_mut() = rock_type;
    }
}

fn roll_balls(rows: &mut Vec<Vec<Location>>, cols: &mut Vec<Vec<Location>>, dir: Dir) {
    let (working_map, reverse) = match dir {
        Dir::North => (&cols, false),
        Dir::East => (&rows, true),
        Dir::South => (&cols, true),
        Dir::West => (&rows, false),
    };

    for line in working_map.iter() {
        let mut last_rock_square_or_start = true;
        let mut rounds_since_last_square: usize = 0;
        let mut nones_since_last_square: usize = 0;
        let mut location_to_roll_back_to: usize = 0;

        for loc_ix in 0..line.len() {
            match *line[loc_ix].borrow() {
                Rock::Square => {
                    if !last_rock_square_or_start {
                        // this is a square, but the last wasn't.  We need to roll the balls
                        if reverse {
                            sort_rocks(
                                line,
                                location_to_roll_back_to,
                                (Rock::None, nones_since_last_square),
                                (Rock::Round, rounds_since_last_square),
                            );
                        } else {
                            sort_rocks(
                                line,
                                location_to_roll_back_to,
                                (Rock::Round, rounds_since_last_square),
                                (Rock::None, nones_since_last_square),
                            );
                        }
                    };

                    rounds_since_last_square = 0;
                    nones_since_last_square = 0;
                    last_rock_square_or_start = true;
                    location_to_roll_back_to = loc_ix + 1;
                }
                Rock::Round => {
                    rounds_since_last_square += 1;
                    last_rock_square_or_start = false;
                }
                Rock::None => {
                    nones_since_last_square += 1;
                    last_rock_square_or_start = false;
                }
            }
        }
        if !last_rock_square_or_start {
            // finished the row, and have rocks to sort
            if reverse {
                sort_rocks(
                    line,
                    location_to_roll_back_to,
                    (Rock::None, nones_since_last_square),
                    (Rock::Round, rounds_since_last_square),
                );
            } else {
                sort_rocks(
                    line,
                    location_to_roll_back_to,
                    (Rock::Round, rounds_since_last_square),
                    (Rock::None, nones_since_last_square),
                );
            }
        };
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
