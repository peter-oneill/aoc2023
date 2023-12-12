use crate::Solver;
use core::panic;
use std::{collections::HashMap, str::Lines};

pub struct Solver12;

static mut indent: usize = 0;

impl Solver for Solver12 {
    fn day_number(&self) -> u32 {
        12
    }

    fn part1(&self, input_lines: Lines) -> String {
        // naive brute force will take too long.  Instead need some state to limit the search space
        let mut sum = 0;
        let number_matcher = regex::Regex::new(r"\d+").unwrap();

        for line in input_lines {
            let (spring_row, numbers) = line.split_once(" ").unwrap();
            //    let spring_row = format!(
            //        "{}?{}?{}?{}?{}",
            //        springs, springs, springs, springs, springs
            //    );
            //    let numbers = format!(
            //        "{},{},{},{},{}",
            //        numbers, numbers, numbers, numbers, numbers
            //    );

            let group_sizes: Vec<usize> = number_matcher
                .find_iter(&numbers)
                .map(|n| n.as_str().parse::<usize>().unwrap())
                .collect();

            let mut match_cache = MatchCache {
                matches: HashMap::new(),
            };

            let matches_this_row = match_cache.can_start_group_at_or_after_position(
                &spring_row.chars().collect::<Vec<char>>(),
                &group_sizes,
            );
            // unsafe { println!("{n:>width$}matches_this_row: {}", matches_this_row, width=indent, n=0); }
            sum += matches_this_row;
            // break;
        }
        sum.to_string()
    }

    fn part2(&self, input_lines: Lines) -> String {
        // naive brute force will take too long.  Instead need some state to limit the search space
        let mut sum: u64 = 0;
        let number_matcher = regex::Regex::new(r"\d+").unwrap();

        for (ix, line) in input_lines.enumerate() {
            let (springs, numbers) = line.split_once(" ").unwrap();
            let spring_row = format!(
                "{}?{}?{}?{}?{}",
                springs, springs, springs, springs, springs
            );
            let numbers = format!(
                "{},{},{},{},{}",
                numbers, numbers, numbers, numbers, numbers
            );

            let group_sizes: Vec<usize> = number_matcher
                .find_iter(&numbers)
                .map(|n| n.as_str().parse::<usize>().unwrap())
                .collect();

            let mut match_cache = MatchCache {
                matches: HashMap::new(),
            };

            let matches_this_row = match_cache.can_start_group_at_or_after_position(
                &spring_row.chars().collect::<Vec<char>>(),
                &group_sizes,
            );
            // unsafe { println!("{n:>width$}row {} matches_this_row: {}", ix, matches_this_row, width=indent, n=0); }
            sum += matches_this_row as u64;
            // break;
        }
        sum.to_string()
    }
}

struct MatchCache {
    matches: HashMap<(usize, usize), u64>,
}

impl MatchCache {
    fn can_start_group_at_or_after_position(
        &mut self,
        remaining_springs: &[char],
        remaining_groups: &[usize],
    ) -> u64 {
        // currently this recurses even when we've previously verified the same end of a row can match
        // we could us a cache instead?
        if let Some(number) = self
            .matches
            .get(&(remaining_springs.len(), remaining_groups.len()))
        {
            return *number;
        }
        unsafe {
            indent += 2;
        }
        unsafe {
            print!("{n:>width$}1", width = indent, n = 0);
        }
        for c in remaining_springs {
            print!("{}", c);
        }
        println!(", {:?} ", remaining_groups);
        let remaining_broken_spring_count = remaining_springs.iter().filter(|c| **c == '#').count();

        // There are no more groups to match
        if remaining_groups.len() == 0 {
            // If there are no more broken springs, return one match variant
            if remaining_broken_spring_count == 0 {
                unsafe {
                    println!("{n:>width$}2 no more broken springs", width = indent, n = 0);
                }
                unsafe {
                    indent -= 2;
                }
                self.matches
                    .insert((remaining_springs.len(), remaining_groups.len()), 1);
                return 1;
            } else {
                // There are more broken springs - whoops!  can't match
                unsafe {
                    println!(
                        "{n:>width$}3 more broken springs - whoops can't match",
                        width = indent,
                        n = 0
                    );
                }

                unsafe {
                    indent -= 2;
                }
                self.matches
                    .insert((remaining_springs.len(), remaining_groups.len()), 0);
                return 0;
            }
        }

        // There's not enough space to match just the next group
        if remaining_springs.len() < remaining_groups[0] {
            unsafe {
                println!(
                    "{n:>width$}4 not enough space to match just the next group",
                    width = indent,
                    n = 0
                );
            }

            unsafe {
                indent -= 2;
            }
            self.matches
                .insert((remaining_springs.len(), remaining_groups.len()), 0);
            return 0;
        }

        // Do some checks to shortcut recursion when it's bound to fail

        // There are too many broken springs in the row, to match the remaining groups
        if remaining_broken_spring_count > remaining_groups.iter().sum() {
            unsafe {
                println!("{n:>width$}5 too many broken in row", width = indent, n = 0);
            }

            unsafe {
                indent -= 2;
            }
            self.matches
                .insert((remaining_springs.len(), remaining_groups.len()), 0);
            return 0;
        }

        // There are more groups to match, but not enough springs (of any type) to match them
        if remaining_springs.len() < remaining_groups.iter().map(|n| n + 1).sum::<usize>() - 1 {
            // unsafe { println!( }
            //     "not enough springs of any type to match {} {}",
            //     remaining_springs.len(),
            //     remaining_groups.iter().map(|n| n + 1).sum::<usize>()
            // );
            unsafe {
                println!(
                    "{n:>width$}6 not enough springs of any type to match",
                    width = indent,
                    n = 0
                );
            }
            unsafe {
                indent -= 2;
            }
            self.matches
                .insert((remaining_springs.len(), remaining_groups.len()), 0);
            return 0;
        }

        // There are more groups to match, but not enough springs of the right type to match them
        if remaining_springs.iter().filter(|c| **c != '.').count() < remaining_groups.iter().sum() {
            unsafe {
                println!(
                    "{n:>width$}7 not enough springs of the right type to match",
                    width = indent,
                    n = 0
                );
            }
            unsafe {
                indent -= 2;
            }
            self.matches
                .insert((remaining_springs.len(), remaining_groups.len()), 0);
            return 0;
        }

        let group_size = remaining_groups[0];

        match remaining_springs[0] {
            '.' => {
                // Can't start exactly here - can we start at the next position?

                unsafe {
                    println!(
                    "{n:>width$}8 can't start exactly here - can we start at the next position?",
                    width = indent,
                    n = 0
                );
                }
                let val = self.can_start_group_at_or_after_position(
                    &remaining_springs[1..],
                    remaining_groups,
                );
                unsafe {
                    indent -= 2;
                }
                self.matches
                    .insert((remaining_springs.len(), remaining_groups.len()), val);
                return val;
            }
            '#' => {
                // must start a group here if we can
                if remaining_springs.len() == group_size {
                    unsafe {
                        println!(
                            "{n:>width$}9 exact size match - no more groups to check",
                            width = indent,
                            n = 0
                        );
                    }
                    // Exact size match - no more groups to check
                    unsafe {
                        indent -= 2;
                    }
                    self.matches
                        .insert((remaining_springs.len(), remaining_groups.len()), 1);
                    return 1;
                }

                if remaining_springs[0..group_size].iter().any(|c| *c == '.') {
                    // Working springs in the way
                    unsafe {
                        println!(
                            "{n:>width$}10 working springs in the way",
                            width = indent,
                            n = 0
                        );
                    }
                    unsafe {
                        indent -= 2;
                    }
                    self.matches
                        .insert((remaining_springs.len(), remaining_groups.len()), 0);
                    return 0;
                }

                if remaining_springs[group_size] == '#' {
                    // We can't match here, because there would be too many broken springs

                    unsafe {
                        println!(
                            "{n:>width$}11 too many broken springs",
                            width = indent,
                            n = 0
                        );
                    }
                    unsafe {
                        indent -= 2;
                    }
                    self.matches
                        .insert((remaining_springs.len(), remaining_groups.len()), 0);
                    return 0;
                }

                unsafe {
                    println!(
                        "{n:>width$}12 put a group here and proceed",
                        width = indent,
                        n = 0
                    );
                }
                // OK - let's try putting the next group here.
                let val = self.can_start_group_at_or_after_position(
                    &remaining_springs[group_size + 1..],
                    &remaining_groups[1..],
                );
                unsafe {
                    indent -= 2;
                }
                self.matches
                    .insert((remaining_springs.len(), remaining_groups.len()), val);
                return val;
            }
            '?' => {
                // Can choose whether to start a group here or not

                if remaining_springs.len() == group_size {
                    // Exact size match - no more groups to check
                    unsafe {
                        println!(
                            "{n:>width$}13 exact size match - no more groups to check",
                            width = indent,
                            n = 0
                        );
                    }
                    unsafe {
                        indent -= 2;
                    }
                    self.matches
                        .insert((remaining_springs.len(), remaining_groups.len()), 1);
                    return 1;
                }
                if remaining_springs[0..group_size].iter().any(|c| *c == '.') {
                    unsafe {
                        println!(
                            "{n:>width$}14 working springs in the way",
                            width = indent,
                            n = 0
                        );
                    }
                    // Working springs in the way
                    let val = self.can_start_group_at_or_after_position(
                        &remaining_springs[1..],
                        &remaining_groups,
                    );
                    unsafe {
                        indent -= 2;
                    }

                    self.matches
                        .insert((remaining_springs.len(), remaining_groups.len()), val);
                    return val;
                }

                if remaining_springs[group_size] == '#' {
                    unsafe {
                        println!(
                            "{n:>width$}15 too many broken springs",
                            width = indent,
                            n = 0
                        );
                    }
                    // We can't match exactly here, because there would be too many broken springs
                    let val = self.can_start_group_at_or_after_position(
                        &remaining_springs[1..],
                        &remaining_groups,
                    );
                    unsafe {
                        indent -= 2;
                    }
                    self.matches
                        .insert((remaining_springs.len(), remaining_groups.len()), val);
                    return val;
                }

                // Can start here, or could not.
                unsafe {
                    println!(
                        "{n:>width$}16 can start here, or could not.  ",
                        width = indent,
                        n = 0
                    );
                }
                let val = self.can_start_group_at_or_after_position(
                    &remaining_springs[group_size + 1..],
                    &remaining_groups[1..],
                ) + self.can_start_group_at_or_after_position(
                    &remaining_springs[1..],
                    &remaining_groups,
                );
                unsafe {
                    indent -= 2;
                }
                self.matches
                    .insert((remaining_springs.len(), remaining_groups.len()), val);
                return val;
            }
            _ => panic!("Unexpected character {}", remaining_springs[0]),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
        let sample_input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(super::Solver12.part1(sample_input.lines()), "21");
    }

    #[test]
    fn part2() {
        let sample_input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(super::Solver12.part2(sample_input.lines()), "525152");
    }

    #[test]
    fn single_line_p1() {
        super::Solver12.part1("..?.????#?????????? 1,1,1,1,1,4".lines());
    }
    #[test]
    fn single_line_p2() {
        super::Solver12.part2("..?.????#?????????? 1,1,1,1,1,4".lines());
    }
}
