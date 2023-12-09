#![allow(unused)]
use crate::{Solution, SolutionPair};
use num::integer;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Pair {
    left: String,
    right: String,
}

impl Pair {
    fn traverse(&self, direction: Direction) -> String {
        match direction {
            Direction::Left => self.left.clone(),
            Direction::Right => self.right.clone(),
        }
    }
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day8").unwrap();
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let instructions: Vec<Direction> = parts
        .first()
        .unwrap()
        .chars()
        .map(|ch| match ch {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unknown direction"),
        })
        .collect();
    let mut nodes: HashMap<String, Pair> = HashMap::new();
    parts.last().unwrap().lines().for_each(|l| {
        let line_parts = l.split(" = ").collect::<Vec<&str>>();
        let parent = line_parts.first().unwrap().to_string();
        let binding = line_parts.last().unwrap().replace("(", "").replace(")", "");
        let children = binding.split(",").collect::<Vec<&str>>();
        let left = children.first().unwrap().trim().to_string();
        let right = children.last().unwrap().trim().to_string();
        nodes.insert(parent, Pair { left, right });
    });
    let mut steps = 0;
    let mut current_node = String::from("AAA");
    'outer: loop {
        for direction in &instructions {
            steps += 1;
            current_node = nodes
                .get(&current_node)
                .unwrap()
                .traverse(direction.clone());
            if current_node == String::from("ZZZ") {
                break 'outer;
            }
        }
    }

    let part_2_solution = nodes
        .clone()
        .into_keys()
        .filter(|parent| parent.ends_with("A"))
        .map(|node| {
            let mut current_node = node.clone();
            let mut steps = 0;
            'outer: loop {
                for direction in &instructions {
                    steps += 1;
                    current_node = nodes
                        .get(&current_node)
                        .unwrap()
                        .traverse(direction.clone());
                    if current_node.ends_with("Z") {
                        break 'outer;
                    }
                }
            }
            steps
        })
        .fold(1, integer::lcm);

    (Solution::UInt(steps), Solution::UInt(part_2_solution))
}
