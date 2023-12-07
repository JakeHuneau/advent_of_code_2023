#![allow(unused)]
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

#[derive(Debug)]
struct Record {
    time: usize,
    distance: usize,
}

fn parse_input(input: &str) -> (Vec<Record>, Record) {
    let input = read_to_string(input).expect("Couldn't read input");
    let lines = input.lines().collect::<Vec<&str>>();
    let times_raw = lines.first().unwrap().split(":").last().unwrap().trim();
    let times = times_raw.split(" ").filter(|&v| v != "");
    let distances_raw = lines.last().unwrap().split(":").last().unwrap().trim();
    let distances = distances_raw.split(" ").filter(|&v| v != "");
    (
        times
            .zip(distances)
            .map(|(time, distance)| Record {
                time: time.parse::<usize>().unwrap(),
                distance: distance.parse::<usize>().unwrap(),
            })
            .collect(),
        Record {
            time: times_raw.replace(" ", "").parse::<usize>().unwrap(),
            distance: distances_raw.replace(" ", "").parse::<usize>().unwrap(),
        },
    )
}

pub fn build_distance_table(time: usize) -> Vec<usize> {
    // Builds a table where each entry is the distance for holding the button for that <index> amount of time
    // Distance is `(time - button_time) * button_time`
    // Distance table is a mirror so only gives first half. Example for 7 seconds is [0, 6, 10, 12, 12, 10, 6, 0] and this will give [6, 10, 12]
    let mut distances = Vec::new();
    for i in 1..(time / 2) {
        distances.push((time - i) * i);
    }
    distances
}

pub fn solve() -> SolutionPair {
    let (records, single_record) = parse_input("input/day6");
    let part_1_solution = records
        .into_iter()
        .map(|record| {
            build_distance_table(record.time)
                .into_iter()
                .filter(|&distance| distance > record.distance)
                .count()
                * 2
        })
        .product();
    let part_2_solution = build_distance_table(single_record.time)
        .into_iter()
        .filter(|&distance| distance > single_record.distance)
        .count()
        * 2;
    (
        Solution::UInt(part_1_solution),
        Solution::UInt(part_2_solution),
    )
}
