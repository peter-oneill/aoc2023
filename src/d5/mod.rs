use std::str::Lines;

use crate::Solver;

pub struct Solver5;

struct map

impl Solver for Solver5 {
    fn day_number(&self) -> u32 {
        5
    }

    fn part1(&self, input_lines: Lines) -> String {
        let maps = Vec<Vec<
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

    fn part2(&self, input_lines: Lines) -> String {
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

#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
        let sample_input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(super::Solver5.part1(sample_input.lines()), "13");
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
        assert_eq!(super::Solver5.part2(sample_input.lines()), "281");
    }
}
