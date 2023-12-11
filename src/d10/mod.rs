use crate::Solver;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    str::Lines,
};

pub struct Solver10;

#[derive(Debug)]
struct Pipe {
    x: i32,
    y: i32,

    // map of input direction to output direction
    directions: RefCell<HashMap<Direction, Direction>>,
    pipetype: PipeType,
}

#[derive(Debug, PartialEq)]
enum PipeType {
    Pipe,
    Null,
    Start,
}

impl Pipe {
    fn new(c: char, x: i32, y: i32) -> Pipe {
        match c {
            '-' => Pipe {
                x,
                y,
                directions: RefCell::new(Pipe::create_pipe_hashmap(Direction::L, Direction::R)),
                pipetype: PipeType::Pipe,
            },
            '|' => Pipe {
                x,
                y,
                directions: RefCell::new(Pipe::create_pipe_hashmap(Direction::U, Direction::D)),
                pipetype: PipeType::Pipe,
            },
            'L' => Pipe {
                x,
                y,
                directions: RefCell::new(Pipe::create_pipe_hashmap(Direction::R, Direction::U)),
                pipetype: PipeType::Pipe,
            },
            'J' => Pipe {
                x,
                y,
                directions: RefCell::new(Pipe::create_pipe_hashmap(Direction::L, Direction::U)),
                pipetype: PipeType::Pipe,
            },
            'F' => Pipe {
                x,
                y,
                directions: RefCell::new(Pipe::create_pipe_hashmap(Direction::R, Direction::D)),
                pipetype: PipeType::Pipe,
            },
            '7' => Pipe {
                x,
                y,
                directions: RefCell::new(Pipe::create_pipe_hashmap(Direction::L, Direction::D)),
                pipetype: PipeType::Pipe,
            },
            'S' => Pipe {
                x,
                y,
                directions: RefCell::new(HashMap::new()),
                pipetype: PipeType::Start,
            },
            '.' => Pipe {
                x,
                y,
                directions: RefCell::new(HashMap::new()),
                pipetype: PipeType::Null,
            },
            _ => panic!("Invalid pipe char"),
        }
    }

    fn is_connected(&self, other: &Pipe) -> bool {
        if self.x == other.x {
            if self.y == other.y + 1 {
                return self.directions.borrow().contains_key(&Direction::U)
                    && (other.pipetype == PipeType::Start
                        || other.directions.borrow().contains_key(&Direction::D));
            } else if self.y == other.y - 1 {
                return self.directions.borrow().contains_key(&Direction::D)
                    && (other.pipetype == PipeType::Start
                        || other.directions.borrow().contains_key(&Direction::U));
            }
        } else if self.y == other.y {
            if self.x == other.x + 1 {
                return self.directions.borrow().contains_key(&Direction::L)
                    && (other.pipetype == PipeType::Start
                        || other.directions.borrow().contains_key(&Direction::R));
            } else if self.x == other.x - 1 {
                return self.directions.borrow().contains_key(&Direction::R)
                    && (other.pipetype == PipeType::Start
                        || other.directions.borrow().contains_key(&Direction::L));
            }
        }
        false
    }

    fn get_next_location(&self, previous: &Pipe) -> ((i32, i32), Turn, Direction) {
        assert!(self.is_connected(previous));

        let input_side: Direction = match (self.x - previous.x, self.y - previous.y) {
            (0, 1) => Direction::U,
            (0, -1) => Direction::D,
            (1, 0) => Direction::L,
            (-1, 0) => Direction::R,
            _ => panic!("Invalid pipe connection"),
        };

        let output_side: Direction = *self.directions.borrow().get(&input_side).unwrap();
        let next_loc = match output_side {
            Direction::U => (self.x, self.y - 1),
            Direction::D => (self.x, self.y + 1),
            Direction::L => (self.x - 1, self.y),
            Direction::R => (self.x + 1, self.y),
            _ => panic!("no output side"),
        };

        let turn = match (input_side, output_side) {
            (Direction::U, Direction::L)
            | (Direction::D, Direction::R)
            | (Direction::L, Direction::D)
            | (Direction::R, Direction::U) => Turn::R,
            (Direction::U, Direction::R)
            | (Direction::D, Direction::L)
            | (Direction::L, Direction::U)
            | (Direction::R, Direction::D) => Turn::L,
            _ => Turn::N,
        };
        (next_loc, turn, output_side)
    }

    fn create_pipe_hashmap(dir_a: Direction, dir_b: Direction) -> HashMap<Direction, Direction> {
        let mut map = HashMap::new();
        map.insert(dir_a, dir_b);
        map.insert(dir_b, dir_a);
        map
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Turn {
    L,
    R,
    N,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    U,
    D,
    L,
    R,
    N,
}

impl Solver for Solver10 {
    fn day_number(&self) -> u32 {
        10
    }

    fn part1(&self, input_lines: Lines) -> String {
        let mut map: Vec<Vec<Pipe>> = Vec::new();

        let mut start: (usize, usize) = (0, 0);

        for (y, line) in input_lines.enumerate() {
            let mut row: Vec<Pipe> = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let new_pipe = Pipe::new(c, x as i32, y as i32);
                if new_pipe.pipetype == PipeType::Start {
                    start = (new_pipe.x as usize, new_pipe.y as usize);
                }
                row.push(new_pipe);
            }
            map.push(row);
        }

        let start = get_location(&map, start.0, start.1).unwrap();
        let mut previous = start;
        let mut current = start;

        // Find which direction to go from the start
        let start_options = vec![
            (start.x, start.y - 1),
            (start.x, start.y + 1),
            (start.x - 1, start.y),
            (start.x + 1, start.y),
        ];

        for option in start_options {
            if option.0 < 0 || option.1 < 0 {
                continue;
            }
            if let Some(next) = get_location(&map, option.0 as usize, option.1 as usize) {
                if next.is_connected(start) {
                    current = next;
                    break;
                }
            }
        }

        let mut count = 1;

        while current.pipetype != PipeType::Start {
            let next_location = current.get_next_location(previous).0;
            previous = current;
            current =
                get_location(&map, next_location.0 as usize, next_location.1 as usize).unwrap();
            count += 1;
        }

        (count / 2).to_string()
    }

    fn part2(&self, input_lines: Lines) -> String {
        let mut map: Vec<Vec<Pipe>> = Vec::new();

        let mut start: (usize, usize) = (0, 0);

        for (y, line) in input_lines.enumerate() {
            let mut row: Vec<Pipe> = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let new_pipe = Pipe::new(c, x as i32, y as i32);
                if new_pipe.pipetype == PipeType::Start {
                    start = (new_pipe.x as usize, new_pipe.y as usize);
                }
                row.push(new_pipe);
            }
            map.push(row);
        }

        let start: &Pipe = get_location(&map, start.0, start.1).unwrap();

        // Find which direction to go from the start
        let mut s_connections: Vec<Direction> = Vec::new();
        let mut next_loc_dir: (i32, i32, Direction) = (0, 0, Direction::N);

        if start.x > 0 {
            // x - 1, y
            if let Some(next) = get_location(
                &map,
                (start.x - 1).try_into().unwrap(),
                start.y.try_into().unwrap(),
            ) {
                s_connections.push(Direction::L);
                next_loc_dir = (next.x, next.y, Direction::L);
            }
        }
        // x + 1, y
        if let Some(next) = get_location(
            &map,
            (start.x + 1).try_into().unwrap(),
            start.y.try_into().unwrap(),
        ) {
            s_connections.push(Direction::R);
            next_loc_dir = (next.x, next.y, Direction::R);
        }
        if start.y > 0 {
            // x, y - 1
            if let Some(next) = get_location(
                &map,
                start.x.try_into().unwrap(),
                (start.y - 1).try_into().unwrap(),
            ) {
                s_connections.push(Direction::U);
                next_loc_dir = (next.x, next.y, Direction::U);
            }
        }
        // x, y + 1
        if let Some(next) = get_location(
            &map,
            start.x.try_into().unwrap(),
            (start.y + 1).try_into().unwrap(),
        ) {
            s_connections.push(Direction::D);
            next_loc_dir = (next.x, next.y, Direction::D);
        }

        let start_x: usize = start.x.try_into().unwrap();
        let start_y: usize = start.y.try_into().unwrap();

        start
            .directions
            .borrow_mut()
            .extend(Pipe::create_pipe_hashmap(
                s_connections[0],
                s_connections[1],
            ));

        let mut current: (&Pipe, Direction) = (
            get_location(
                &map,
                next_loc_dir.0.try_into().unwrap(),
                next_loc_dir.1.try_into().unwrap(),
            )
            .unwrap(),
            next_loc_dir.2,
        );

        let mut previous: &Pipe = get_location(&map, start_x, start_y).unwrap();

        // map[start_y][start_x] = replacement_start;

        let mut turn_counts = vec![0, 0];
        let mut loop_elems: Vec<(&Pipe, Direction)> = Vec::new();
        let mut locations_of_main_loop: HashSet<(i32, i32)> = HashSet::new();

        // Dont add the start element, as it'll be put into the sets when we find it at the end of the loop
        loop_elems.push(current);
        locations_of_main_loop.insert((current.0.x, current.0.y));

        while current.0.pipetype != PipeType::Start {
            let (next_location, turn, dir) = current.0.get_next_location(previous);

            match turn {
                Turn::L => turn_counts[0] += 1,
                Turn::R => turn_counts[1] += 1,
                _ => (),
            }
            previous = current.0;
            current = (
                get_location(&map, next_location.0 as usize, next_location.1 as usize).unwrap(),
                dir,
            );
            locations_of_main_loop.insert((current.0.x, current.0.y));
            loop_elems.push(current);
        }

        let more_lefts = turn_counts[0] > turn_counts[1];

        let search_dir = if more_lefts {
            Direction::D
        } else {
            Direction::U
        };

        let mut tiles_inside = 0;

        // to count included tiles:
        // - only count in one direction from the track - we'll choose ones to the right
        // - only count when going up (if there are more right turns) or down (if there are more left turns)
        // - count until you hit a tile that's part of the loop
        for (p, dir) in loop_elems {
            let input_side = match dir {
                Direction::U => Direction::D,
                Direction::D => Direction::U,
                Direction::L => Direction::R,
                Direction::R => Direction::L,
                _ => panic!("no input side"),
            };
            let next_dir = *p.directions.borrow().get(&input_side).unwrap();

            if dir == search_dir || next_dir == search_dir {
                let row = map.get(p.y as usize).unwrap();
                for location in row.iter().skip(p.x as usize + 1) {
                    if locations_of_main_loop.contains(&(location.x, location.y)) {
                        break;
                    }
                    tiles_inside += 1;
                }
            }
        }

        tiles_inside.to_string()
    }
}

fn get_location<'a>(map: &'a Vec<Vec<Pipe>>, x_ix: usize, y_ix: usize) -> Option<&'a Pipe> {
    if let Some(row) = map.get(y_ix) {
        if let Some(pipe) = row.get(x_ix) {
            return Some(pipe);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
        let sample_input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(super::Solver10.part1(sample_input.lines()), "4");

        let sample_input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(super::Solver10.part1(sample_input.lines()), "8");
    }
    #[test]
    fn part2() {
        let sample_input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(super::Solver10.part2(sample_input.lines()), "4");

        let sample_input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(super::Solver10.part2(sample_input.lines()), "8");

        let sample_input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(super::Solver10.part2(sample_input.lines()), "10");
    }
}
