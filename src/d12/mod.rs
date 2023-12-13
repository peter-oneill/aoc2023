use crate::Solver;
use core::panic;
use std::{collections::HashMap, str::Lines};

pub struct Solver12;

impl Solver for Solver12 {
    fn day_number(&self) -> u32 {
        12
    }

    fn part1(&self, input_lines: Lines) -> String {
        // naive brute force will take too long.  Instead need some state to limit the search space
        let mut sum: u64 = 0;
        let number_matcher = regex::Regex::new(r"\d+").unwrap();

        for line in input_lines {
            {
                let (spring_row, numbers) = line.split_once(' ').unwrap();

                let group_sizes: Vec<usize> = number_matcher
                    .find_iter(numbers)
                    .map(|n| n.as_str().parse::<usize>().unwrap())
                    .collect();

                let mut match_cache = MatchCache {
                    matches: HashMap::new(),
                };

                let matches_this_row = match_cache.can_start_group_at_or_after_position(
                    &spring_row.chars().collect::<Vec<char>>(),
                    &group_sizes,
                );
                sum += matches_this_row;
            }
        }

        sum.to_string()
    }

    fn part2(&self, input_lines: Lines) -> String {
        // naive brute force will take too long.  Instead need some state to limit the search space
        let mut sum: u64 = 0;
        let number_matcher = regex::Regex::new(r"\d+").unwrap();

        for line in input_lines {
            let (springs, numbers) = line.split_once(' ').unwrap();
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
            sum += matches_this_row
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

        // Set some local variables for values we keep getting
        let num_remaining_springs = remaining_springs.len();
        let num_remaining_groups = remaining_groups.len();
        let num_springs_in_remaining_groups = remaining_groups.iter().sum::<usize>();
        let this_entry = (num_remaining_springs, num_remaining_groups);

        if let Some(number) = self.matches.get(&this_entry) {
            return *number;
        }

        let remaining_broken_spring_count = remaining_springs.iter().filter(|c| **c == '#').count();
        let remaining_non_working_spring_count =
            remaining_springs.iter().filter(|c| **c != '.').count();

        // There are no more groups to match
        if remaining_groups.is_empty() {
            // If there are no more broken springs, return one match variant
            if remaining_broken_spring_count == 0 {
                self.matches.insert(this_entry, 1);
                return 1;
            } else {
                // There are more broken springs - whoops!  can't match
                self.matches.insert(this_entry, 0);
                return 0;
            }
        }

        let this_group_size = remaining_groups[0];

        // Now rule out a few cases where we can't match any more, to shortcut the recursion

        // There's not enough space to match just the next group
        if (num_remaining_springs < this_group_size ) ||

           // There are too many broken springs in the row, to match the remaining groups
           (remaining_broken_spring_count > num_springs_in_remaining_groups ) ||

           // There are more groups to match, but not enough springs (of any type) to match them
           (num_remaining_springs < remaining_groups.iter().map(|n| n + 1).sum::<usize>() - 1 ) ||

           // There are more groups to match, but not enough springs of the right type to match them
           (remaining_non_working_spring_count < num_springs_in_remaining_groups )
        {
            self.matches.insert(this_entry, 0);
            return 0;
        }

        match remaining_springs[0] {
            '.' => {
                // Can't start exactly here - can we start at the next position?

                let val = self.can_start_group_at_or_after_position(
                    &remaining_springs[1..],
                    remaining_groups,
                );

                self.matches.insert(this_entry, val);
                val
            }
            '#' => {
                // must start a group here if we can
                if num_remaining_springs == this_group_size {
                    self.matches.insert(this_entry, 1);
                    return 1;
                }

                // Working springs in the way
                if remaining_springs[0..this_group_size]
                    .iter()
                    .any(|c| *c == '.') ||
                    // We can't match here, because there would be too many broken springs
                    remaining_springs[this_group_size] == '#'
                {
                    self.matches.insert(this_entry, 0);
                    return 0;
                }

                // OK - let's try putting the next group here.
                let val = self.can_start_group_at_or_after_position(
                    &remaining_springs[this_group_size + 1..],
                    &remaining_groups[1..],
                );

                self.matches.insert(this_entry, val);
                val
            }
            '?' => {
                // Can choose whether to start a group here or not

                if num_remaining_springs == this_group_size {
                    // Exact size match - no more groups to check
                    self.matches.insert(this_entry, 1);
                    return 1;
                }

                // Working springs in the way
                if remaining_springs[0..this_group_size]
                    .iter()
                    .any(|c| *c == '.') ||

                 // We can't match exactly here, because there would be too many broken springs
                    remaining_springs[this_group_size] == '#'
                {
                    // Try the next location
                    let val = self.can_start_group_at_or_after_position(
                        &remaining_springs[1..],
                        remaining_groups,
                    );

                    self.matches.insert(this_entry, val);
                    return val;
                }

                // Can start here, or could try the next location.
                let val = self.can_start_group_at_or_after_position(
                    &remaining_springs[this_group_size + 1..],
                    &remaining_groups[1..],
                ) + self.can_start_group_at_or_after_position(
                    &remaining_springs[1..],
                    remaining_groups,
                );

                self.matches.insert(this_entry, val);
                val
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
