use crate::Solver;
use std::{str::Lines, vec};

pub struct Solver11;

impl Solver for Solver11 {
    fn day_number(&self) -> u32 {
        11
    }

    fn part1(&self, input_lines: Lines) -> String {
        let cloned_lines: Vec<&str> = input_lines.clone().collect();
        let num_columns = cloned_lines[0].chars().count();
        let num_rows = cloned_lines.len();

        let mut galaxies_on_single_row: Vec<char> = vec!['.'; num_columns];
        let mut galaxies_on_single_column: Vec<char> = vec!['.'; num_rows];

        let mut galaxies: Vec<(i64, i64)> = Vec::new();

        for (y, row) in input_lines.enumerate() {
            for (x, c) in row.char_indices() {
                if c == '#' {
                    galaxies.push((x.try_into().unwrap(), y.try_into().unwrap()));
                    galaxies_on_single_row[x] = '#';
                    galaxies_on_single_column[y] = '#';
                }
            }
        }

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
        // 0, 5, 6, 10

        // let mut x_offset = 0;
        // let mut y_offset = 0;

        let mut dist_sum = 0;

        for left_ix in 0..galaxies.len() {
            let left_g = galaxies[left_ix];
            for right_g in galaxies[left_ix + 1..].iter() {
                if (left_g.0 == right_g.0) && (left_g.1 == right_g.1) {
                    continue;
                }
                let left_offset_x: i64 = empty_column_ixs
                    .iter()
                    .filter(|ix| ix < &&left_g.0)
                    .count()
                    .try_into()
                    .unwrap();
                let right_offset_x: i64 = empty_column_ixs
                    .iter()
                    .filter(|ix| ix < &&right_g.0)
                    .count()
                    .try_into()
                    .unwrap();
                let left_offset_y: i64 = empty_row_ixs
                    .iter()
                    .filter(|ix| ix < &&left_g.1)
                    .count()
                    .try_into()
                    .unwrap();
                let right_offset_y: i64 = empty_row_ixs
                    .iter()
                    .filter(|ix| ix < &&right_g.1)
                    .count()
                    .try_into()
                    .unwrap();
                let x_dist = ((left_g.0 + left_offset_x) - (right_g.0 + right_offset_x)).abs();
                let y_dist = ((left_g.1 + left_offset_y) - (right_g.1 + right_offset_y)).abs();
                dist_sum += x_dist + y_dist;
            }
        }

        dist_sum.to_string()
    }

    fn part2(&self, input_lines: Lines) -> String {
        part_2_with_factor(input_lines, 1000000)
    }
}

fn part_2_with_factor(input_lines: Lines, factor: i64) -> String {
    let cloned_lines: Vec<&str> = input_lines.clone().collect();
    let num_columns = cloned_lines[0].chars().count();
    let num_rows = cloned_lines.len();

    let mut galaxies_on_single_row: Vec<char> = vec!['.'; num_columns];
    let mut galaxies_on_single_column: Vec<char> = vec!['.'; num_rows];

    let mut galaxies: Vec<(i64, i64)> = Vec::new();

    for (y, row) in input_lines.enumerate() {
        for (x, c) in row.char_indices() {
            if c == '#' {
                galaxies.push((x.try_into().unwrap(), y.try_into().unwrap()));
                galaxies_on_single_row[x] = '#';
                galaxies_on_single_column[y] = '#';
            }
        }
    }

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

    for left_ix in 0..galaxies.len() {
        let left_g = galaxies[left_ix];
        for right_g in galaxies[left_ix + 1..].iter() {
            if (left_g.0 == right_g.0) && (left_g.1 == right_g.1) {
                continue;
            }
            let left_offset_x: i64 = (factor - 1)
                * TryInto::<i64>::try_into(
                    empty_column_ixs.iter().filter(|ix| ix < &&left_g.0).count(),
                )
                .unwrap();
            let right_offset_x: i64 = (factor - 1)
                * TryInto::<i64>::try_into(
                    empty_column_ixs
                        .iter()
                        .filter(|ix| ix < &&right_g.0)
                        .count(),
                )
                .unwrap();
            let left_offset_y: i64 = (factor - 1)
                * TryInto::<i64>::try_into(
                    empty_row_ixs.iter().filter(|ix| ix < &&left_g.1).count(),
                )
                .unwrap();
            let right_offset_y: i64 = (factor - 1)
                * TryInto::<i64>::try_into(
                    empty_row_ixs.iter().filter(|ix| ix < &&right_g.1).count(),
                )
                .unwrap();
            let x_dist = ((left_g.0 + left_offset_x) - (right_g.0 + right_offset_x)).abs();
            let y_dist = ((left_g.1 + left_offset_y) - (right_g.1 + right_offset_y)).abs();
            dist_sum += x_dist + y_dist;
        }
    }

    dist_sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::{d11::part_2_with_factor, Solver};
    #[test]
    fn part1() {
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
        assert_eq!(super::Solver11.part1(sample_input.lines()), "374");
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
        assert_eq!(part_2_with_factor(sample_input.lines(), 10), "1030");
        assert_eq!(part_2_with_factor(sample_input.lines(), 100), "8410");
    }
}
