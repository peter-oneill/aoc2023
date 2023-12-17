use std::{cell::RefCell, str::Lines, thread::current};

use crate::Solver;
pub struct Solver17;

#[derive(Copy, Clone)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Copy, Clone, Debug)]
struct Node {
    x: isize,
    y: isize,
    heat_loss: usize,
    leasts_by_dir: [[Option<usize>; 3]; 4],
    visited_by_dir: [[bool; 3]; 4],
    weight: Option<usize>,
}

impl Node {
    fn new(x: usize, y: usize, heat_loss: char) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
            heat_loss: heat_loss.to_digit(10).unwrap() as usize,
            leasts_by_dir: [[None; 3]; 4],
            visited_by_dir: [[false; 3]; 4],
            weight: None,
        }
    }

    fn make_start_node(&mut self) {
        self.weight = Some(0);
        self.leasts_by_dir = [[Some(0); 3]; 4];
        self.visited_by_dir = [[true; 3]; 4];
    }

    fn update_cost(
        &mut self,
        heat_loss: usize,
        direction: Direction,
        conseq_steps: usize,
        search_nodes: &mut Vec<(isize, isize, usize)>,
    ) {
        let leasts = &mut self.leasts_by_dir[direction as usize];
        let heat_loss = heat_loss + self.heat_loss;

        if leasts[conseq_steps].is_some_and(|loss| loss <= heat_loss) {
            // We already have a better path
            return;
        }

        leasts[conseq_steps] = Some(heat_loss);

        if !self.weight.is_some_and(|w| w <= heat_loss) {
            // This is a cheaper path for the next round of node selection.
            search_nodes.push((self.x, self.y, heat_loss));
            self.weight = Some(heat_loss);
        }

        // This path could also be strictly than paths with more consecutive steps
        for least in leasts[conseq_steps..].iter_mut() {
            if !least.is_some_and(|loss| loss < heat_loss) {
                *least = Some(heat_loss);
            }
        }
    }

    fn min_heat_loss(&self, dir1: Direction, dir2: Direction) -> Option<usize> {
        [
            self.leasts_by_dir[dir1 as usize],
            self.leasts_by_dir[dir2 as usize],
        ]
        .iter()
        .flatten()
        .filter_map(|v| *v)
        .min()

        // [self.leasts_by_dir[dir1 as usize][0], self.leasts_by_dir[dir2 as usize][0]]
        //     .iter()
        //     .filter_map(|x| *x)
        //     .min()
    }
}

struct Map {
    grid: Grid,
    search_nodes: Vec<(isize, isize, usize)>,
}

struct Grid {
    inner: Vec<Vec<Node>>,
}

impl Grid {
    fn get_mut(&mut self, y: isize, x: isize) -> Option<&mut Node> {
        self.inner
            .get_mut::<usize>(y.try_into().ok()?)?
            .get_mut::<usize>(x.try_into().ok()?)
    }

    fn get(&mut self, y: isize, x: isize) -> Option<&Node> {
        self.get_mut(y, x).map(|n| n as &Node)
    }
}

impl Map {
    fn solve_from_location(&mut self, x: isize, y: isize) {
        self.search_nodes.push((x, y, 0));
        self.grid.get_mut(y, x).unwrap().make_start_node();

        // let mut lit = 0;
        while !self.search_nodes.is_empty() {
            self.search_nodes.sort_by(|a, b| b.2.cmp(&a.2));
            // println!("search nodes: {:?}", self.search_nodes);
            let (x, y, _) = self.search_nodes.pop().unwrap(); // sort_by(|a, b| b.2.cmp(&a.2)).pop();

            // println!("going from node {},{}", x, y);
            // for (rix, r) in self.grid.inner.iter().enumerate() {
            //     for (cix, c) in r.iter().enumerate() {
            //         println!(
            //             "{cix},{rix} :{:?}",
            //             c.leasts_by_dir
            //                 .iter()
            //                 .flatten()
            //                 .map(|v| v.unwrap_or(0))
            //                 .collect::<Vec<usize>>()
            //         );
            //     }
            //     println!();
            // }

            let current_node = self.grid.get_mut(y, x).unwrap();
            if current_node.weight == None {
                continue;
            }

            current_node.weight = None;

            // Take of copy of the current node to use for values when updating surrounding nodes
            // Resolves ownership / multiple borrow issues
            let current_node = self.grid.inner[y as usize][x as usize];

            // Update each of the surrounding nodes
            let min_up_down = current_node.min_heat_loss(Direction::N, Direction::S);
            let min_left_right = current_node.min_heat_loss(Direction::W, Direction::E);

            // N
            if let Some(node) = self.grid.get_mut(y - 1, x) {
                if let Some(heat_loss) = min_left_right {
                    node.update_cost(heat_loss, Direction::N, 0, &mut self.search_nodes);
                }
                Self::continue_straight_line(
                    current_node,
                    node,
                    Direction::N,
                    &mut self.search_nodes,
                )
            }
            // S
            if let Some(node) = self.grid.get_mut(y + 1, x) {
                if let Some(heat_loss) = min_left_right {
                    node.update_cost(heat_loss, Direction::S, 0, &mut self.search_nodes);
                }
                Self::continue_straight_line(
                    current_node,
                    node,
                    Direction::S,
                    &mut self.search_nodes,
                )
            }
            // W
            if let Some(node) = self.grid.get_mut(y, x - 1) {
                if let Some(heat_loss) = min_up_down {
                    node.update_cost(heat_loss, Direction::W, 0, &mut self.search_nodes);
                }
                Self::continue_straight_line(
                    current_node,
                    node,
                    Direction::W,
                    &mut self.search_nodes,
                )
            }
            // E
            if let Some(node) = self.grid.get_mut(y, x + 1) {
                if let Some(heat_loss) = min_up_down {
                    node.update_cost(heat_loss, Direction::E, 0, &mut self.search_nodes);
                }
                Self::continue_straight_line(
                    current_node,
                    node,
                    Direction::E,
                    &mut self.search_nodes,
                )
            }
        }
    }

    fn continue_straight_line(
        current_node: Node,
        next_node: &mut Node,
        dir: Direction,
        search_nodes: &mut Vec<(isize, isize, usize)>,
    ) {
        if let Some(heat_loss) = current_node.leasts_by_dir[dir as usize][0] {
            next_node.update_cost(heat_loss, dir, 1, search_nodes);
        }
        if let Some(heat_loss) = current_node.leasts_by_dir[dir as usize][1] {
            next_node.update_cost(heat_loss, dir, 2, search_nodes);
        }
        // if let Some(heat_loss) = current_node.leasts_by_dir[dir as usize][2] {
        // next_node.update_cost(heat_loss + current_node.heat_loss, dir, 3, search_nodes);
        // }
    }
}
impl Solver for Solver17 {
    fn day_number(&self) -> u32 {
        17
    }

    fn part1(&self, input_lines: Lines) -> String {
        let inner_grid = input_lines
            .enumerate()
            .map(|(line_ix, line)| {
                line.char_indices()
                    .map(|(c_ix, c)| Node::new(c_ix, line_ix, c))
                    .collect::<Vec<Node>>()
            })
            .collect::<Vec<Vec<Node>>>();

        let end_location = (inner_grid[0].len() - 1, inner_grid.len() - 1);

        let mut map = Map {
            grid: Grid { inner: inner_grid },
            search_nodes: Vec::with_capacity(500),
        };

        map.solve_from_location(0, 0);
        let end_node: Node = *map
            .grid
            .get(end_location.0 as isize, end_location.1 as isize)
            .unwrap();

        let min = end_node
            .leasts_by_dir
            .iter()
            .flatten()
            .filter_map(|v| *v)
            .min()
            .unwrap();

        // for (rix, r) in map.grid.inner.iter().enumerate() {
        //     for (cix, c) in r.iter().enumerate() {
        //         println!(
        //             "{cix},{rix} :{:?}",
        //             c.leasts_by_dir
        //                 .iter()
        //                 .flatten()
        //                 .map(|v| v.unwrap_or(0))
        //                 .collect::<Vec<usize>>()
        //         );
        //     }
        //     println!();
        // }

        (min).to_string()
    }

    fn part2(&self, _input_lines: Lines) -> String {
        "0".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
        let _ = "
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let sample_input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(super::Solver17.part1(sample_input.lines()), "102");
    }

    #[test]
    fn part2() {
        let sample_input = "";
        assert_eq!(super::Solver17.part2(sample_input.lines()), "0");
    }
}
