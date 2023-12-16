use std::str::Lines;

use crate::Solver;
pub struct Solver16;
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]

enum NodeType {
    Empty,
    Horizontal,
    Vertical,
    ForwardSlash,
    BackSlash,
}

#[derive(Copy, Clone)]
enum Direction {
    N = 0,
    E,
    S,
    W,
}
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]

struct Node {
    node_type: NodeType,
    lit: bool,
    entered_this_way_before: [bool; 4],
}

impl Node {
    fn new(c: u8) -> Node {
        let node_type = match c {
            b'.' => NodeType::Empty,
            b'-' => NodeType::Horizontal,
            b'|' => NodeType::Vertical,
            b'/' => NodeType::ForwardSlash,
            b'\\' => NodeType::BackSlash,
            _ => panic!("Unknown char {}", c),
        };
        Node {
            node_type,
            lit: false,
            entered_this_way_before: [false; 4],
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Map {
    grid: Vec<Vec<Node>>,
    max_x: isize,
    max_y: isize,
}

impl Map {
    fn new(grid: Vec<Vec<Node>>) -> Map {
        let max_x = grid[0].len() as isize - 1;
        let max_y = grid.len() as isize - 1;
        Map { grid, max_x, max_y }
    }
    fn reset(&mut self) {
        self.grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|node| {
                node.lit = false;
                node.entered_this_way_before = [false; 4];
            })
        });
    }

    fn get(&mut self, y: isize, x: isize) -> Option<&mut Node> {
        self.grid
            .get_mut::<usize>(y.try_into().ok()?)?
            .get_mut::<usize>(x.try_into().ok()?)
    }

    fn traverse(&mut self, x: isize, y: isize, dir: Direction) -> usize {
        let mut current_x = x;
        let mut current_y = y;
        let mut current_dir = dir;
        let mut lit = 0;
        loop {
            match self.get(current_y, current_x) {
                None => return lit,
                Some(current_node) => {
                    // Has this node already been lit when it had the same output direction before?
                    if current_node.entered_this_way_before[current_dir as usize] {
                        return lit;
                    }
                    current_node.entered_this_way_before[current_dir as usize] = true;

                    if !current_node.lit {
                        lit += 1;
                    }

                    current_node.lit = true;

                    match (current_dir, &current_node.node_type) {
                        (Direction::W, NodeType::Empty)
                        | (Direction::W, NodeType::Horizontal)
                        | (Direction::N, NodeType::BackSlash)
                        | (Direction::S, NodeType::ForwardSlash) => {
                            current_x -= 1;
                            current_dir = Direction::W;
                        }
                        (Direction::E, NodeType::Empty)
                        | (Direction::E, NodeType::Horizontal)
                        | (Direction::N, NodeType::ForwardSlash)
                        | (Direction::S, NodeType::BackSlash) => {
                            current_x += 1;
                            current_dir = Direction::E;
                        }
                        (Direction::N, NodeType::Empty)
                        | (Direction::N, NodeType::Vertical)
                        | (Direction::E, NodeType::ForwardSlash)
                        | (Direction::W, NodeType::BackSlash) => {
                            current_y -= 1;
                            current_dir = Direction::N;
                        }
                        (Direction::S, NodeType::Empty)
                        | (Direction::S, NodeType::Vertical)
                        | (Direction::E, NodeType::BackSlash)
                        | (Direction::W, NodeType::ForwardSlash) => {
                            current_y += 1;
                            current_dir = Direction::S;
                        }
                        (Direction::N, NodeType::Horizontal)
                        | (Direction::S, NodeType::Horizontal) => {
                            return lit
                                + self.traverse(current_x - 1, current_y, Direction::W)
                                + self.traverse(current_x + 1, current_y, Direction::E);
                        }
                        (Direction::E, NodeType::Vertical) | (Direction::W, NodeType::Vertical) => {
                            return lit
                                + self.traverse(current_x, current_y - 1, Direction::N)
                                + self.traverse(current_x, current_y + 1, Direction::S);
                        }
                    }
                }
            }
        }
    }
}
impl Solver for Solver16 {
    fn day_number(&self) -> u32 {
        16
    }

    fn part1(&self, input_lines: Lines) -> String {
        let grid: Vec<Vec<Node>> = input_lines
            .map(|line| line.bytes().map(Node::new).collect())
            .collect();

        let mut map = Map::new(grid);

        let lit = map.traverse(0, 0, Direction::E);

        lit.to_string()
    }

    fn part2(&self, input_lines: Lines) -> String {
        let grid: Vec<Vec<Node>> = input_lines
            .map(|line| line.bytes().map(Node::new).collect())
            .collect();

        let mut map = Map::new(grid);

        let x_len = map.grid[0].len() as isize;
        let y_len = map.grid.len() as isize;

        let mut starts = Vec::with_capacity(2 * map.grid.len() + 2 * map.grid[0].len());

        for y in 0..y_len {
            starts.push((0, y, Direction::E));
            starts.push((x_len - 1, y, Direction::W));
        }

        for x in 0..x_len {
            starts.push((x, 0, Direction::S));
            starts.push((x, y_len - 1, Direction::N));
        }

        starts
            .iter()
            .map(|(x, y, dir)| {
                map.reset();
                map.traverse(*x, *y, *dir)
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
    #[test]
    fn full_input_tests() {
        let input = include_str!("input.txt");
        assert_eq!(super::Solver16.part1(input.lines()), "6921");
        assert_eq!(super::Solver16.part2(input.lines()), "7594");
    }
}
