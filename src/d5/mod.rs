use std::str::Lines;

use crate::Solver;

pub struct Solver5;

#[derive(Debug)]
struct Mapping {
    from: i64,
    to: i64,
    delta: i64,
}

impl Solver for Solver5 {
    fn day_number(&self) -> u32 {
        5
    }

    fn part1(&self, mut input_lines: Lines) -> String {
        let mut maps: Vec<Vec<Mapping>> = Vec::new();

        let seeds = input_lines.next().unwrap().split(':').last().unwrap();

        let mut seeds: Vec<i64> = seeds
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let mut map_num = 0;

        for line in input_lines.filter(|l| !l.is_empty()) {
            if !line.chars().nth(0).unwrap().is_numeric() {
                map_num += 1;
                maps.push(Vec::new());
                continue;
            }

            // else it's a mapping
            let mut nums = line.split(' ').map(|s| s.parse::<i64>().unwrap());
            let dest_range_start = nums.next().unwrap();
            let source_range_start = nums.next().unwrap();
            let range_len = nums.next().unwrap();

            let new_map = Mapping {
                from: source_range_start,
                to: source_range_start + range_len - 1,
                delta: dest_range_start - source_range_start,
            };

            maps[map_num - 1].push(new_map);
        }

        let mut lowest: Option<i64> = None;

        for seed in &mut seeds {
            for map in &maps {
                for mapping in map {
                    if *seed >= mapping.from && *seed <= mapping.to {
                        *seed += mapping.delta;
                        break;
                    }
                }
            }

            if let Some(v) = lowest {
                if *seed < v {
                    lowest = Some(*seed);
                }
            } else {
                lowest = Some(*seed);
            }
        }

        lowest.unwrap().to_string()
    }

    fn part2(&self, mut input_lines: Lines) -> String {
        let mut maps: Vec<Vec<Mapping>> = Vec::new();

        let seeds = input_lines.next().unwrap().split(':').last().unwrap();

        let seed_ranges: Vec<i64> = seeds
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let mut seeds: Vec<i64> = Vec::new();

        for ix in 0..seed_ranges.len() / 2 {
            let start_val = seed_ranges[2 * ix];
            let count = seed_ranges[2 * ix + 1];
            for d in 0..count {
                seeds.push(start_val + d);
            }
        }

        let mut map_num = 0;

        for line in input_lines.filter(|l| !l.is_empty()) {
            if !line.chars().nth(0).unwrap().is_numeric() {
                map_num += 1;
                maps.push(Vec::new());
                continue;
            }

            // else it's a mapping
            let mut nums = line.split(' ').map(|s| s.parse::<i64>().unwrap());
            let dest_range_start = nums.next().unwrap();
            let source_range_start = nums.next().unwrap();
            let range_len = nums.next().unwrap();

            let new_map = Mapping {
                from: source_range_start,
                to: source_range_start + range_len - 1,
                delta: dest_range_start - source_range_start,
            };

            maps[map_num - 1].push(new_map);
        }

        let mut lowest: Option<i64> = None;

        for seed in &mut seeds {
            for map in &maps {
                for mapping in map {
                    if *seed >= mapping.from && *seed <= mapping.to {
                        *seed += mapping.delta;
                        break;
                    }
                }
            }

            if let Some(v) = lowest {
                if *seed < v {
                    lowest = Some(*seed);
                }
            } else {
                lowest = Some(*seed);
            }
        }

        lowest.unwrap().to_string()
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
        assert_eq!(super::Solver5.part1(sample_input.lines()), "35");
    }
    #[test]
    fn part2() {
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
        assert_eq!(super::Solver5.part2(sample_input.lines()), "46");
    }
}
