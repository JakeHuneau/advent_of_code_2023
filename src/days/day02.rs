#![allow(unused)]
use crate::{Solution, SolutionPair};
use itertools::{all, max};
use std::{fs::read_to_string, thread::available_parallelism};

#[derive(Debug, Copy, Clone)]
struct GameSet {
    green_count: usize,
    red_count: usize,
    blue_count: usize,
}

impl GameSet {
    fn is_possible(&self, available_cubes: GameSet) -> bool {
        self.green_count <= available_cubes.green_count
            && self.red_count <= available_cubes.red_count
            && self.blue_count <= available_cubes.blue_count
    }

    fn get_power(&self) -> usize {
        self.green_count * self.red_count * self.blue_count
    }
}

#[derive(Debug)]
struct Game {
    game_number: usize,
    sets: Vec<GameSet>,
}

impl Game {
    fn is_possible(&self, available_cubes: GameSet) -> bool {
        all(self.sets.clone(), |set| set.is_possible(available_cubes))
    }

    fn minimum_cubes(&self) -> GameSet {
        GameSet {
            blue_count: max(self.sets.iter().map(|set| set.blue_count)).unwrap(),
            red_count: max(self.sets.iter().map(|set| set.red_count)).unwrap(),
            green_count: max(self.sets.iter().map(|set| set.green_count)).unwrap(),
        }
    }
}

fn parse_game_set(game_set_str: &str) -> GameSet {
    // Looks like "<x> blue, <y> green, <z> red" but does not need any of these,
    // So can be "<x> blue, <z> red"
    let mut green_count = 0;
    let mut red_count = 0;
    let mut blue_count = 0;

    game_set_str.split(",").for_each(|cubes| {
        let parts: Vec<&str> = cubes.trim().split(" ").collect();
        if let [count, color] = parts.as_slice() {
            match *color {
                "green" => green_count = count.parse().unwrap(),
                "red" => red_count = count.parse().unwrap(),
                "blue" => blue_count = count.parse().unwrap(),
                _ => panic!("Unknown color: {}", color),
            };
        } else {
            panic!("Couldn't parse game set string: {}", game_set_str);
        }
    });

    GameSet {
        green_count,
        red_count,
        blue_count,
    }
}

fn parse_game(game_str: &str) -> Game {
    // Ex: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    let parts: Vec<&str> = game_str.split(":").collect();
    if let [game_str, game_sets] = parts.as_slice() {
        let game_number = game_str.split(" ").last().unwrap().parse().unwrap();
        let sets = game_sets
            .split(";")
            .map(|game_set_str| parse_game_set(game_set_str))
            .collect();
        Game { game_number, sets }
    } else {
        panic!("Couldn't parse game string: {}", game_str);
    }
}

pub fn solve() -> SolutionPair {
    let mut part_1_solution = 0;
    let mut part_2_solution = 0;

    let available_cubes = GameSet {
        red_count: 12,
        green_count: 13,
        blue_count: 14,
    };

    read_to_string("input/day2")
        .expect("Couldn't read file")
        .lines()
        .for_each(|line| {
            let game = parse_game(line);
            if game.is_possible(available_cubes) {
                part_1_solution += game.game_number;
            }
            let minimum_cubes = game.minimum_cubes();
            dbg!(minimum_cubes);
            part_2_solution += minimum_cubes.get_power();
        });
    (
        Solution::UInt(part_1_solution),
        Solution::UInt(part_2_solution),
    )
}
