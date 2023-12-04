#![allow(unused)]
use crate::{Solution, SolutionPair};
use std::collections::HashSet;
use std::fs::read_to_string;

pub fn solve() -> SolutionPair {
    let num_wins: Vec<usize> = read_to_string("input/day4")
        .expect("Couldn't read file")
        .lines()
        .map(|line| {
            let scores: Vec<&str> = line.split(":").last().unwrap().split("|").collect();
            let mut winning_numbers = scores[0].trim().split(" ").collect::<HashSet<&str>>();
            let mut numbers = scores[1].trim().split(" ").collect::<HashSet<&str>>();
            winning_numbers.remove("");
            numbers.remove("");
            winning_numbers.intersection(&numbers).count()
        })
        .collect();
    let part_1_solution = num_wins
        .iter()
        .map(|&game_wins| {
            if game_wins > 0 {
                2_usize.pow(game_wins as u32 - 1)
            } else {
                0
            }
        })
        .sum();
    let mut ticket_counts: Vec<usize> = vec![1; num_wins.len()];
    for (i, game_wins) in num_wins.iter().enumerate() {
        for game_num in i + 1..=i + game_wins {
            ticket_counts[game_num] += ticket_counts[i];
        }
    }
    let part_2_solution = ticket_counts.iter().sum();
    (
        Solution::UInt(part_1_solution),
        Solution::UInt(part_2_solution),
    )
}
