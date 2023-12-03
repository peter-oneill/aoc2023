use std::{cmp::max, str::Lines};

use crate::Solver;

pub struct Solver2;

impl Solver for Solver2 {
    fn day_number(&self) -> u32 {
        2
    }

    fn part1(&self, input_lines: Lines) -> String {
        let mut valid_game_sum = 0;
        let matcher = regex::Regex::new(r"(\d+) (green|blue|red)").unwrap();

        for line in input_lines {
            let parts: Vec<&str> = line.split(":").collect();
            let colors = matcher.captures_iter(parts[1]);
            let mut maxes = [0, 0, 0];

            for c in colors {
                let count = c[1].parse::<u32>().unwrap();
                match c[2].as_ref() {
                    "red" => maxes[0] = max(count, maxes[0]),
                    "green" => maxes[1] = max(count, maxes[1]),
                    "blue" => maxes[2] = max(count, maxes[2]),
                    _ => panic!("Unknown color"),
                }
            }

            if maxes[0] <= 12 && maxes[1] <= 13 && maxes[2] <= 14 {
                valid_game_sum += parts[0][5..].parse::<u32>().unwrap();
            }
        }
        valid_game_sum.to_string()
    }

    fn part2(&self, input_lines: Lines) -> String {
        let mut power_sum = 0;
        let matcher = regex::Regex::new(r"(\d+) (green|blue|red)").unwrap();

        for line in input_lines {
            let parts: Vec<&str> = line.split(":").collect();
            let colors = matcher.captures_iter(parts[1]);
            let mut maxes = [0, 0, 0];

            for c in colors {
                let count = c[1].parse::<u32>().unwrap();
                match c[2].as_ref() {
                    "red" => maxes[0] = max(count, maxes[0]),
                    "green" => maxes[1] = max(count, maxes[1]),
                    "blue" => maxes[2] = max(count, maxes[2]),
                    _ => panic!("Unknown color"),
                }
            }

            power_sum += maxes[0] * maxes[1] * maxes[2];
        }
        power_sum.to_string()
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
        assert_eq!(super::Solver2.part1(sample_input.lines()), "8");
    }
    #[test]
    fn part2() {
        let sample_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(super::Solver2.part2(sample_input.lines()), "2286");
    }
}
