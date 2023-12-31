use std::str::Lines;

use crate::Solver;
pub struct Solver23;

impl Solver for Solver23 {
    fn day_number(&self) -> u32 {
        23
    }

    fn part1(&self, _input_lines: Lines) -> String {
        "0".to_string()
    }

    fn part2(&self, _input_lines: Lines) -> String {
        "0".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
        let sample_input = "";
        assert_eq!(super::Solver23.part1(sample_input.lines()), "0");
    }

    #[test]
    fn part2() {
        let sample_input = "";
        assert_eq!(super::Solver23.part2(sample_input.lines()), "0");
    }
}
