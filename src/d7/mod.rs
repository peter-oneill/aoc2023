use std::{collections::HashMap, str::Lines};

use crate::Solver;

pub struct Solver7;

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

impl Solver for Solver7 {
    fn day_number(&self) -> u32 {
        7
    }

    fn part1(&self, input_lines: Lines) -> String {
        let mut fives = Vec::<Hand>::new();
        let mut fours = Vec::<Hand>::new();
        let mut full_houses = Vec::<Hand>::new();
        let mut threes = Vec::<Hand>::new();
        let mut two_pairs = Vec::<Hand>::new();
        let mut one_pairs = Vec::<Hand>::new();
        let mut high_cards = Vec::<Hand>::new();

        for line in input_lines {
            let mut parts = line.split(' ');
            let cards = parts.next().unwrap().chars().map(|c| {
                if c.is_numeric() {
                    c.to_digit(10).unwrap() as i32
                } else {
                    match c {
                        'T' => 10,
                        'J' => 11,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => panic!("Unknown card"),
                    }
                }
            });
            let bet = parts.next().unwrap().parse::<usize>().unwrap();

            let mut card_counts: HashMap<i32, i32> = HashMap::new();
            let mut quality = 0;

            for card in cards {
                quality = quality * 14 + card;
                match card_counts.get_mut(&card) {
                    Some(count) => *count += 1,
                    None => {
                        card_counts.insert(card, 1);
                    }
                };
            }

            let mut ordered_cards = card_counts.values().collect::<Vec<&i32>>();
            ordered_cards.sort();

            match (ordered_cards.pop(), ordered_cards.pop()) {
                (Some(5), _) => fives.push(Hand { quality, bet }),
                (Some(4), _) => fours.push(Hand { quality, bet }),
                (Some(3), Some(2)) => full_houses.push(Hand { quality, bet }),
                (Some(3), _) => threes.push(Hand { quality, bet }),
                (Some(2), Some(2)) => two_pairs.push(Hand { quality, bet }),
                (Some(2), _) => one_pairs.push(Hand { quality, bet }),
                _ => high_cards.push(Hand { quality, bet }),
            }
        }

        fives.sort();
        fours.sort();
        full_houses.sort();
        threes.sort();
        two_pairs.sort();
        one_pairs.sort();
        high_cards.sort();

        let mut total_hands = fives.len()
            + fours.len()
            + full_houses.len()
            + threes.len()
            + two_pairs.len()
            + one_pairs.len()
            + high_cards.len();

        let ordered_hands = vec![
            fives,
            fours,
            full_houses,
            threes,
            two_pairs,
            one_pairs,
            high_cards,
        ];

        let mut sum = 0;

        for mut hand_type in ordered_hands {
            hand_type.reverse();
            for hand in &hand_type {
                sum += hand.bet * total_hands;
                total_hands -= 1;
            }
        }

        sum.to_string()
    }

    fn part2(&self, input_lines: Lines) -> String {
        let mut fives = Vec::<Hand>::new();
        let mut fours = Vec::<Hand>::new();
        let mut full_houses = Vec::<Hand>::new();
        let mut threes = Vec::<Hand>::new();
        let mut two_pairs = Vec::<Hand>::new();
        let mut one_pairs = Vec::<Hand>::new();
        let mut high_cards = Vec::<Hand>::new();

        for line in input_lines {
            let mut parts = line.split(' ');
            let vals = parts.next().unwrap();

            let cards = vals.chars().map(|c| {
                if c.is_numeric() {
                    c.to_digit(10).unwrap() as i32
                } else {
                    match c {
                        'T' => 10,
                        'J' => 1,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => panic!("Unknown card"),
                    }
                }
            });
            let bet = parts.next().unwrap().parse::<usize>().unwrap();

            let mut card_counts: HashMap<i32, i32> = HashMap::new();
            let mut quality = 0;

            for card in cards {
                quality = quality * 14 + card;
                match card_counts.get_mut(&card) {
                    Some(count) => *count += 1,
                    None => {
                        card_counts.insert(card, 1);
                    }
                };
            }

            let num_jokers = card_counts.remove(&1).unwrap_or(0);

            let mut ordered_cards = card_counts.values().cloned().collect::<Vec<i32>>();
            ordered_cards.sort();

            let most = ordered_cards.pop().unwrap_or(0) + num_jokers;
            ordered_cards.push(most);

            match (ordered_cards.pop(), ordered_cards.pop()) {
                (Some(5), _) => fives.push(Hand { quality, bet }),
                (Some(4), _) => fours.push(Hand { quality, bet }),
                (Some(3), Some(2)) => full_houses.push(Hand { quality, bet }),
                (Some(3), _) => threes.push(Hand { quality, bet }),
                (Some(2), Some(2)) => two_pairs.push(Hand { quality, bet }),
                (Some(2), _) => one_pairs.push(Hand { quality, bet }),
                _ => high_cards.push(Hand { quality, bet }),
            }
        }

        let mut total_hands = fives.len()
            + fours.len()
            + full_houses.len()
            + threes.len()
            + two_pairs.len()
            + one_pairs.len()
            + high_cards.len();

        let ordered_hands = vec![
            fives,
            fours,
            full_houses,
            threes,
            two_pairs,
            one_pairs,
            high_cards,
        ];

        let mut sum = 0;

        for mut hand_type in ordered_hands {
            hand_type.sort();
            hand_type.reverse();
            for hand in &hand_type {
                sum += hand.bet * total_hands;
                total_hands -= 1;
            }
        }

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
        let sample_input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(super::Solver7.part1(sample_input.lines()), "6440");
    }
    #[test]
    fn part2() {
        let sample_input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(super::Solver7.part2(sample_input.lines()), "5905");
    }
}
