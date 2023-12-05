#![allow(unused)]
use crate::{Solution, SolutionPair};
use rayon::prelude::*;
use std::collections::VecDeque;
use std::fs::read_to_string;

#[derive(Debug)]
struct Map {
    destination_start: usize,
    source_start: usize,
    range: usize,
}

#[derive(Debug)]
struct SeedRange {
    start: usize,
    range: usize,
}

impl Map {
    fn get_destination(&self, source: usize) -> Option<usize> {
        // If there's a destination, returns that and None otherwise
        if source >= self.source_start && source < self.source_start + self.range {
            let difference = source - self.source_start;
            Some(self.destination_start + difference)
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Vec<Map>>) {
    let input = read_to_string(input).expect("Couldn't parse input");
    let mut input_parts = input.split("\n\n").collect::<VecDeque<&str>>();
    let seeds = input_parts
        .pop_front()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split(" ")
        .map(|seed_number| seed_number.parse::<usize>().unwrap())
        .collect();

    let mut maps = Vec::new();
    while let Some(map_string) = input_parts.pop_front() {
        maps.push(
            map_string
                .split(":")
                .last()
                .unwrap()
                .trim()
                .split("\n")
                .map(|mapping| {
                    let mapping_parts = mapping
                        .split(" ")
                        .map(|value| value.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
                    Map {
                        destination_start: mapping_parts[0],
                        source_start: mapping_parts[1],
                        range: mapping_parts[2],
                    }
                })
                .collect(),
        );
    }

    (seeds, maps)
}

pub fn solve() -> SolutionPair {
    let (seeds, maps) = parse_input("input/day5");
    let seed_ranges: Vec<SeedRange> = seeds
        .chunks(2)
        .map(|chunk| SeedRange {
            start: chunk[0],
            range: chunk[1],
        })
        .collect();

    let part_1_smallest_destination = seeds
        .into_par_iter()
        .map(|seed| {
            let mut current_source = seed;
            for map in &maps {
                for mapping in map {
                    if let Some(destination) = mapping.get_destination(current_source) {
                        current_source = destination;
                        break; // Only take the first one
                    }
                }
            }
            current_source
        })
        .min()
        .unwrap();

    let part_2_smallest_destination = seed_ranges
        .par_iter()
        .map(|seed_range| {
            (seed_range.start..seed_range.start + seed_range.range)
                .into_par_iter()
                .map(|seed| {
                    let mut current_source = seed;
                    for map in &maps {
                        for mapping in map {
                            if let Some(destination) = mapping.get_destination(current_source) {
                                current_source = destination;
                                break; // Only take the first one
                            }
                        }
                    }
                    current_source
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap();
    (
        Solution::UInt(part_1_smallest_destination),
        Solution::UInt(part_2_smallest_destination),
    )
}
