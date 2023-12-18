use std::str::Lines;

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
    leasts_by_dir: [[Option<usize>; 10]; 4],
    visited_by_dir: [[bool; 10]; 4],
    weight: Option<usize>,
}

impl Node {
    fn new(x: usize, y: usize, heat_loss: char) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
            heat_loss: heat_loss.to_digit(10).unwrap() as usize,
            leasts_by_dir: [[None; 10]; 4],
            visited_by_dir: [[false; 10]; 4],
            weight: None,
        }
    }

    fn make_start_node(&mut self) {
        self.weight = Some(0);
        self.leasts_by_dir = [[Some(0); 10]; 4];
        self.visited_by_dir = [[true; 10]; 4];
    }

    fn update_cost(
        &mut self,
        heat_loss: usize,
        direction: Direction,
        conseq_steps: usize,
        search_nodes: &mut OrderedMap,
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
            search_nodes.add(heat_loss, (self.x, self.y));
            self.weight = Some(heat_loss);
        }
    }

    fn min_heat_loss(
        &self,
        dir1: Direction,
        dir2: Direction,
        min_straight_line: usize,
        max_straight_line: usize,
    ) -> Option<usize> {
        let a = self.leasts_by_dir[dir1 as usize][min_straight_line - 1..max_straight_line]
            .iter()
            .filter_map(|v| *v)
            .min();
        let b = self.leasts_by_dir[dir2 as usize][min_straight_line - 1..max_straight_line]
            .iter()
            .filter_map(|v| *v)
            .min();

        match (a, b) {
            (Some(a), Some(b)) => Some(std::cmp::min(a, b)),
            (_, _) => a.or(b),
        }
    }
}

struct OrderedMap {
    map: Vec<Vec<(isize, isize)>>,
}
impl OrderedMap {
    fn new(s: usize) -> Self {
        Self {
            map: vec![vec![]; s],
        }
    }
    fn add(&mut self, key: usize, value: (isize, isize)) {
        self.map[key].push(value);
    }

    fn pop(&mut self) -> Option<(isize, isize)> {
        self.map
            .iter_mut()
            .find(|vec| !vec.is_empty())
            .and_then(|vec| vec.pop())
    }
}
struct Map {
    grid: Grid,
    search_nodes: OrderedMap,
    min_straight_line: usize,
    max_straight_line: usize,
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
}

impl Map {
    fn solve_from_location(&mut self, x: isize, y: isize, end_location: (usize, usize)) -> usize {
        self.search_nodes.add(0, (x, y));

        self.grid.get_mut(y, x).unwrap().make_start_node();

        while let Some((x, y)) = self.search_nodes.pop() {
            let current_node = self.grid.get_mut(y, x).unwrap();

            if current_node.weight.is_none() {
                continue;
            }
            current_node.weight = None;

            if (x as usize, y as usize) == end_location {
                if let Some(val) = current_node
                    .leasts_by_dir
                    .iter()
                    .flat_map(|v| v[self.min_straight_line - 1..].iter().filter_map(|v| *v))
                    .min()
                {
                    return val;
                }
            }

            // Take of copy of the current node to use for values when updating surrounding nodes
            // Resolves ownership / multiple borrow issues
            let current_node = self.grid.inner[y as usize][x as usize];

            // Update each of the surrounding nodes
            let min_up_down = current_node.min_heat_loss(
                Direction::N,
                Direction::S,
                self.min_straight_line,
                self.max_straight_line,
            );
            let min_left_right = current_node.min_heat_loss(
                Direction::W,
                Direction::E,
                self.min_straight_line,
                self.max_straight_line,
            );

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
                    self.max_straight_line,
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
                    self.max_straight_line,
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
                    self.max_straight_line,
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
                    self.max_straight_line,
                )
            }
        }
        panic!("No path found");
    }

    fn continue_straight_line(
        // &self,
        current_node: Node,
        next_node: &mut Node,
        dir: Direction,
        search_nodes: &mut OrderedMap,
        max_straight_line: usize,
    ) {
        for ii in 0..(max_straight_line - 1) {
            if let Some(heat_loss) = current_node.leasts_by_dir[dir as usize][ii] {
                if heat_loss > 0 {
                    next_node.update_cost(heat_loss, dir, ii + 1, search_nodes);
                }
            }
        }
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
            search_nodes: OrderedMap::new((end_location.0 + 1) * (end_location.1 + 1) * 10),
            min_straight_line: 1,
            max_straight_line: 3,
        };

        map.solve_from_location(0, 0, end_location).to_string()
    }

    fn part2(&self, input_lines: Lines) -> String {
        let inner_grid = input_lines
            .enumerate()
            .map(|(line_ix, line)| {
                line.char_indices()
                    .map(|(c_ix, c)| Node::new(c_ix, line_ix, c))
                    .collect::<Vec<Node>>()
            })
            .collect::<Vec<Vec<Node>>>();

        let end_location = (inner_grid[0].len() - 1, inner_grid.len() - 1);

        let min_straight_line = 4;
        let max_straight_line = 10;

        let mut map = Map {
            grid: Grid { inner: inner_grid },
            search_nodes: OrderedMap::new((end_location.0 + 1) * (end_location.1 + 1) * 10),
            min_straight_line,
            max_straight_line,
        };
        map.solve_from_location(0, 0, end_location).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
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
        assert_eq!(super::Solver17.part2(sample_input.lines()), "94");

        let sample_input_2 = "111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(super::Solver17.part2(sample_input_2.lines()), "71");
    }

    #[test]
    fn test_actual() {
        let input = include_str!("input.txt");
        assert_eq!(super::Solver17.part1(input.lines()), "758");
        assert_eq!(super::Solver17.part2(input.lines()), "892");
    }
}
