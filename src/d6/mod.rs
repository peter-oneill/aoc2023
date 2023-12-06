use std::str::Lines;

use crate::Solver;

pub struct Solver6;

impl Solver for Solver6 {
    fn day_number(&self) -> u32 {
        6
    }

    fn part1(&self, mut input_lines: Lines) -> String {
        let number_matcher = regex::Regex::new(r"\d+").unwrap();

        let times: Vec<f64> = number_matcher
            .find_iter(input_lines.next().unwrap().split(": ").last().unwrap())
            .map(|n| n.as_str().parse::<i64>().unwrap() as f64)
            .collect();

        let dists: Vec<f64> = number_matcher
            .find_iter(input_lines.next().unwrap().split(": ").last().unwrap())
            .map(|n| n.as_str().parse::<i64>().unwrap() as f64)
            .collect();

        let mut bounds: Vec<f64> = Vec::new();

        for ix in 0..dists.len() {
            let t = times[ix];
            let d = dists[ix];
            let disc = t * t - (4_f64 * d);
            let lower_bound = (t - disc.sqrt()) / 2_f64;
            let upper_bound = (t + disc.sqrt()) / 2_f64;
            bounds.push(
                (upper_bound - 0.00000000001).floor() - (lower_bound + 0.0000000000001).ceil()
                    + 1_f64,
            );
        }

        let mut sum = 1_f64;
        for b in bounds {
            sum *= b;
        }
        sum.to_string()
    }

    fn part2(&self, mut input_lines: Lines) -> String {
        let mut time: i64 = 0;

        for d in input_lines.next().unwrap().matches(char::is_numeric) {
            time = time * 10 + d.parse::<i64>().unwrap()
        }

        let mut dist: i64 = 0;

        for d in input_lines.next().unwrap().matches(char::is_numeric) {
            dist = dist * 10 + d.parse::<i64>().unwrap()
        }

        let time = time as f64;
        let dist = dist as f64;

        let disc = time * time - (4_f64 * dist);
        let lower_bound = (time - disc.sqrt()) / 2_f64;
        let upper_bound = (time + disc.sqrt()) / 2_f64;
        let val =
            (upper_bound - 0.00000000001).floor() - (lower_bound + 0.0000000000001).ceil() + 1_f64;

        val.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
        let sample_input = "Time: 7 15 30
Distance: 9 40 200";
        assert_eq!(super::Solver6.part1(sample_input.lines()), "288");
    }
    #[test]
    fn part2() {
        let sample_input = "Time: 7 15 30
Distance: 9 40 200";
        assert_eq!(super::Solver6.part2(sample_input.lines()), "71503");
    }
}
