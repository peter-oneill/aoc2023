use num::{integer, traits::AsPrimitive};
use std::{cell::RefCell, collections::HashMap, fmt::Display, str::Lines, sync::Arc};

use crate::Solver;

pub struct Solver8;

#[derive(PartialEq, Eq, Debug)]
struct Node<'a> {
    orig_string: &'a str,
    ends_with_z: bool,
    left: Option<Arc<RefCell<Node<'a>>>>,
    right: Option<Arc<RefCell<Node<'a>>>>,
}

impl<'a> Node<'a> {
    fn from(line: &'a str) -> Self {
        Node {
            orig_string: line,
            ends_with_z: line.chars().nth(2).unwrap() == 'Z',
            left: None,
            right: None,
        }
    }
}

impl<'a> Display for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node: [
orig_string: {},
ends_with_z: {},
left: {},
right: {}
]",
            self.orig_string,
            self.ends_with_z,
            self.left.as_ref().unwrap().borrow().orig_string,
            self.right.as_ref().unwrap().borrow().orig_string
        )
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
                    'L' => network[current].0,
                    'R' => network[current].1,
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
        let mut steps: i64 = 0;
        let directions = input_lines.next().unwrap().chars().collect::<Vec<char>>();
        let _ = input_lines.next();

        let mut current_nodes: Vec<Arc<RefCell<Node>>> = Vec::new();
        let mut all_nodes: HashMap<&str, Arc<RefCell<Node>>> = HashMap::new();
        for line in input_lines {
            let node = Node::from(line);
            let key = &node.orig_string[0..3];
            all_nodes.insert(key, Arc::new(RefCell::new(node)));
        }

        for node in all_nodes.values() {
            let mut n = node.borrow_mut();

            let left = all_nodes.get(&n.orig_string[7..10]).unwrap();
            let wrapped_left = Some(left.clone());
            n.left = wrapped_left;

            let right = all_nodes.get(&n.orig_string[12..15]).unwrap();
            let wrapped_right = Some(right.clone());
            n.right = wrapped_right;
        }

        for (_, n) in all_nodes {
            if n.borrow().orig_string.chars().nth(2).unwrap() == 'A' {
                current_nodes.push(n.clone());
            }
        }

        let num_locations = current_nodes.len();
        let mut found_locs: HashMap<usize, i64> = HashMap::new();
        loop {
            for c in &directions {
                steps += 1;
                let mut num_to_complete = num_locations;

                for l in current_nodes.iter() {}
                for (ix, n) in current_nodes.iter_mut().enumerate() {
                    *n = match c {
                        'L' => n.borrow().left.as_ref().unwrap().clone(),
                        'R' => n.borrow().right.as_ref().unwrap().clone(),
                        _ => panic!("Unknown direction"),
                    };
                    if n.borrow().ends_with_z {
                        if !found_locs.contains_key(&ix) {
                            found_locs.insert(ix, steps);
                            if found_locs.len() == num_locations {
                                let mut vals = found_locs.values();
                                let first = vals.next().unwrap().clone();
                                let mut lcm = first;
                                for s in vals {
                                    lcm = integer::lcm(lcm, s.clone());
                                }
                                return lcm.to_string();
                            }
                        }

                        num_to_complete -= 1;
                    }
                }
                if num_to_complete == 0 {
                    return steps.to_string();
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
