use crate::Solver;
use std::{str::Lines, vec};

pub struct Solver13;

impl Solver for Solver13 {
    fn day_number(&self) -> u32 {
        13
    }

    fn part1(&self, input_lines: Lines) -> String {
        let mut char_maps: Vec<Vec<Vec<char>>> = vec![];
        let mut translated_char_maps: Vec<Vec<Vec<char>>> = vec![];

        let mut char_map: Vec<Vec<char>> = vec![];
        let mut translated_char_map: Vec<Vec<char>> = vec![];

        let maps: Vec<Lines> = input_lines.split(|line| line.is_empty()).collect();

        char_maps = maps
            .iter()
            .map(|map| map.map(|line| line.chars().collect()).collect())
            .collect();

        for char_map in char_maps {
            let max_x = char_map[0].len();
            let max_y = char_map.len();

            let mut translated_char_map = vec![vec!['.'; max_y]; max_x];

            for y in 0..max_y {
                for x in 0..max_x {
                    translated_char_map[x][y] = char_map[y][x];
                }
            }

            // look for vertical symmetry
            let mut lines_hash: HashMap<Vec<char>, usize> = HashMap::new();
            for line in char_map.iter().enumerate() {
                let count = lines_hash.entry(line.clone()).or_insert(0);
                *count += 1;
            }
        }

        for line in input_lines {}

        "0".to_string()
    }

    fn part2(&self, input_lines: Lines) -> String {
        solve_with_expansion_factor(input_lines, 1000000)
    }
}

fn solve_with_expansion_factor(input_lines: Lines, factor: i64) -> String {
    let cloned_lines: Vec<&str> = input_lines.clone().collect();
    let num_columns = cloned_lines[0].chars().count();
    let num_rows = cloned_lines.len();

    let mut galaxies_on_single_row: Vec<char> = vec!['.'; num_columns];
    let mut galaxies_on_single_column: Vec<char> = vec!['.'; num_rows];

    let mut galaxies: Vec<(i64, i64)> = Vec::new();

    // 1:  for each found galaxy:
    // - add to the list of galaxies
    // - mark the row and column as having a galaxy
    for (y, row) in input_lines.enumerate() {
        for (x, c) in row.char_indices() {
            if c == '#' {
                galaxies.push((x.try_into().unwrap(), y.try_into().unwrap()));
                galaxies_on_single_row[x] = '#';
                galaxies_on_single_column[y] = '#';
            }
        }
    }

    // 2: get a lists of row/column indices which have no galaxies
    let empty_column_ixs = galaxies_on_single_row
        .iter()
        .enumerate()
        .filter(|(_, c)| c == &&'.')
        .map(|(ix, _)| ix.try_into().unwrap())
        .collect::<Vec<i64>>();
    let empty_row_ixs = galaxies_on_single_column
        .iter()
        .enumerate()
        .filter(|(_, c)| c == &&'.')
        .map(|(ix, _)| ix.try_into().unwrap())
        .collect::<Vec<i64>>();

    let mut dist_sum = 0;

    // 3: for each pair of galaxies, calculate the distance between them
    for left_ix in 0..galaxies.len() {
        let l_gal = galaxies[left_ix];
        for r_gal in galaxies[left_ix + 1..].iter() {
            // The offset in each direction is the number of empty rows/columns indices which we've passed
            let left_offset_x: i64 = (factor - 1)
                * TryInto::<i64>::try_into(
                    empty_column_ixs.iter().filter(|ix| ix < &&l_gal.0).count(),
                )
                .unwrap();
            let right_offset_x: i64 = (factor - 1)
                * TryInto::<i64>::try_into(
                    empty_column_ixs.iter().filter(|ix| ix < &&r_gal.0).count(),
                )
                .unwrap();
            let left_offset_y: i64 = (factor - 1)
                * TryInto::<i64>::try_into(
                    empty_row_ixs.iter().filter(|ix| ix < &&l_gal.1).count(),
                )
                .unwrap();
            let right_offset_y: i64 = (factor - 1)
                * TryInto::<i64>::try_into(
                    empty_row_ixs.iter().filter(|ix| ix < &&r_gal.1).count(),
                )
                .unwrap();
            let x_dist = ((l_gal.0 + left_offset_x) - (r_gal.0 + right_offset_x)).abs();
            let y_dist = ((l_gal.1 + left_offset_y) - (r_gal.1 + right_offset_y)).abs();
            dist_sum += x_dist + y_dist;
        }
    }

    dist_sum.to_string()
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
        assert_eq!(super::Solver11.part1(sample_input.lines()), "405");
    }

    #[test]
    fn part2() {
        let sample_input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(
            super::solve_with_expansion_factor(sample_input.lines(), 10),
            "1030"
        );
        assert_eq!(
            super::solve_with_expansion_factor(sample_input.lines(), 100),
            "8410"
        );
    }
}
