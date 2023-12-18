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

#[derive(Clone, Debug)]
struct Node {
    x: isize,
    y: isize,
    heat_loss: usize,
    leasts_by_dir: Vec<Vec<Option<usize>>>,
}

impl Node {
    fn new(x: usize, y: usize, heat_loss: char, max_steps: usize) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
            heat_loss: heat_loss.to_digit(10).unwrap() as usize,
            leasts_by_dir: vec![vec![None; max_steps]; 4],
        }
    }

    fn make_start_node(&mut self) {
        self.leasts_by_dir = vec![vec![Some(0); self.leasts_by_dir[0].len()]; 4];
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

        search_nodes.add(heat_loss, (self.x, self.y, direction, conseq_steps));
    }
}

struct OrderedMap {
    map: Vec<Vec<(isize, isize, Direction, usize)>>,
}
impl OrderedMap {
    fn new(s: usize) -> Self {
        Self {
            map: vec![vec![]; s],
        }
    }
    fn add(&mut self, key: usize, value: (isize, isize, Direction, usize)) {
        self.map[key].push(value);
    }

    fn pop(&mut self) -> Option<(isize, isize, Direction, usize)> {
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
    #[allow(dead_code)]
    fn print(&self) {
        for row in &self.grid.inner {
            println!("Y: {} ", row[0].y);
            for node in row {
                println!(
                    "X: {} {:?} ",
                    node.x,
                    node.leasts_by_dir
                        .iter()
                        .map(|v| v.iter().map(|v| v.unwrap_or(0)).collect::<Vec<usize>>())
                        .collect::<Vec<Vec<usize>>>()
                );
            }
            println!();
        }
    }
    fn solve_from_location(&mut self, x: isize, y: isize, end_location: (usize, usize)) -> usize {
        self.search_nodes.add(0, (x, y, Direction::N, 0));
        self.search_nodes.add(0, (x, y, Direction::E, 0));

        self.grid.get_mut(y, x).unwrap().make_start_node();

        while let Some((x, y, dir, steps)) = self.search_nodes.pop() {
            let current_node = &self.grid.inner[y as usize][x as usize];
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
            // Update each of the reachable nodes
            let heat_loss =
                self.grid.inner[y as usize][x as usize].leasts_by_dir[dir as usize][steps].unwrap();

            match dir {
                Direction::N | Direction::S => {
                    let mut updated_heat_loss = heat_loss;
                    for step_ix in 0..std::cmp::min(
                        self.max_straight_line,
                        self.grid.inner[0].len() - x as usize - 1,
                    ) {
                        let node =
                            &mut self.grid.inner[y as usize][(x + step_ix as isize + 1) as usize];
                        if step_ix >= self.min_straight_line - 1 {
                            node.update_cost(
                                updated_heat_loss,
                                Direction::E,
                                step_ix,
                                &mut self.search_nodes,
                            );
                        }
                        updated_heat_loss += node.heat_loss;
                    }
                    let mut updated_heat_loss = heat_loss;
                    for step_ix in 0..std::cmp::min(self.max_straight_line, x as usize) {
                        let node =
                            &mut self.grid.inner[y as usize][(x - step_ix as isize - 1) as usize];

                        // if let Some(node) = self.grid.get_mut(y, x - step_ix as isize - 1) {
                        if step_ix >= self.min_straight_line - 1 {
                            node.update_cost(
                                updated_heat_loss,
                                Direction::W,
                                step_ix,
                                &mut self.search_nodes,
                            );
                        }
                        updated_heat_loss += node.heat_loss;
                    }
                }

                Direction::W | Direction::E => {
                    let mut updated_heat_loss = heat_loss;
                    for step_ix in 0..std::cmp::min(
                        self.max_straight_line,
                        self.grid.inner.len() - y as usize - 1,
                    ) {
                        let node =
                            &mut self.grid.inner[(y + step_ix as isize + 1) as usize][x as usize];
                        if step_ix >= self.min_straight_line - 1 {
                            node.update_cost(
                                updated_heat_loss,
                                Direction::S,
                                step_ix,
                                &mut self.search_nodes,
                            );
                        }
                        updated_heat_loss += node.heat_loss;
                    }
                    let mut updated_heat_loss = heat_loss;
                    for step_ix in 0..std::cmp::min(self.max_straight_line, y as usize) {
                        let node =
                            &mut self.grid.inner[(y - step_ix as isize - 1) as usize][x as usize];

                        if step_ix >= self.min_straight_line - 1 {
                            node.update_cost(
                                updated_heat_loss,
                                Direction::N,
                                step_ix,
                                &mut self.search_nodes,
                            );
                        }
                        updated_heat_loss += node.heat_loss;
                    }
                }
            }
        }
        panic!("No path found");
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
                    .map(|(c_ix, c)| Node::new(c_ix, line_ix, c, 3))
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
                    .map(|(c_ix, c)| Node::new(c_ix, line_ix, c, 10))
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
