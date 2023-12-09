#![allow(unused)]
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

fn extrapolate_next_value(values: Vec<isize>) -> (isize, isize) {
    /// Given a number sequence, figure out the next value.
    /// Do this by finding differences until all 0's, then
    /// work way back up.
    /// Ex: 0   3   6   9  12  15
    ///  Goes to
    /// 0   3   6   9  12  15  [18]
    ///   3   3   3   3   3   [3]
    ///     0   0   0   0   0
    /// Returns: (extrapolated before value, after value)
    // First work way down
    let mut tower: Vec<Vec<isize>> = vec![values.clone()];
    let mut current_values = values;
    loop {
        let differences: Vec<isize> = current_values
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect();
        if differences.iter().all(|&v| v == 0) {
            break;
        } else {
            tower.push(differences.clone());
            current_values = differences;
        }
    }

    // Now work way up
    let mut current_diff_end = tower.last().unwrap().last().unwrap().clone();
    let mut current_diff_start = tower.last().unwrap().first().unwrap().clone();
    tower.pop();
    tower.iter().rev().for_each(|current_row| {
        current_diff_end = current_row.last().unwrap() + current_diff_end;
        current_diff_start = current_row.first().unwrap() - current_diff_start;
    });
    (current_diff_start, current_diff_end)
}

pub fn solve() -> SolutionPair {
    let data: Vec<Vec<isize>> = read_to_string("input/day9")
        .unwrap()
        .lines()
        .map(|l| l.split(" ").map(|v| v.parse::<isize>().unwrap()).collect())
        .collect();
    let mut part_1 = 0;
    let mut part_2 = 0;
    for d in data {
        let (this_2, this_1) = extrapolate_next_value(d);
        part_1 += this_1;
        part_2 += this_2;
    }
    (Solution::Int(part_1), Solution::Int(part_2))
}
