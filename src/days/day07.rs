#![allow(unused)]
use crate::{Solution, SolutionPair};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;

fn map_card(card: char) -> usize {
    if let Some(value) = card.to_digit(10) {
        value as usize
    } else {
        match card {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 0,
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<usize>,
    bid: usize,
}

impl Hand {
    fn get_points(&self) -> usize {
        // Returns 0 for no pair
        // 1 for 1 pair
        // 2 for 2 pair
        // 3 for 3 of a kind
        // 4 for full house
        // 5 for 4 of a kind
        // 6 for five of a kind
        let mut counts: [u8; 15] = [0; 15];
        self.cards
            .iter()
            .for_each(|&card| counts[card as usize] += 1);
        if counts[0] > 0 {
            let mut best_score = 0;
            let mut best_score_index = 0;
            for (idx, &c) in counts[1..].iter().enumerate() {
                if c > best_score {
                    best_score = c;
                    best_score_index = idx;
                }
            }
            counts[best_score_index + 1] += counts[0];
            counts[0] = 0;
        }
        if counts.contains(&5) {
            return 6;
        } else if counts.contains(&4) {
            return 5;
        } else if counts.contains(&3) {
            if counts.contains(&2) {
                return 4;
            }
            return 3;
        } else if counts.contains(&2) {
            if counts.into_iter().filter(|&count| count == 2).count() == 2 {
                return 2;
            }
            return 1;
        }
        0
    }

    fn joker_transform(&mut self) {
        for (idx, &card) in self.cards.clone().iter().enumerate() {
            if card == 11 {
                self.cards[idx] = 0;
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_points().cmp(&other.get_points()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for (this_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    if this_card > other_card {
                        return Ordering::Greater;
                    } else if this_card < other_card {
                        return Ordering::Less;
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

pub fn solve() -> SolutionPair {
    let mut hands: Vec<Hand> = read_to_string("input/day7")
        .unwrap()
        .lines()
        .map(|hand| {
            let parts: Vec<&str> = hand.split(" ").collect();
            let cards = parts.first().unwrap();
            let bid = parts.last().unwrap();

            Hand {
                cards: cards.chars().map(|card| map_card(card)).collect(),
                bid: bid.parse().unwrap(),
            }
        })
        .collect();
    hands.sort();
    let part_1_solution = hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bid)
        .sum();

    hands.iter_mut().for_each(|hand| hand.joker_transform());
    hands.sort();
    let part_2_solution = hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bid)
        .sum();

    (
        Solution::UInt(part_1_solution),
        (Solution::UInt(part_2_solution)),
    )
}
