use std::{
    cmp::{max, min},
    str::Lines,
};

use crate::Solver;

pub struct Solver3;

#[derive(Debug)]
struct Number {
    value: u32,
    start: usize,
    end: usize,
}

impl Number {
    fn new(start: usize, d: u32) -> Number {
        Number {
            value: d,
            start,
            end: start,
        }
    }
    fn append_digit(self, d: u32) -> Self {
        Number {
            value: self.value * 10 + d,
            start: self.start,
            end: self.end + 1,
        }
    }
}

enum LineState {
    Number(Number),
    Other,
}

impl LineState {
    fn append(self, ix: usize, c: char) -> (Self, Option<Number>) {
        let c_type = CharType::from_char(&c);

        match self {
            Self::Number(n) => match c_type {
                CharType::Numeric(d) => (Self::Number(n.append_digit(d)), None),
                CharType::Dot | CharType::Symbol => (Self::Other, Some(n)),
            },
            Self::Other => match c_type {
                CharType::Numeric(d) => (Self::Number(Number::new(ix, d)), None),
                CharType::Dot | CharType::Symbol => (Self::Other, None),
            },
        }
    }
}

#[derive(PartialEq, Debug)]
enum CharType {
    Numeric(u32),
    Symbol,
    Dot,
}

impl CharType {
    fn from_char(c: &char) -> CharType {
        match c {
            '0'..='9' => CharType::Numeric(c.to_digit(10).unwrap()),
            '.' => CharType::Dot,
            _ => CharType::Symbol,
        }
    }
}

impl Solver for Solver3 {
    fn day_number(&self) -> u32 {
        3
    }

    fn part1(&self, input_lines: Lines) -> String {
        let mut all_numbers: Vec<Vec<Number>> = Vec::new();
        let mut all_char_types: Vec<Vec<CharType>> = Vec::new();

        for line in input_lines {
            let mut line_numbers: Vec<Number> = Vec::new();
            let mut line_char_types: Vec<CharType> = Vec::new();
            let mut line_state = LineState::Other;

            for (ix, c) in line.char_indices() {
                let (l, completed_number) = line_state.append(ix, c);
                line_state = l;

                if let Some(n) = completed_number {
                    line_numbers.push(n)
                }

                line_char_types.push(CharType::from_char(&c));
            }

            // Handle the case where a number finishes at the end of a line
            if let LineState::Number(n) = line_state {
                line_numbers.push(n);
            }

            all_numbers.push(line_numbers);
            all_char_types.push(line_char_types);
        }

        let mut sum = 0;
        let num_lines = all_char_types.len();
        let line_length = all_char_types[0].len();

        for (line_num, numbers) in all_numbers.iter().enumerate() {
            let start_line_ix = max(line_num, 1) - 1;
            let end_line_ix = min(line_num + 1, num_lines - 1);

            for num in numbers {
                for line_ix in start_line_ix..=end_line_ix {
                    if find_touching_symbols(num, &all_char_types[line_ix], line_length) {
                        sum += num.value;
                        continue;
                    }
                }
            }
        }

        sum.to_string()
    }

    fn part2(&self, _input_lines: Lines) -> String {
        // for line in input_lines {}
        "".to_string()
    }
}

fn find_touching_symbols(num: &Number, symbols: &Vec<CharType>, line_length: usize) -> bool {
    let start_ix = max(num.start, 1) - 1;
    let end_ix = min(num.end + 1, line_length - 1);

    for ix in start_ix..=end_ix {
        if symbols[ix] == CharType::Symbol {
            return true;
        }
    }

    false
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
        assert_eq!(super::Solver3.part2(sample_input.lines()), "");
    }
}
