use std::str::Lines;

use crate::Solver;
pub struct Solver21;

impl Solver for Solver21 {
    fn day_number(&self) -> u32 {
        21
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
        assert_eq!(super::Solver21.part1(sample_input.lines()), "0");
    }

    #[test]
    fn part2() {
        let sample_input = "";
        assert_eq!(super::Solver21.part2(sample_input.lines()), "0");
    }
}
