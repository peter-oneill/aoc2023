use std::{collections::HashMap, str::Lines};

use crate::Solver;

pub struct Solver8;

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    quality: i32,
    bet: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.quality.cmp(&other.quality)
    }
}

impl Solver for Solver8 {
    fn day_number(&self) -> u32 {
        8
    }

    fn part1(&self, mut input_lines: Lines) -> String {
        let directions = input_lines.next().unwrap().chars().collect::<Vec<char>>();
        let _ = input_lines.next();

        let mut network: HashMap<&str, (&str, &str)> = HashMap::new();
        for line in input_lines {
            network.insert(&line[0..3], (&line[7..10], &line[12..15]));
        }

        let mut steps = 0;
        let mut current = "AAA";
        let target = "ZZZ";

        loop {
            for c in &directions {
                current = match c {
                    'L' => {
                        // println!("current: {}", current);
                        // println!("choosing 0");
                        network[current].0
                    }

                    'R' => {
                        // println!("current: {}", current);
                        // println!("choosing 1");
                        network[current].1
                    }

                    _ => panic!("Unknown direction"),
                };
                steps += 1;
                if current == target {
                    return steps.to_string();
                }
            }
        }
    }

    fn part2(&self, mut input_lines: Lines) -> String {
        let mut steps = 0;
        let directions = input_lines.next().unwrap().chars().collect::<Vec<char>>();
        let _ = input_lines.next();

        let mut current_nodes: Vec<&str> = Vec::new();
        let mut network: HashMap<&str, (&str, &str)> = HashMap::new();
        // println!("steps: {}", steps);
        for line in input_lines {
            if line.chars().nth(2).unwrap() == 'A' {
                current_nodes.push(&line[0..3]);
            }
            network.insert(&line[0..3], (&line[7..10], &line[12..15]));
        }
        // println!("steps: {}", steps);

        let num_locations = current_nodes.len();
        // println!("steps: {}", steps);

        loop {
            for c in &directions {
                // println!("steps: {}", steps);
                let mut num_to_complete = num_locations;
                for n in current_nodes.iter_mut() {
                    *n = match c {
                        'L' => {
                            // println!("current: {}", n);
                            // println!("choosing 0");
                            network[n].0
                        }
                        'R' => {
                            // println!("current: {}", n);
                            // println!("choosing 1");
                            network[n].1
                        }
                        _ => panic!("Unknown direction"),
                    };
                    // println!("n[2]: {}", n.chars().nth(2).unwrap());
                    if n.chars().nth(2).unwrap() == 'Z' {
                        num_to_complete -= 1;
                        // println!("num_to_complete: {}", num_to_complete);
                    }
                }
                steps += 1;
                if num_to_complete == 0 {
                    return steps.to_string();
                }
                if steps % 1000000 == 0 {
                println!("{:?} {}", current_nodes, steps);
                if steps % 10000000 == 0 {
                    return "10M iterations - quitting".to_string()
                }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
        let sample_input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(super::Solver8.part1(sample_input.lines()), "2");
    }
    #[test]
    fn part2() {
        let sample_input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(super::Solver8.part2(sample_input.lines()), "6");
    }
}
