use std::str::Lines;

use crate::Solver;

pub struct Solver1;

impl Solver for Solver1 {
    fn day_number(&self) -> u32 {
        1
    }

    fn part1(input_lines: Lines) -> String {
        let mut total = 0;

        for line in input_lines {
            let digits: Vec<u32> = line
                .matches(char::is_numeric)
                .map(|s| s.parse::<u32>().unwrap())
                .collect();

            if digits.len() == 0 {
                panic!("{}", line);
            }
            total += digits[0] * 10 + digits[digits.len() - 1];
        }

        total.to_string()
    }

    fn part2(input_lines: Lines) -> String {
        let mut total = 0;

        for line in input_lines {
            let mut digits = Vec::<u32>::new();
            let chars = line.char_indices();
            for my_char in chars {
                if my_char.1.is_numeric() {
                    digits.push(my_char.1.to_digit(10).unwrap());
                } else if let Some(digit) = find_digit_word(&line[my_char.0..]) {
                    digits.push(digit);
                }
            }

            if digits.len() == 0 {
                panic!("{}", line);
            }
            total += digits[0] * 10 + digits[digits.len() - 1];
        }

        total.to_string()
    }
}

fn find_digit_word(word: &str) -> Option<u32> {
    let len = word.len();

    if len < 3 {
        return None;
    }
    match &word[0..3] {
        "one" => return Some(1),
        "two" => return Some(2),
        "six" => return Some(6),
        _ => (),
    }

    if len < 4 {
        return None;
    }
    match &word[0..4] {
        "zero" => return Some(0),
        "four" => return Some(4),
        "five" => return Some(5),
        "nine" => return Some(9),
        _ => (),
    }

    if len < 5 {
        return None;
    }
    return match &word[0..5] {
        "three" => Some(3),
        "seven" => Some(7),
        "eight" => Some(8),
        _ => None,
    };
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
        let sample_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(super::Solver1::part1(sample_input.lines()), "142");
    }
    #[test]
    fn part2() {
        let sample_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(super::Solver1::part2(sample_input.lines()), "281");
    }
}
