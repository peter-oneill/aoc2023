use crate::Solver;
use itertools::Itertools;
use std::{borrow::BorrowMut, collections::HashMap, str::Lines, vec};
pub struct Solver13;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Dir {
    Vertical,
    Horizontal,
}

#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash)]
enum NodeVal {
    Hash,
    Dot,
}

impl NodeVal {
    fn from_char(c: char) -> NodeVal {
        match c {
            '#' => NodeVal::Hash,
            '.' => NodeVal::Dot,
            _ => panic!("Invalid node value"),
        }
    }
}
impl Solver for Solver13 {
    fn day_number(&self) -> u32 {
        13
    }

    fn part1(&self, input_lines: Lines) -> String {
        let mut symmetry_sum = 0;

        let char_maps: Vec<Vec<Vec<NodeVal>>> = input_lines
            .map(|line| {
                line.chars()
                    .map(|c| NodeVal::from_char(c))
                    .collect::<Vec<NodeVal>>()
            }) // convert each line to a vector of chars
            .group_by(|line| line.is_empty()) // group the lines into groups of empty and non-empty lines
            .into_iter()
            .filter_map(|(empty, map)| if empty { None } else { Some(map.collect()) }) // drop empty lines
            .collect();

        for char_map in char_maps {
            let symm_val = find_horiz_or_vert_sym_values(&char_map)[0];
            if symm_val.1 == Dir::Vertical {
                symmetry_sum += symm_val.0 * 100;
            } else {
                symmetry_sum += symm_val.0;
            }
        }

        symmetry_sum.to_string()
    }

    fn part2(&self, input_lines: Lines) -> String {
        // naive version - just test for changing each character in each map

        let mut symmetry_sum = 0;

        let char_maps: Vec<Vec<Vec<NodeVal>>> = input_lines
            .map(|line| {
                line.chars()
                    .map(|c| NodeVal::from_char(c))
                    .collect::<Vec<NodeVal>>()
            }) // convert each line to a vector of chars
            .group_by(|line| line.is_empty()) // group the lines into groups of empty and non-empty lines
            .into_iter()
            .filter_map(|(empty, map)| if empty { None } else { Some(map.collect()) }) // drop empty lines
            .collect();

        for char_map in char_maps {
            let original_symmetry_line = find_horiz_or_vert_sym_values(&char_map)[0];

            let mut found_symmetry = false;

            for (y, row) in char_map.iter().enumerate() {
                for (x, c) in row.iter().enumerate() {
                    let mut altered_char_map = char_map.clone();
                    if c == &NodeVal::Hash {
                        altered_char_map[y][x] = NodeVal::Dot;

                        if let Some(val) =
                            find_new_symmetry_values(altered_char_map, original_symmetry_line)
                        {
                            symmetry_sum += val;
                            found_symmetry = true;
                            break;
                        }
                    } else {
                        altered_char_map[y][x] = NodeVal::Hash;

                        if let Some(val) =
                            find_new_symmetry_values(altered_char_map, original_symmetry_line)
                        {
                            symmetry_sum += val;
                            found_symmetry = true;
                            break;
                        }
                    }
                }
                if found_symmetry {
                    break;
                }
            }
        }

        symmetry_sum.to_string()
    }
}

fn find_new_symmetry_values(
    char_map: Vec<Vec<NodeVal>>,
    orig_symmetry_value: (usize, Dir),
) -> Option<usize> {
    let symmetry_values = find_horiz_or_vert_sym_values(&char_map);
    let new_symmetry = symmetry_values
        .iter()
        .find(|symm_val| **symm_val != orig_symmetry_value);

    new_symmetry.map(|(val, dir)| {
        if dir == &Dir::Vertical {
            val * 100
        } else {
            *val
        }
    })
}

fn find_horiz_or_vert_sym_values(char_map: &Vec<Vec<NodeVal>>) -> Vec<(usize, Dir)> {
    let max_x = char_map[0].len() - 1;
    let max_y = char_map.len() - 1;
    let mut translated_char_map = vec![vec![NodeVal::Dot; max_y + 1]; max_x + 1];

    for (y_ix, row) in char_map.iter().enumerate() {
        for (x_ix, c) in row.iter().enumerate() {
            translated_char_map[x_ix][y_ix] = *c;
        }
    }

    let mut vals = vec![];

    // look for vertical symmetry
    let symm_vals = find_symmetries(char_map);
    if !symm_vals.is_empty() {
        vals.append(
            symm_vals
                .iter()
                .map(|symm_val| (*symm_val, Dir::Vertical))
                .collect::<Vec<(usize, Dir)>>()
                .borrow_mut(),
        );
    }

    // look for horizontal symmetry
    let symm_vals = find_symmetries(&translated_char_map);
    if !symm_vals.is_empty() {
        vals.append(
            symm_vals
                .iter()
                .map(|symm_val| (*symm_val, Dir::Horizontal))
                .collect::<Vec<(usize, Dir)>>()
                .borrow_mut(),
        );
    }

    vals
}

fn find_symmetries(char_map: &Vec<Vec<NodeVal>>) -> Vec<usize> {
    let mut symmetries: Vec<usize> = vec![];
    // let max_x = char_map[0].len() - 1;
    let max_y = char_map.len() - 1;
    let mut lines_hash: HashMap<&Vec<NodeVal>, Vec<usize>> = HashMap::new();

    let mut start_of_symmetry = None;

    for (ix, line) in char_map.iter().enumerate() {
        let matching = lines_hash.get_mut(line);

        match matching {
            None => {
                start_of_symmetry = None;
                lines_hash.insert(line, vec![ix]);
            }
            Some(matching) => {
                let mut found_good_match = false;

                for prev_match in matching.iter() {
                    // the line matches a previous one
                    if start_of_symmetry.is_none() {
                        // we aren't in the middle of symmetry - check it's exactly the previous line that matches
                        if ix == prev_match + 1 {
                            // Yes!  start the symmetry
                            start_of_symmetry = Some(ix);
                        } else {
                            // No!  we've matched a line from some time back, but aren't in the middle of symmetry.  It therefore can't be a valid symmetry.  continue the loop
                            continue;
                        }
                    }

                    if (ix as i32 - start_of_symmetry.unwrap() as i32)
                        != (start_of_symmetry.unwrap() as i32 - 1 - *prev_match as i32)
                    {
                        if ix == prev_match + 1 {
                            // restart the symmetry!
                            start_of_symmetry = Some(ix);
                        } else {
                            // No!  we've matched a line from some time back, but aren't in the middle of symmetry.  It therefore can't be a valid symmetry.  continue the loop
                            continue;
                        }
                    }

                    found_good_match = true;

                    if (ix == 0) || (*prev_match == 0) || (ix == max_y) || (*prev_match == max_y) {
                        symmetries.push(start_of_symmetry.unwrap());
                        start_of_symmetry = None;
                    }
                    break;
                }

                if !found_good_match {
                    start_of_symmetry = None;
                }
                matching.push(ix);
            }
        }
    }
    symmetries
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
        let sample_input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(super::Solver13.part1(sample_input.lines()), "405");
    }

    #[test]
    fn part2() {
        let sample_input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(super::Solver13.part2(sample_input.lines()), "400");
    }

    #[test]
    fn single_map() {
        let sample_input = "##..####.
#....#.##
..##..#.#
#.......#
#.##.###.
#....##..
#.##.###.
#.##.###.
#.##.###.
#.##.###.
#....##..";
    }
}
