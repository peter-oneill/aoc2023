use std::str::Lines;

use crate::Solver;

pub struct Solver2;

impl Solver for Solver2 {
    fn day_number(&self) -> u32 {
        2
    }

    fn part1(input_lines: Lines) -> String {
        for line in input_lines {
            Game 1: 14 green, 8 blue, 9 red; 5 blue, 4 green, 2 red;
            let matcher = regex::Regex::new(r"Game \d+: (((\d+ green), )?((\d+ blue), )?(\d+ red)?)+;").unwrap();
        //     let mut digits = Vec::<u32>::new();
        //     let chars = line.char_indices();
        //     for my_char in chars {
        //         if my_char.1.is_numeric() {
        //             digits.push(my_char.1.to_digit(10).unwrap());
        //         } else if let Some(digit) = find_digit_word(&line[my_char.0..]) {
        //             digits.push(digit);
        //         }
        //     }

        //     if digits.len() == 0 {
        //         panic!("{}", line);
        //     }
        //     total += digits[0] * 10 + digits[digits.len() - 1];
        // }

        total.to_string()
    }

    fn part2(input_lines: Lines) -> String {
       
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
        let sample_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(super::Solver1::part1(sample_input.lines()), "8");
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
