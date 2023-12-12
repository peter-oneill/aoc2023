use num::complex::ComplexFloat;

use crate::Solver;
use core::panic;
use std::str::Lines;

pub struct Solver12;

impl Solver for Solver12 {
    fn day_number(&self) -> u32 {
        12
    }

    fn part1(&self, input_lines: Lines) -> String {
        let mut sum = 0;
        let number_matcher = regex::Regex::new(r"\d+").unwrap();

        let mut all_possible_lines = Vec::new();

        let max_line_len = input_lines
            .clone()
            .map(|line| line.split(" ").next().unwrap().len())
            .max()
            .unwrap()
            .try_into()
            .unwrap();

        for ii in 0..2_u64.pow(max_line_len) {
            let mut line = String::new();
            for jj in 0..max_line_len {
                if ii & (1 << jj) != 0 {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            all_possible_lines.push(line);
        }

        for line in input_lines {
            let (whole_line, numbers) = line.split_once(" ").unwrap();

            // First build a regex to match this line against test lines
            let whole_line_matcher =
                regex::Regex::new(&whole_line.replace(".", r"\.").replace("?", "[.#]")).unwrap();

            // Now build a matcher for the working/not working combinations in the line
            let mut needle = r"^\.*".to_string();

            let mut number_matches = number_matcher.find_iter(numbers);

            if let Some(number) = number_matches.next() {
                needle.push_str(&format!("#{{{}}}", number.as_str()));
            }
            for number in number_matches {
                needle.push_str(r"\.+");
                needle.push_str(&format!("#{{{}}}", number.as_str()));
            }
            needle.push_str(r"\.*$");
            let finished_needle = regex::Regex::new(&needle).unwrap();
            // println!("line: {}", line);

            // Now loop through each possible line to find matches
            let line_len = whole_line.len();
            let num_possibilities = 2_usize.pow(line_len.try_into().unwrap());

            for possible_line in all_possible_lines.split_at(num_possibilities).0 {
                let part_to_match = &possible_line[0..whole_line.len()];

                if !whole_line_matcher.is_match(part_to_match) {
                    continue;
                }

                if finished_needle.is_match(part_to_match) {
                    // println!("match {}, {}", line, possible_line);
                    sum += 1;
                }
            }
        }
        sum.to_string()
    }

    fn part2(&self, input_lines: Lines) -> String {
        // naive brute force will take too long.  Instead need some state to limit the search space
        let mut sum = 0;
        let number_matcher = regex::Regex::new(r"\d+").unwrap();

        for line in input_lines {
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

            let matches_this_row = can_start_group_at_or_after_position(
                &spring_row.chars().collect::<Vec<char>>(),
                &group_sizes,
            );
            // println!("matches_this_row: {}", matches_this_row);
            sum += matches_this_row;
            // break;
        }
        sum.to_string()
    }
}

fn can_start_group_at_or_after_position(
    remaining_springs: &[char],
    remaining_groups: &[usize],
) -> u32 {
    print!("{:?}, {:?} ", remaining_springs, remaining_groups);
    let remaining_broken_spring_count = remaining_springs.iter().filter(|c| **c == '#').count();

    // There are no more groups to match
    if remaining_groups.len() == 0 {
        // If there are no more broken springs, return one match variant
        if remaining_broken_spring_count == 0 {
            // println!(" no more broken springs");
            return 1;
        } else {
            // There are more broken springs - whoops!  can't match
            // println!(" more broken springs - whoops can't match");

            return 0;
        }
    }

    // There's not enough space to match just the next group
    if remaining_springs.len() < remaining_groups[0] {
        // println!(" not enough space to match just the next group");

        return 0;
    }

    // Do some checks to shortcut recursion when it's bound to fail

    // There are too many broken springs in the row, to match the remaining groups
    if remaining_broken_spring_count > remaining_groups.iter().sum() {
        // println!(" too many broken in row");

        return 0;
    }

    // There are more groups to match, but not enough springs (of any type) to match them
    if remaining_springs.len() < remaining_groups.iter().map(|n| n + 1).sum::<usize>() - 1 {
        // println!(
        // "not enough springs of any type to match {} {}",
        // remaining_springs.len(),
        // remaining_groups.iter().map(|n| n + 1).sum::<usize>()
        // );

        return 0;
    }

    // There are more groups to match, but not enough springs of the right type to match them
    if remaining_springs.iter().filter(|c| **c != '.').count() < remaining_groups.iter().sum() {
        // println!("not enough springs of the right type to match");
        return 0;
    }

    let group_size = remaining_groups[0];

    match remaining_springs[0] {
        '.' => {
            // Can't start exactly here - can we start at the next position?

            // println!("can't start exactly here - can we start at the next position?");
            return can_start_group_at_or_after_position(&remaining_springs[1..], remaining_groups);
        }
        '#' => {
            // must start a group here if we can
            if remaining_springs.len() == group_size {
                // println!("exact size match - no more groups to check");
                // Exact size match - no more groups to check
                return 1;
            }

            if remaining_springs[0..group_size].iter().any(|c| *c == '.') {
                // Working springs in the way
                // println!("working springs in the way");
                return 0;
            }

            if remaining_springs[group_size] == '#' {
                // We can't match here, because there would be too many broken springs

                // println!("too many broken springs");
                return 0;
            }

            // OK - let's try putting the next group here.
            return can_start_group_at_or_after_position(
                &remaining_springs[group_size + 1..],
                &remaining_groups[1..],
            );
        }
        '?' => {
            // Can choose whether to start a group here or not

            if remaining_springs.len() == group_size {
                // Exact size match - no more groups to check
                // println!("exact size match - no more groups to check");
                return 1;
            }
            if remaining_springs[0..group_size].iter().any(|c| *c == '.') {
                // println!("working springs in the way");
                // Working springs in the way
                return can_start_group_at_or_after_position(
                    &remaining_springs[1..],
                    &remaining_groups,
                );
            }

            if remaining_springs[group_size] == '#' {
                // println!("too many broken springs");
                // We can't match exactly here, because there would be too many broken springs
                return can_start_group_at_or_after_position(
                    &remaining_springs[1..],
                    &remaining_groups,
                );
            }

            // Can start here, or could not.
            return can_start_group_at_or_after_position(
                &remaining_springs[group_size + 1..],
                &remaining_groups[1..],
            ) + can_start_group_at_or_after_position(
                &remaining_springs[1..],
                &remaining_groups,
            );
        }
        _ => panic!("Unexpected character {}", remaining_springs[0]),
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
}
