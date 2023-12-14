use crate::Solver;
use itertools::Itertools;
use std::{collections::HashMap, fmt::Debug, str::Lines, vec};
pub struct Solver13;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Dir {
    Vertical,
    Horizontal,
}

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
enum NodeVal {
    Hash,
    Dot,
}

impl Debug for NodeVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hash => write!(f, "#"),
            Self::Dot => write!(f, "."),
        }
    }
}

impl NodeVal {
    fn from_char(c: char) -> NodeVal {
        match c {
            '#' => NodeVal::Hash,
            '.' => NodeVal::Dot,
            _ => panic!("Invalid node value"),
        }
    }

    fn other_type(&self) -> NodeVal {
        match self {
            Self::Hash => Self::Dot,
            Self::Dot => Self::Hash,
        }
    }
}

impl Solver for Solver13 {
    fn day_number(&self) -> u32 {
        13
    }

    fn part1(&self, input_lines: Lines) -> String {
        let mut symmetry_sum = 0;

        let node_maps: Vec<Vec<Vec<NodeVal>>> = input_lines
            .map(|line| {
                line.chars()
                    .map(NodeVal::from_char)
                    .collect::<Vec<NodeVal>>()
            }) // convert each line to a vector of chars
            .group_by(|line| line.is_empty()) // group the lines into groups of empty and non-empty lines
            .into_iter()
            .filter_map(|(empty, map)| if empty { None } else { Some(map.collect()) }) // drop empty lines
            .collect();

        for node_map in node_maps {
            symmetry_sum += find_only_symmetry_values(&node_map).unwrap();
        }

        symmetry_sum.to_string()
    }

    fn part2(&self, input_lines: Lines) -> String {
        // naive version - just test for changing each nodeacter in each map

        let mut symmetry_sum = 0;

        let node_maps: Vec<Vec<Vec<NodeVal>>> = input_lines
            .map(|line| {
                line.chars()
                    .map(NodeVal::from_char)
                    .collect::<Vec<NodeVal>>()
            }) // convert each line to a vector of chars
            .group_by(|line| line.is_empty()) // group the lines into groups of empty and non-empty lines
            .into_iter()
            .filter_map(|(empty, map)| if empty { None } else { Some(map.collect()) }) // drop empty lines
            .collect();

        for node_map in node_maps {
            let original_symmetry_line = find_horiz_or_vert_sym_values(&node_map).next().unwrap();

            let mut found_symmetry = false;

            for (y, row) in node_map.iter().enumerate() {
                for (x, c) in row.iter().enumerate() {
                    let mut altered_node_map = node_map.clone();
                    altered_node_map[y][x] = c.other_type();

                    if any_matches(&altered_node_map, x, y) {
                        if let Some(val) =
                            find_new_symmetry_values(&altered_node_map, original_symmetry_line)
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

fn any_matches(node_map: &Vec<Vec<NodeVal>>, x: usize, y: usize) -> bool {
    if node_map.iter().filter(|row| **row == node_map[y]).count() > 1 {
        return true;
    }

    let mut translated_node_map = vec![vec![NodeVal::Dot; node_map.len()]; node_map[0].len()];

    for (y_ix, row) in node_map.iter().enumerate() {
        for (x_ix, c) in row.iter().enumerate() {
            translated_node_map[x_ix][y_ix] = *c;
        }
    }

    translated_node_map
        .iter()
        .filter(|row| **row == translated_node_map[x])
        .count()
        > 1
}
fn find_only_symmetry_values(node_map: &Vec<Vec<NodeVal>>) -> Option<usize> {
    find_symmetry_values(node_map, |_| true)
}

fn find_symmetry_values<P>(node_map: &Vec<Vec<NodeVal>>, filter: P) -> Option<usize>
where
    P: FnMut(&(usize, Dir)) -> bool,
{
    find_horiz_or_vert_sym_values(node_map)
        .filter(filter)
        .map(
            |(val, dir)| {
                if dir == Dir::Vertical {
                    val * 100
                } else {
                    val
                }
            },
        )
        .next()
}

fn find_new_symmetry_values(
    node_map: &Vec<Vec<NodeVal>>,
    orig_symmetry_value: (usize, Dir),
) -> Option<usize> {
    let filter = |symm_val: &(usize, Dir)| *symm_val != orig_symmetry_value;
    find_symmetry_values(node_map, filter)
}

fn find_horiz_or_vert_sym_values(
    node_map: &Vec<Vec<NodeVal>>,
) -> impl Iterator<Item = (usize, Dir)> {
    let max_x = node_map[0].len() - 1;
    let max_y = node_map.len() - 1;
    let mut translated_node_map = vec![vec![NodeVal::Dot; max_y + 1]; max_x + 1];

    for (y_ix, row) in node_map.iter().enumerate() {
        for (x_ix, c) in row.iter().enumerate() {
            translated_node_map[x_ix][y_ix] = *c;
        }
    }

    vec![
        find_symmetries(node_map, Dir::Vertical),
        find_symmetries(&translated_node_map, Dir::Horizontal),
    ]
    .into_iter()
    .flatten()
}

fn eq_dist_from_symmetry_line(this_line: i32, other_line: i32, symmetry_start_line: i32) -> bool {
    (this_line - symmetry_start_line) == (symmetry_start_line - 1 - other_line)
}
fn find_symmetries(node_map: &Vec<Vec<NodeVal>>, direction: Dir) -> Vec<(usize, Dir)> {
    let mut symmetries = vec![];
    let max_y = node_map.len() - 1;
    let mut lines_hash: HashMap<&Vec<NodeVal>, Vec<usize>> = HashMap::new();

    let mut start_of_symmetry: Option<usize> = None;

    for (ix, line) in node_map.iter().enumerate() {
        let matching = lines_hash.get_mut(line);

        match matching {
            None => {
                start_of_symmetry = None;
                lines_hash.insert(line, vec![ix]);
            }
            Some(matching) => {
                let mut found_good_match = false;

                for prev_match in matching.iter() {
                    if !start_of_symmetry.is_some_and(|start| {
                        eq_dist_from_symmetry_line(ix as i32, *prev_match as i32, start as i32)
                    }) {
                        // Either this is the first match, or the match was not a mirror image
                        if ix != prev_match + 1 {
                            // not a good starting match
                            continue;
                        }

                        // We'll only get to the here line if there were no other good matches, so restart the symmetry here
                        start_of_symmetry = Some(ix);
                    }

                    found_good_match = true;

                    if (ix == 0) || (*prev_match == 0) || (ix == max_y) || (*prev_match == max_y) {
                        symmetries.push((start_of_symmetry.unwrap(), direction));
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
        super::Solver13.part1(sample_input.lines());
    }
}
