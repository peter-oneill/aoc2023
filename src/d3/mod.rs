use std::str::Lines;

use crate::Solver;

pub struct Solver3;

impl Solver for Solver3 {
    fn day_number(&self) -> u32 {
        3
    }

    fn part1(&self, input_lines: Lines) -> String {
        for line in input_lines {}
    }

    fn part2(&self, input_lines: Lines) -> String {
        for line in input_lines {}
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
        let sample_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(super::Solver3.part1(sample_input.lines()), "4361");
    }
    #[test]
    fn part2() {
        let sample_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(super::Solver3.part2(sample_input.lines()), "0");
    }
}
