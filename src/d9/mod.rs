use crate::Solver;
use std::str::Lines;

pub struct Solver9;

impl Solver for Solver9 {
    fn day_number(&self) -> u32 {
        9
    }

    fn part1(&self, input_lines: Lines) -> String {
        let number_matcher = regex::Regex::new(r"-?\d+").unwrap();
        let mut sum = 0;
        for line in input_lines {
            let original_numbers: Vec<i64> = number_matcher
                .find_iter(line)
                .map(|m| m.as_str().parse::<i64>().unwrap())
                .collect();
            sum += next_value(original_numbers);
        }

        sum.to_string()
    }

    fn part2(&self, input_lines: Lines) -> String {
        let number_matcher = regex::Regex::new(r"-?\d+").unwrap();
        let mut sum = 0;
        for line in input_lines {
            let mut original_numbers: Vec<i64> = number_matcher
                .find_iter(line)
                .map(|m| m.as_str().parse::<i64>().unwrap())
                .collect();
            original_numbers.reverse();
            sum += next_value(original_numbers);
        }

        sum.to_string()
    }
}

fn next_value(numbers: Vec<i64>) -> i64 {
    let (mut left, remaining) = numbers.split_first().unwrap();
    let mut last: i64 = *left;
    let mut differences: Vec<i64> = Vec::new();
    let mut any_non_zero = false;

    for n in remaining {
        last = *n;
        let diff = n - left;
        left = n;
        differences.push(diff);
        if !any_non_zero && diff != 0 {
            any_non_zero = true;
        }
    }

    if !any_non_zero {
        return last;
    }

    let next = next_value(differences);
    next + last
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
        let sample_input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(super::Solver9.part1(sample_input.lines()), "114");
    }
    #[test]
    fn part2() {
        let sample_input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(super::Solver9.part2(sample_input.lines()), "2");
    }
}
