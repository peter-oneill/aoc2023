use std::{collections::VecDeque, str::Lines, vec};

use crate::Solver;
pub struct Solver18;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}
#[derive(Debug, Clone, Copy)]
struct Line {
    top: Point,
    bottom: Point,
}

impl Line {
    fn truncate(&mut self, new_bottom: isize, lines: &mut VecDeque<Line>) {
        let mut remainder = *self;
        remainder.top.y = new_bottom;
        insert(lines, remainder);
        self.bottom.y = new_bottom;
    }
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Solver for Solver18 {
    fn day_number(&self) -> u32 {
        18
    }

    fn part1(&self, input_lines: Lines) -> String {
        let distance_and_directions = input_lines.map(|line| {
            let mut instruction = line.split(' ');
            let dir = instruction.next().unwrap().as_bytes()[0];
            let dist = instruction.next().unwrap().parse::<isize>().unwrap();
            (dist, dir)
        });

        find_area(distance_and_directions)
    }

    fn part2(&self, input_lines: Lines) -> String {
        let distance_and_directions = input_lines.map(|line| {
            let instruction = line.split('#').nth(1).unwrap();
            let (dist, dir) = instruction.split_at(5);
            let dist = isize::from_str_radix(dist, 16).unwrap();
            let dir = match dir.as_bytes()[0] {
                b'0' => b'R',
                b'1' => b'D',
                b'2' => b'L',
                b'3' => b'U',
                _ => panic!("Unknown direction"),
            };
            (dist, dir)
        });

        find_area(distance_and_directions)
    }
}

fn find_area(distance_and_directions: impl Iterator<Item = (isize, u8)>) -> String {
    let mut current_loc = Point { x: 0, y: 0 };
    let mut vert_lines: VecDeque<Line> = vec![].into();
    let mut total_line_length = 0_i64;

    for (dist, dir) in distance_and_directions {
        total_line_length += dist as i64;
        match dir {
            b'R' => current_loc.x += dist,
            b'L' => current_loc.x -= dist,
            b'U' => {
                let bottom = current_loc;
                current_loc.y -= dist;
                let top = current_loc;
                vert_lines.push_back(Line { top, bottom });
            }
            b'D' => {
                let top = current_loc;
                current_loc.y += dist;
                let bottom = current_loc;
                vert_lines.push_back(Line { top, bottom });
            }
            _ => panic!("Unknown direction"),
        }
    }

    vert_lines
        .make_contiguous()
        .sort_by(|a, b| a.top.y.cmp(&b.top.y).then(a.top.x.cmp(&b.top.x)));

    let mut sum = 0_i64;

    while let Some(mut opening_line) = vert_lines.pop_front() {
        let closing_line_ix = vert_lines
            .iter()
            .position(|line| line.top.y == opening_line.top.y)
            .unwrap();
        let mut closing_line = vert_lines.remove(closing_line_ix).unwrap();

        match opening_line.bottom.y.cmp(&closing_line.bottom.y) {
            std::cmp::Ordering::Less => {
                closing_line.truncate(opening_line.bottom.y, &mut vert_lines);
            }
            std::cmp::Ordering::Greater => {
                opening_line.truncate(closing_line.bottom.y, &mut vert_lines);
            }
            std::cmp::Ordering::Equal => {}
        }

        let common_rect = Rectangle {
            top_left: opening_line.top,
            bottom_right: closing_line.bottom,
        };

        let mut interfering_line_ix = None;
        for (ix, line) in vert_lines.iter_mut().enumerate() {
            if line.top.y < common_rect.bottom_right.y
                && line.top.x > common_rect.top_left.x
                && line.top.x < common_rect.bottom_right.x
            {
                interfering_line_ix = Some(ix);
                break;
            }
        }

        if let Some(interfering_line_ix) = interfering_line_ix {
            opening_line.truncate(vert_lines[interfering_line_ix].top.y, &mut vert_lines);
            closing_line.truncate(vert_lines[interfering_line_ix].top.y, &mut vert_lines);
        }

        sum += (closing_line.bottom.y - opening_line.top.y) as i64
            * (closing_line.bottom.x - opening_line.top.x) as i64;
    }

    sum += 1 + (total_line_length / 2 as i64);
    sum.to_string()
}

fn insert(vert_lines: &mut VecDeque<Line>, line_to_insert: Line) {
    for ii in 0..vert_lines.len() {
        let line = vert_lines[ii];
        if line.top.y > line_to_insert.top.y
            || (line.top.y == line_to_insert.top.y && line.top.x > line_to_insert.top.x)
        {
            vert_lines.insert(ii, line_to_insert);
            return;
        }
    }
    vert_lines.push_back(line_to_insert);
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
        let sample_input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(super::Solver18.part1(sample_input.lines()), "62");
    }

    #[test]
    fn part2() {
        let sample_input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(super::Solver18.part2(sample_input.lines()), "952408144115");
    }
}
