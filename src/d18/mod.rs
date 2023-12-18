use std::{collections::VecDeque, fs::DirBuilder, str::Lines, vec};

use num::{traits::int, PrimInt};

use crate::Solver;
pub struct Solver18;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum NodeType {
    Unknown,
    Trench,
    ToLeft,
    ToRight,
}

struct Node {
    node_type: NodeType,
}
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
        // Don't know how much space we'll need.  Count the ups and rights, and double them + 3
        // (1 for the origin, and 2 for an extra space on either side of the trench)
        let (rights, ups) = input_lines
            .clone()
            .filter_map(|line| {
                let mut splits = line.split(' ');
                let dir = splits.next().unwrap();
                let dist = splits.next().unwrap().parse::<u64>().unwrap();
                if dir == "R" {
                    Some((dist, 0))
                } else if dir == "U" {
                    Some((0, dist))
                } else {
                    None
                }
            })
            .fold((0, 0), |(r, u), (r2, u2)| (r + r2, u + u2));

        let mut current_location = (rights as usize + 1, ups as usize + 1);

        let rights = rights * 2 + 3;
        let ups = ups * 2 + 3;

        let mut grid = vec![vec![NodeType::Unknown; rights as usize * 2 + 3]; ups as usize * 2 + 3];

        let mut last_dir = b'R'; // Doesn't actually matter which
        let mut right_turns: isize = 0;
        let mut vec_of_lefts: Vec<(usize, usize)> = vec![];
        let mut vec_of_rights: Vec<(usize, usize)> = vec![];

        let mut bounds = (
            current_location.0,
            current_location.1,
            current_location.0,
            current_location.1,
        );

        for line in input_lines {
            let mut splits = line.split(' ');
            let dir = splits.next().unwrap().bytes().next().unwrap();
            let dist = splits.next().unwrap().parse::<usize>().unwrap();

            let (x, y) = current_location;
            match dir {
                b'R' => {
                    for i in 0..=dist {
                        grid[y][x + i as usize] = NodeType::Trench;

                        if grid[y + 1][x + i as usize] == NodeType::Unknown {
                            grid[y + 1][x + i as usize] = NodeType::ToRight;
                            vec_of_rights.push((x + i as usize, y + 1));
                        }
                        if grid[y - 1][x + i as usize] == NodeType::Unknown {
                            grid[y - 1][x + i as usize] = NodeType::ToLeft;
                            vec_of_lefts.push((x + i as usize, y - 1));
                        }
                    }
                    current_location = (x + dist as usize, y);

                    match last_dir {
                        b'U' => right_turns += 1,
                        b'D' => right_turns -= 1,
                        _ => {}
                    }
                }
                b'U' => {
                    for i in 0..=dist {
                        grid[y - i as usize][x] = NodeType::Trench;
                        if grid[y - i][x + 1] == NodeType::Unknown {
                            grid[y - i][x + 1] = NodeType::ToRight;
                            vec_of_rights.push((x + 1 as usize, y - i));
                        }
                        if grid[y - i][x - 1] == NodeType::Unknown {
                            grid[y - i][x - 1] = NodeType::ToLeft;
                            vec_of_lefts.push((x - 1 as usize, y - 1));
                        }
                    }
                    current_location = (x, y - dist as usize);
                    match last_dir {
                        b'L' => right_turns += 1,
                        b'R' => right_turns -= 1,
                        _ => {}
                    }
                }
                b'L' => {
                    for i in 0..=dist {
                        grid[y][x - i as usize] = NodeType::Trench;
                        if grid[y - 1][x - i as usize] == NodeType::Unknown {
                            grid[y - 1][x - i as usize] = NodeType::ToRight;
                            vec_of_rights.push((x - i as usize, y - 1));
                        }
                        if grid[y + 1][x - i as usize] == NodeType::Unknown {
                            grid[y + 1][x - i as usize] = NodeType::ToLeft;
                            vec_of_lefts.push((x - i as usize, y + 1));
                        }
                    }
                    current_location = (x - dist as usize, y);
                    match last_dir {
                        b'D' => right_turns += 1,
                        b'U' => right_turns -= 1,
                        _ => {}
                    }
                }
                b'D' => {
                    for i in 0..=dist {
                        // println!("{} {}", x, y + i as usize);
                        grid[y + i as usize][x] = NodeType::Trench;
                        if grid[y + i][x - 1 as usize] == NodeType::Unknown {
                            grid[y + i][x - 1 as usize] = NodeType::ToRight;
                            vec_of_rights.push((x - 1 as usize, y + i));
                        }
                        if grid[y + i][x + 1 as usize] == NodeType::Unknown {
                            grid[y + i][x + 1 as usize] = NodeType::ToLeft;
                            vec_of_lefts.push((x + 1 as usize, y + i));
                        }
                    }
                    current_location = (x, y + dist as usize);
                    match last_dir {
                        b'R' => right_turns += 1,
                        b'L' => right_turns -= 1,
                        _ => {}
                    }
                }
                _ => panic!("Unknown direction"),
            }
            if current_location.0 > bounds.2 {
                bounds.2 = current_location.0;
            }
            if current_location.1 > bounds.3 {
                bounds.3 = current_location.1;
            }
            if current_location.0 < bounds.0 {
                bounds.0 = current_location.0;
            }
            if current_location.1 < bounds.1 {
                bounds.1 = current_location.1;
            }
            last_dir = dir;
        }

        let (flood_type, mut flood_list) = if right_turns > 0 {
            (NodeType::ToRight, vec_of_rights)
        } else {
            (NodeType::ToLeft, vec_of_lefts)
        };
        // Got a laid out grid.  Now flood fill the map

        // for r in bounds.1 - 1..=bounds.3 + 1 {
        //     let row = &grid[r];
        //     for c in bounds.0 - 1..=bounds.2 + 1 {
        //         let node = &row[c];
        //         print!(
        //             "{}",
        //             match node {
        //                 NodeType::Unknown => '.',
        //                 NodeType::Trench => '#',
        //                 NodeType::ToLeft => 'L',
        //                 NodeType::ToRight => 'R',
        //             }
        //         )
        //     }
        //     println!();
        // }
        //     .iter()
        //     .filter(|r| r.iter().any(|n| n.node_type != NodeType::Unknown))
        // {
        //     for n in row {
        //         print!(
        //             "{}",
        //             match n.node_type {
        //                 NodeType::Unknown => '.',
        //                 NodeType::Trench => '#',
        //                 NodeType::ToLeft => 'L',
        //                 NodeType::ToRight => 'R',
        //             }
        //         )
        //     }
        //     println!("")
        // }

        // println!("Flood type: {:?}", flood_type);
        // println!("Flood list: {:?}", flood_list);

        while let Some(location) = flood_list.pop() {
            if grid[location.1][location.0] != NodeType::Trench {
                // println!("Filling {:?}", location);
                // if location == (0, 870) {
                // break;
                // }
                grid[location.1][location.0] = NodeType::Trench;

                for neighbor in [
                    (location.0, location.1 + 1),
                    (location.0, location.1 - 1),
                    (location.0 + 1, location.1),
                    (location.0 - 1, location.1),
                ] {
                    flood_list.push(neighbor);
                }
            }
        }

        // for r in bounds.1 - 1..=bounds.3 + 1 {
        //     let row = &grid[r];
        //     for c in bounds.0 - 1..=bounds.2 + 1 {
        //         let node = &row[c];
        //         print!(
        //             "{}",
        //             match node {
        //                 NodeType::Unknown => '.',
        //                 NodeType::Trench => '#',
        //                 NodeType::ToLeft => 'L',
        //                 NodeType::ToRight => 'R',
        //             }
        //         )
        //     }
        //     println!();
        // }

        // Now we have a grid with all the nodes filled in.  Count the number of trench nodes
        let trench_count = grid
            .iter()
            .flatten()
            .filter(|n| **n == NodeType::Trench)
            .count();

        trench_count.to_string()
    }

    fn part2(&self, input_lines: Lines) -> String {
        // Make a list of directions
        let dist_dirs = input_lines.map(|line| {
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

        // let dist_dirs = input_lines.map(|line| {
        //     let (dir, rest) = line.split_at(1);
        //     let dist = rest[1..].split(' ').nth(0).unwrap();
        //     let dist = dist.parse::<isize>().unwrap();
        //     let dir = dir.as_bytes()[0];
        //     (dist, dir)
        // });

        let mut current_loc = Point { x: 0, y: 0 };
        let mut vert_lines: VecDeque<Line> = vec![].into();
        let mut total_line_length = 0_i64;

        for (dist, dir) in dist_dirs.clone() {
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
        // println!("Vert lines: {:?}", vert_lines);

        let mut sum = 0_i64;
        let mut iii = 0;

        while let Some(mut opening_line) = vert_lines.pop_front() {
            // println!("Opening line: {:?}", opening_line);
            let closing_line_ix = vert_lines
                .iter()
                .position(|line| line.top.y == opening_line.top.y)
                .unwrap();
            let mut closing_line = vert_lines.remove(closing_line_ix).unwrap();
            // println!("closing line: {:?}", closing_line);

            match opening_line.bottom.y.cmp(&closing_line.bottom.y) {
                std::cmp::Ordering::Less => {
                    // opening line is shorter
                    // Truncate the closing line.
                    let mut remainder = closing_line;
                    remainder.top.y = opening_line.bottom.y;
                    insert(&mut vert_lines, remainder);
                    closing_line.bottom.y = opening_line.bottom.y;
                }
                std::cmp::Ordering::Greater => {
                    // closing line is shorter
                    // Truncate the opening line.
                    let mut remainder = opening_line;
                    remainder.top.y = closing_line.bottom.y;
                    insert(&mut vert_lines, remainder);
                    opening_line.bottom.y = closing_line.bottom.y;
                }
                std::cmp::Ordering::Equal => {}
            }

            // let common_bottom = std::cmp::min(opening_line.bottom.y, closing_line.bottom.y);

            let common_rect = Rectangle {
                top_left: opening_line.top,
                bottom_right: closing_line.bottom,
            };

            // next interfering
            let mut interfering_line_ix = None;
            for (ix, line) in vert_lines.iter_mut().enumerate() {
                // println!("Checking line: {:?} against rect {:?}", line, common_rect);
                match line.top.y.cmp(&common_rect.bottom_right.y) {
                    std::cmp::Ordering::Less => {
                        // println!("Less");
                        if line.top.x > common_rect.top_left.x
                            && line.top.x < common_rect.bottom_right.x
                        {
                            // println!("Interfering line: {:?}", line);
                            interfering_line_ix = Some(ix);
                            break;
                        }
                    }
                    std::cmp::Ordering::Greater => {
                        break;
                    }
                    std::cmp::Ordering::Equal => {
                        break;
                    } //     if line.top.x >= common_rect.top_left.x
                      //         && line.top.x <= common_rect.bottom_right.x
                      //     {
                      //         interfering_line_ix = Some(ix);
                      //         break;
                      //     }
                      //     line.top.y += 1;
                      // }
                }
            }

            if let Some(interfering_line_ix) = interfering_line_ix {
                let interfering_point = vert_lines[interfering_line_ix].top;

                // interfering_line_ix = Some(line_ix);
                // println!("Interfering point: {:?}", interfering_point);

                // Truncate the opening line.
                let mut remainder = opening_line;
                remainder.top.y = interfering_point.y;
                insert(&mut vert_lines, remainder);
                opening_line.bottom.y = interfering_point.y;

                // Truncate the closing line.
                let mut remainder = closing_line;
                remainder.top.y = interfering_point.y;
                insert(&mut vert_lines, remainder);
                closing_line.bottom.y = interfering_point.y;
            }

            // println!("opening line: {:?}", opening_line);
            // println!("closing line: {:?}", closing_line);
            // println!(
            //     "ysize : {}",
            //     (closing_line.bottom.y - opening_line.top.y + 1)
            // );
            // println!(
            //     "xsize : {}",
            //     (closing_line.bottom.x - opening_line.top.x + 1)
            // );
            // println!(
            //     "adding rectangle: {:?}",
            //     Rectangle {
            //         top_left: opening_line.top,
            //         bottom_right: closing_line.bottom,
            //     }
            // );
            sum += (closing_line.bottom.y - opening_line.top.y) as i64
                * (closing_line.bottom.x - opening_line.top.x) as i64;

            // println!("Vert lines: {:?}", vert_lines);
            // iii += 1;
            // if iii == 5 {
            //     break;
            // }
        }

        // let rectangles: Vec = vec![];

        sum += 1 + (total_line_length / 2 as i64);
        sum.to_string()
    }
}

fn insert(vert_lines: &mut VecDeque<Line>, line_to_insert: Line) {
    for ii in 0..vert_lines.len() {
        let line = vert_lines[ii];
        if line.top.y > line_to_insert.top.y
            || (line.top.y == line_to_insert.top.y && line.top.x > line_to_insert.top.x)
        {
            // println!("Inserting {:?} at {}", line_to_insert, ii);
            vert_lines.insert(ii, line_to_insert);
            return;
        }
    }
    // println!("Inserting {:?} at back", line_to_insert);
    vert_lines.push_back(line_to_insert);
}

// fn something(vert_lines: &mut Vec<Line>, opening_line: Line) -> usize {
//     for ii in 0..vert_lines.len() {
//         let line = vert_lines[ii];
//         if line.top.y == opening_line.top.y {
//             println!("Found a match: {:?}", line);
//         }
//         return ii;
//     }
//     panic!("No match found");
// }
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
