use std::str::Lines;

use crate::Solver;

pub struct Solver5;

#[derive(Debug)]
struct Mapping {
    from: i64,
    to: i64,
    delta: i64,
}

#[derive(Debug)]
struct SeedRange {
    start: i64,
    count: i64,
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

        let seeds: Vec<i64> = seeds
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let mut next_seed_ranges: Vec<SeedRange> = Vec::new();

        for ix in 0..seeds.len() / 2 {
            let start = seeds[2 * ix];
            let count = seeds[2 * ix + 1];
            next_seed_ranges.push(SeedRange { start, count });
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

        // let mut next_seed_ranges: Vec<SeedRange> = Vec::new(); ??

        for map in maps {
            let mut unmatched_seed_ranges = next_seed_ranges;
            next_seed_ranges = Vec::new();
            for mapping in &map {
                let seed_ranges = unmatched_seed_ranges;
                unmatched_seed_ranges = Vec::new();

                for sr in seed_ranges {
                    let sr_end = sr.start + sr.count - 1;
                    if sr.start < mapping.from {
                        if sr_end >= mapping.from {
                            // there's an overlap

                            let num_seeds = mapping.from - sr.start;

                            // seeds before the mapping
                            unmatched_seed_ranges.push(SeedRange {
                                start: sr.start,
                                count: num_seeds,
                            });

                            let mut remaining = sr.count - num_seeds;

                            if sr_end > mapping.to {
                                // there are some seeds after the matching set
                                let num_seeds = sr_end - mapping.to;

                                unmatched_seed_ranges.push(SeedRange {
                                    start: mapping.to + 1,
                                    count: num_seeds,
                                });

                                remaining -= num_seeds;
                            }

                            // rest of the range is in the mapping
                            next_seed_ranges.push(SeedRange {
                                start: mapping.from + mapping.delta,
                                count: remaining,
                            });
                        } else {
                            // no overlap
                            unmatched_seed_ranges.push(sr);
                        }
                    } else if sr.start <= mapping.to {
                        // some overlap

                        let mut remaining = sr.count;

                        if sr_end > mapping.to {
                            // there are some seeds after the matching set
                            let num_seeds = sr_end - mapping.to;

                            unmatched_seed_ranges.push(SeedRange {
                                start: mapping.to + 1,
                                count: num_seeds,
                            });

                            remaining -= num_seeds;
                        }

                        // rest of the range is in the mapping
                        next_seed_ranges.push(SeedRange {
                            start: sr.start + mapping.delta,
                            count: remaining,
                        });
                    } else {
                        // no overlap
                        unmatched_seed_ranges.push(sr);
                    }
                }
            }

            // handle seed ranges that intersected no mappings
            next_seed_ranges.append(&mut unmatched_seed_ranges);
        }

        let values = next_seed_ranges.iter().map(|sr| sr.start);
        let lowest = values.min();

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
