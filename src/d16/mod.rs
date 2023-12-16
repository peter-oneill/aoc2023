use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::{HashMap, HashSet},
    str::Lines,
};

use crate::Solver;
pub struct Solver16;

#[derive(Debug)]
struct Map {
    x_size: isize,
    y_size: isize,
    grid: RefCell<Vec<Vec<Node>>>,
    starts: HashMap<NodeExit, RefCell<bool>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    N = 0,
    E,
    S,
    W,
    None,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
            Direction::None => Direction::None,
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
    // map: &'static Map,
    x: isize,
    y: isize,
    node_type: NodeType,
    dir_visited: [bool; 5],
    output_for_input: [[Direction; 2]; 4],
    lit: bool,
    tested_entry_here: bool,
}

impl Node {
    fn new(x: usize, y: usize, c: char) -> Node {
        let (node_type, output_for_input) = match c {
            '.' => (
                NodeType::Empty,
                [
                    [Direction::S, Direction::None],
                    [Direction::W, Direction::None],
                    [Direction::N, Direction::None],
                    [Direction::E, Direction::None],
                ],
            ),
            '-' => (
                NodeType::Horizontal,
                [
                    [Direction::E, Direction::W],
                    [Direction::W, Direction::None],
                    [Direction::E, Direction::W],
                    [Direction::E, Direction::None],
                ],
            ),
            '|' => (
                NodeType::Vertical,
                [
                    [Direction::S, Direction::None],
                    [Direction::N, Direction::S],
                    [Direction::N, Direction::None],
                    [Direction::N, Direction::S],
                ],
            ),
            '/' => (
                NodeType::ForwardSlash,
                [
                    [Direction::W, Direction::None],
                    [Direction::S, Direction::None],
                    [Direction::E, Direction::None],
                    [Direction::N, Direction::None],
                ],
            ),
            '\\' => (
                NodeType::BackSlash,
                [
                    [Direction::E, Direction::None],
                    [Direction::N, Direction::None],
                    [Direction::W, Direction::None],
                    [Direction::S, Direction::None],
                ],
            ),
            _ => panic!("Invalid node type"),
        };

        Node {
            // map,
            x: x as isize,
            y: y as isize,
            node_type,
            output_for_input,
            dir_visited: [false; 5],
            lit: false,
            tested_entry_here: false,
        }
    }

    fn visit(&mut self, map: &Map, from_dir: Direction) -> [Direction; 2] {
        // println!("Visiting {:?} from {:?}", self, from_dir);
        if self.dir_visited[from_dir as usize] {
            return [Direction::None, Direction::None];
        }

        self.lit = true;
        self.dir_visited[from_dir as usize] = true;
        let out_dirs = self.output_for_input[from_dir as usize];
        self.dir_visited[out_dirs[0] as usize] = true;
        self.dir_visited[out_dirs[1] as usize] = true;

        // println!("out dirs: {:?}", out_dirs);

        // if this is an entrance direction, mark the node as tested
        if (self.x == 0 && from_dir == Direction::W)
            || (self.x == map.y_size - 1 && from_dir == Direction::E)
            || (self.y == 0 && from_dir == Direction::N)
            || (self.y == map.x_size - 1 && from_dir == Direction::S)
        {
            *(*map
                .starts
                .get(
                    &NodeExit {
                        x: self.x,
                        y: self.y,
                        exit_dir: from_dir,
                    }
                    .to_node_entry(),
                )
                .unwrap())
            .borrow_mut() = true;
            // self.tested_entry_here = true;
        }
        out_dirs
    }
}

impl Map {
    fn reset(&mut self) {
        let mut grid = self.grid.borrow_mut();
        for row in grid.iter_mut() {
            for node in row.iter_mut() {
                node.lit = false;
                node.dir_visited = [false; 5];
            }
        }
    }
    // Visits the node and returns a list of output directions _to check_.
    // If the output directions are already lit, they aren't returned.
    fn visit_node(&mut self, last_loc_and_bearing: NodeExit) -> Vec<NodeExit> {
        // println!("Visited {:?}", last_loc_and_bearing);
        let node_entry = last_loc_and_bearing.to_node_entry();
        // println!("Visiting {:?}", node_entry);

        if node_entry.x < 0
            || node_entry.x >= self.x_size as isize
            || node_entry.y < 0
            || node_entry.y >= self.y_size as isize
        {
            return vec![];
        }

        let mut grid = self.grid.borrow_mut();
        let this_node = grid
            .get_mut(node_entry.y as usize)
            .unwrap()
            .get_mut(node_entry.x as usize)
            .unwrap();
        let next_dirs = this_node.visit(self, node_entry.exit_dir);

        next_dirs
            .iter()
            .filter_map(|d| {
                if d == &Direction::None {
                    None
                } else {
                    Some(NodeExit {
                        x: node_entry.x,
                        y: node_entry.y,
                        exit_dir: *d,
                    })
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct NodeExit {
    x: isize,
    y: isize,
    exit_dir: Direction,
}

impl NodeExit {
    fn to_node_entry(&self) -> NodeExit {
        let (x, y) = match self.exit_dir {
            Direction::N => (self.x, self.y - 1),
            Direction::E => (self.x + 1, self.y),
            Direction::S => (self.x, self.y + 1),
            Direction::W => (self.x - 1, self.y),
            Direction::None => panic!("Invalid direction"),
        };

        NodeExit {
            x,
            y,
            exit_dir: self.exit_dir.opposite(),
        }
    }
}

impl Solver for Solver16 {
    fn day_number(&self) -> u32 {
        16
    }

    fn part1(&self, input_lines: Lines) -> String {
        let mut map: Map = Map {
            x_size: 0,
            y_size: 0,
            grid: RefCell::new(vec![]),
            starts: HashMap::new(),
        };
        let mut mirror_grid = vec![];
        for (line_ix, line) in input_lines.enumerate() {
            mirror_grid.push(vec![]);
            for (char_ix, char) in line.chars().enumerate() {
                mirror_grid[line_ix].push(Node::new(char_ix, line_ix, char));
            }
        }

        *map.grid.borrow_mut() = mirror_grid;
        map.x_size = map.grid.borrow().len() as isize;
        map.y_size = map.grid.borrow()[0].len() as isize;
        map.starts.insert(
            NodeExit {
                x: -1,
                y: 0,
                exit_dir: Direction::E,
            },
            RefCell::new(false),
        );

        let starts_copy = map.starts.keys().map(|k| *k).collect::<Vec<NodeExit>>();

        for start in starts_copy {
            let mut current_loc_and_brearings = vec![start];

            while let Some(loc) = current_loc_and_brearings.pop() {
                current_loc_and_brearings.extend(map.visit_node(loc));
            }
        }

        let grid = map.grid.borrow();
        grid.iter().flatten().filter(|n| n.lit).count().to_string()
    }

    fn part2(&self, input_lines: Lines) -> String {
        let mut map: Map = Map {
            x_size: 0,
            y_size: 0,
            grid: RefCell::new(vec![]),
            starts: HashMap::new(),
        };
        let mut mirror_grid = vec![];
        for (line_ix, line) in input_lines.enumerate() {
            mirror_grid.push(vec![]);
            for (char_ix, char) in line.chars().enumerate() {
                mirror_grid[line_ix].push(Node::new(char_ix, line_ix, char));
            }
        }

        *map.grid.borrow_mut() = mirror_grid;
        map.x_size = map.grid.borrow().len() as isize;
        map.y_size = map.grid.borrow()[0].len() as isize;

        for y in 0..map.y_size {
            map.starts.insert(
                NodeExit {
                    x: -1,
                    y,
                    exit_dir: Direction::E,
                },
                RefCell::new(false),
            );
            map.starts.insert(
                NodeExit {
                    x: map.x_size,
                    y,
                    exit_dir: Direction::W,
                },
                RefCell::new(false),
            );
        }
        for x in 0..map.x_size {
            map.starts.insert(
                NodeExit {
                    x,
                    y: -1,
                    exit_dir: Direction::S,
                },
                RefCell::new(false),
            );
            map.starts.insert(
                NodeExit {
                    x,
                    y: map.y_size,
                    exit_dir: Direction::N,
                },
                RefCell::new(false),
            );
        }

        let mut max = 0;

        let starts_copy = map.starts.keys().map(|k| *k).collect::<Vec<NodeExit>>();

        for start in starts_copy {
            if map.starts[&start].borrow().clone() {
                continue;
            }
            let mut current_loc_and_brearings = vec![start];
            map.reset();

            while let Some(loc) = current_loc_and_brearings.pop() {
                current_loc_and_brearings.extend(map.visit_node(loc));
            }
            let new_val = map.grid.borrow().iter().flatten().filter(|n| n.lit).count();
            max = std::cmp::max(max, new_val);
        }

        max.to_string()
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
    #[test]
    fn full_input_tests() {
        let input = include_str!("input.txt");
        assert_eq!(super::Solver16.part1(input.lines()), "6921");
        assert_eq!(super::Solver16.part2(input.lines()), "7594");
    }
}
