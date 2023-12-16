use std::{borrow::BorrowMut, str::Lines};

use crate::Solver;
pub struct Solver16;

#[derive(Clone, Copy, Debug)]
enum Direction {
    N = 0,
    E,
    S,
    W,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        }
    }
}
#[derive(Debug)]

enum NodeType {
    Empty,
    Vertical,
    Horizontal,
    ForwardSlash,
    BackSlash,
}
#[derive(Debug)]
struct Node {
    node_type: NodeType,
    // for each input dir (N E S W), gives the output directions (up to two) and whether the original direction is lit
    output_directions: [(Vec<Direction>, bool); 4],
    lit: bool,
}

impl Node {
    fn from(c: char) -> Node {
        match c {
            '.' => Node {
                node_type: NodeType::Empty,
                output_directions: [
                    (vec![Direction::S], false),
                    (vec![Direction::W], false),
                    (vec![Direction::N], false),
                    (vec![Direction::E], false),
                ],
                lit: false,
            },
            '-' => Node {
                node_type: NodeType::Vertical,
                output_directions: [
                    (vec![Direction::E, Direction::W], false),
                    (vec![Direction::W], false),
                    (vec![Direction::E, Direction::W], false),
                    (vec![Direction::E], false),
                ],
                lit: false,
            },
            '|' => Node {
                node_type: NodeType::Horizontal,
                output_directions: [
                    (vec![Direction::S], false),
                    (vec![Direction::N, Direction::S], false),
                    (vec![Direction::N], false),
                    (vec![Direction::N, Direction::S], false),
                ],
                lit: false,
            },
            '/' => Node {
                node_type: NodeType::ForwardSlash,
                output_directions: [
                    (vec![Direction::W], false),
                    (vec![Direction::S], false),
                    (vec![Direction::E], false),
                    (vec![Direction::N], false),
                ],
                lit: false,
            },
            '\\' => Node {
                node_type: NodeType::BackSlash,
                output_directions: [
                    (vec![Direction::E], false),
                    (vec![Direction::N], false),
                    (vec![Direction::W], false),
                    (vec![Direction::S], false),
                ],
                lit: false,
            },
            _ => panic!("Invalid node type"),
        }
    }

    // Visits the node and returns a list of output directions _to check_.
    // If the output directions are already lit, they aren't returned.
    fn visit_node(&mut self, input_direction: Direction) -> Vec<Direction> {
        self.lit = true;
        let output_directions: &Vec<Direction> =
            &self.output_directions[input_direction as usize].0;

        // We've already had a beam in this direction, so don't need to check anything.  just return nothing
        if self.output_directions[input_direction as usize].1 {
            return vec![];
        };

        // We haven't had a beam in this direction. Light this and the output directions, then return output directions that weren't previously lit
        let new_output_directions: Vec<Direction> = output_directions
            .iter()
            .filter_map(|d| {
                let lit = self.output_directions[*d as usize].1;
                if !lit {
                    Some(*d)
                } else {
                    None
                }
            })
            .collect();

        for d in new_output_directions.iter() {
            self.output_directions[*d as usize].1 = true;
        }
        self.output_directions[input_direction as usize].1 = true;

        new_output_directions
    }
}
#[derive(Debug, Clone, Copy)]
struct NodeEntry {
    coords: (usize, usize),
    entry_dir: Direction,
}

impl NodeEntry {
    fn from(
        coords: (usize, usize),
        max_coords: (isize, isize),
        exit_dir: Direction,
    ) -> Option<NodeEntry> {
        let coords = (coords.0 as isize, coords.1 as isize);
        let coords: (isize, isize) = match exit_dir {
            Direction::N => (coords.0, coords.1 - 1),
            Direction::E => (coords.0 + 1, coords.1),
            Direction::S => (coords.0, coords.1 + 1),
            Direction::W => (coords.0 - 1, coords.1),
        };
        if coords.0 < 0 || coords.0 > max_coords.0 || coords.1 < 0 || coords.1 > max_coords.1 {
            return None;
        }

        Some(NodeEntry {
            coords: (coords.0 as usize, coords.1 as usize),
            entry_dir: exit_dir.opposite(),
        })
    }
}

impl Solver for Solver16 {
    fn day_number(&self) -> u32 {
        16
    }

    fn part1(&self, input_lines: Lines) -> String {
        let mut mirror_grid = vec![];
        for (line_ix, line) in input_lines.enumerate() {
            mirror_grid.push(vec![]);
            for char in line.chars() {
                mirror_grid[line_ix].push(Node::from(char));
            }
        }

        let max_coords = (
            mirror_grid[0].len() as isize - 1,
            mirror_grid.len() as isize - 1,
        );

        let mut current_locs = vec![NodeEntry {
            coords: (0, 0),
            entry_dir: Direction::W,
        }];

        while !current_locs.is_empty() {
            let loc = current_locs.pop().unwrap();
            let node = &mut mirror_grid[loc.coords.1][loc.coords.0];
            let new_dirs = node.visit_node(loc.entry_dir);
            current_locs.extend(
                new_dirs
                    .iter()
                    .filter_map(|d| NodeEntry::from(loc.coords, max_coords, *d)),
            );
        }

        mirror_grid
            .iter()
            .flatten()
            .filter(|n| n.lit)
            .count()
            .to_string()
    }

    fn part2(&self, input_lines: Lines) -> String {
        let mut mirror_grid = vec![];
        for (line_ix, line) in input_lines.enumerate() {
            mirror_grid.push(vec![]);
            for char in line.chars() {
                mirror_grid[line_ix].push(Node::from(char));
            }
        }

        let max_coords = (
            mirror_grid[0].len() as isize - 1,
            mirror_grid.len() as isize - 1,
        );
        let mut start_locs = vec![];
        for y in 0..mirror_grid.len() {
            start_locs.push(NodeEntry {
                coords: (0, y),
                entry_dir: Direction::W,
            });
            start_locs.push(NodeEntry {
                coords: (mirror_grid[0].len() - 1, y),
                entry_dir: Direction::E,
            });
        }
        for x in 0..mirror_grid[0].len() {
            start_locs.push(NodeEntry {
                coords: (x, 0),
                entry_dir: Direction::N,
            });
            start_locs.push(NodeEntry {
                coords: (x, mirror_grid.len() - 1),
                entry_dir: Direction::S,
            });
        }

        start_locs
            .iter()
            .map(|start_loc| {
                let mut current_locs = vec![start_loc.clone()];

                for y in 0..mirror_grid.len() {
                    for x in 0..mirror_grid[0].len() {
                        mirror_grid[y][x].lit = false;
                        for d in 0..4 {
                            mirror_grid[y][x].output_directions[d].1 = false;
                        }
                    }
                }

                while !current_locs.is_empty() {
                    let loc = current_locs.pop().unwrap();
                    let node = &mut mirror_grid[loc.coords.1][loc.coords.0];
                    let new_dirs = node.visit_node(loc.entry_dir);
                    current_locs.extend(
                        new_dirs
                            .iter()
                            .filter_map(|d| NodeEntry::from(loc.coords, max_coords, *d)),
                    );
                }

                mirror_grid.iter().flatten().filter(|n| n.lit).count()
            })
            .max()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
        let sample_input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(super::Solver16.part1(sample_input.lines()), "46");
    }

    #[test]
    fn part2() {
        let sample_input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(super::Solver16.part2(sample_input.lines()), "51");
    }
}
