use std::str::Lines;

use crate::Solver;

pub struct Solver4;

impl Solver for Solver4 {
    fn day_number(&self) -> u32 {
        4
    }

    fn part1(&self, input_lines: Lines) -> String {
        let mut total = 0;
        let number_matcher = regex::Regex::new(r"\d+").unwrap();

        for line in input_lines {
            let mut numbers = line.split(':').last().unwrap().split('|');

            let winning_numbers = numbers.next().unwrap();
            let winning_numbers = number_matcher
                .find_iter(winning_numbers)
                .map(|n| n.as_str());

            let card_numbers = numbers.next().unwrap();
            let card_numbers: Vec<&str> = number_matcher
                .find_iter(card_numbers)
                .map(|n| n.as_str())
                .collect();

            let number_of_matches: u32 =
                winning_numbers.filter(|n| card_numbers.contains(n)).count() as u32;

            if number_of_matches == 0 {
                continue;
            }

            total += u32::pow(2, number_of_matches - 1);
        }

        total.to_string()
    }

    fn part2(&self, input_lines: Lines) -> String {
        let mut total = 0;

        total += 1;

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
        let sample_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(super::Solver4.part1(sample_input.lines()), "13");
    }
    #[test]
    fn part2() {
        //         let sample_input = "two1nine
        // eightwothree
        // abcone2threexyz
        // xtwone3four
        // 4nineeightseven2
        // zoneight234
        // 7pqrstsixteen";
        //         assert_eq!(super::Solver4.part2(sample_input.lines()), "281");
    }
}
