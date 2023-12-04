#![allow(unused)]
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

fn check_neighbors(schematic: &Vec<Vec<char>>, x: usize, y: usize, only_right: bool) -> bool {
    // Checks neighbors for any symbol other than '.' or a number
    // If 'only_right' then will only check up-right, right, down-right
    if let Some(above) = y.checked_sub(1) {
        // up-right
        if *schematic.get(above).unwrap().get(x + 1).unwrap_or(&'.') != '.' {
            return true;
        }
    }
    // right
    let right_value = *schematic.get(y).unwrap().get(x + 1).unwrap_or(&'.');
    if !right_value.is_digit(10) && right_value != '.' {
        return true;
    }
    // down-right
    if schematic
        .get(y + 1)
        .unwrap_or(&Vec::<char>::new())
        .get(x + 1)
        .unwrap_or(&'.')
        .clone()
        != '.'
    {
        return true;
    }

    if !only_right {
        // Check everything else
        if let Some(left) = x.checked_sub(1) {
            if let Some(above) = y.checked_sub(1) {
                // up-left
                if *schematic.get(above).unwrap().get(left).unwrap() != '.' {
                    return true;
                }
            }
            // left
            if *schematic.get(y).unwrap().get(left).unwrap() != '.' {
                return true;
            }
            // down-left
            if *schematic
                .get(y + 1)
                .unwrap_or(&Vec::<char>::new())
                .get(left)
                .unwrap_or(&'.')
                != '.'
            {
                return true;
            }
        }
        // up
        if let Some(above) = y.checked_sub(1) {
            if *schematic.get(above).unwrap().get(x).unwrap() != '.' {
                return true;
            }
        }
        // down
        if *schematic
            .get(y + 1)
            .unwrap_or(&Vec::<char>::new())
            .get(x)
            .unwrap_or(&'.')
            != '.'
        {
            return true;
        }
    }
    false
}

fn get_full_number(row: &Vec<char>, x: usize) -> (usize, usize) {
    // Returns full number, end index
    let mut number_buffer: Vec<char> = vec![row.get(x).unwrap().clone()];
    let mut current_x = x - 1;
    loop {
        let current_value = *row.get(current_x).unwrap();
        if current_value.is_digit(10) {
            number_buffer.insert(0, current_value);
        } else {
            break;
        }
        if let Some(new_x) = current_x.checked_sub(1) {
            current_x = new_x;
        } else {
            break;
        }
    }
    current_x = x + 1;
    loop {
        let current_value = *row.get(current_x).unwrap_or(&'.');
        if current_value.is_digit(10) {
            number_buffer.push(current_value)
        } else {
            break;
        }
        current_x += 1;
    }
    (
        number_buffer
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap(),
        current_x - 1,
    )
}

fn get_number_neighbors(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<usize> {
    let mut number_neighbors = Vec::new();

    let mut x_end = 0;

    if let Some(left) = x.checked_sub(1) {
        // left
        if schematic.get(y).unwrap().get(left).unwrap().is_digit(10) {
            let (full_number, _) = get_full_number(schematic.get(y).unwrap(), left);
            number_neighbors.push(full_number);
        }
    }

    // right
    if schematic
        .get(y)
        .unwrap()
        .get(x + 1)
        .unwrap_or(&'.')
        .is_digit(10)
    {
        let (full_number, _) = get_full_number(schematic.get(y).unwrap(), x + 1);
        number_neighbors.push(full_number);
    }

    // above
    if let Some(above) = y.checked_sub(1) {
        // up-left
        if let Some(left) = x.checked_sub(1) {
            if schematic
                .get(above)
                .unwrap()
                .get(left)
                .unwrap()
                .is_digit(10)
            {
                let (full_number, this_x_end) =
                    get_full_number(schematic.get(above).unwrap(), left);
                x_end = this_x_end;
                number_neighbors.push(full_number);
            }
        }

        // up
        if x_end < x && schematic.get(above).unwrap().get(x).unwrap().is_digit(10) {
            let (full_number, this_x_end) = get_full_number(schematic.get(above).unwrap(), x);
            x_end = this_x_end;
            number_neighbors.push(full_number);
        }

        // up-right
        if x_end < x
            && schematic
                .get(above)
                .unwrap()
                .get(x + 1)
                .unwrap_or(&'.')
                .is_digit(10)
        {
            let (full_number, _) = get_full_number(schematic.get(above).unwrap(), x + 1);
            number_neighbors.push(full_number);
        }
    }

    // under
    x_end = 0;

    // down-left
    if let Some(left) = x.checked_sub(1) {
        if schematic
            .get(y + 1)
            .unwrap()
            .get(left)
            .unwrap()
            .is_digit(10)
        {
            let (full_number, this_x_end) = get_full_number(schematic.get(y + 1).unwrap(), left);
            x_end = this_x_end;
            number_neighbors.push(full_number);
        }
    }

    // down
    if x_end < x && schematic.get(y + 1).unwrap().get(x).unwrap().is_digit(10) {
        let (full_number, this_x_end) = get_full_number(schematic.get(y + 1).unwrap(), x);
        x_end = this_x_end;
        number_neighbors.push(full_number);
    }

    // down-right
    if x_end < x
        && schematic
            .get(y + 1)
            .unwrap()
            .get(x + 1)
            .unwrap_or(&'.')
            .is_digit(10)
    {
        let (full_number, x_end) = get_full_number(schematic.get(y + 1).unwrap(), x + 1);
        number_neighbors.push(full_number);
    }

    number_neighbors
}

fn walk_schematic(schematic: Vec<Vec<char>>) -> (usize, usize) {
    let mut part1_sum = 0;
    let mut part2_sum = 0;
    let mut x = 0;
    let mut y = 0;
    let width = schematic.first().unwrap().len();
    let height = schematic.len();

    let mut number_buffer: Vec<char> = Vec::new();
    let mut number_has_symbol_neighbor = false;

    while y < height {
        x = 0;
        while x < width {
            let current_value = schematic
                .get(y)
                .expect("Out of bounds with y")
                .get(x)
                .expect("Out of bounds with x");
            if current_value.is_digit(10) {
                number_buffer.push(current_value.clone());

                // Check neighbors if doesn't already have symbol
                if !number_has_symbol_neighbor {
                    // If we're extending the number, only need to check right side
                    number_has_symbol_neighbor =
                        check_neighbors(&schematic, x, y, number_buffer.len() > 1);
                }
            } else {
                // not at a number, need to check buffer and reset
                if number_buffer.len() > 0 {
                    if number_has_symbol_neighbor {
                        part1_sum += number_buffer
                            .iter()
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap();
                    }
                    number_buffer.clear();
                    number_has_symbol_neighbor = false;
                }
            }

            if current_value.clone() == '*' {
                let number_neighbors = get_number_neighbors(&schematic, x, y);
                println!("{} {} {:?}", y, x, number_neighbors);
                if number_neighbors.len() == 2 {
                    part2_sum +=
                        number_neighbors.first().unwrap() * number_neighbors.last().unwrap();
                }
            }
            x += 1;
        }
        if number_buffer.len() > 0 {
            if number_has_symbol_neighbor {
                part1_sum += number_buffer
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
            }
            number_buffer.clear();
            number_has_symbol_neighbor = false;
        }

        y += 1;
    }

    (part1_sum, part2_sum)
}

pub fn solve() -> SolutionPair {
    let schematic: Vec<Vec<char>> = read_to_string("input/day3")
        .expect("Could not read file")
        .lines()
        .map(|line| line.chars().into_iter().collect())
        .collect();
    let (sol_1, sol_2) = walk_schematic(schematic);
    (Solution::UInt(sol_1), Solution::UInt(sol_2))
}
