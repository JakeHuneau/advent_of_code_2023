#![allow(unused)]
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

struct FoundNumber {
    number: char,
    index: usize,
}

pub fn solve() -> SolutionPair {
    // Filter out to get the numbers then use the first and last
    let part_1 = read_to_string("input/day1")
        .expect("Couldn't read file")
        .lines()
        .map(|s| {
            let nums: Vec<char> = s.chars().filter(|&c| "0123456789".contains(c)).collect();
            format!("{}{}", nums.first().unwrap(), nums.last().unwrap())
                .parse::<usize>()
                .unwrap()
        })
        .sum();

    // Track the index of the first and last finding of each possible value
    let part_2 = read_to_string("input/day1")
        .expect("Couldn't read file")
        .lines()
        .map(|s| {
            let mut first_number = FoundNumber {
                number: '0',
                index: usize::MAX,
            };
            let mut last_number = FoundNumber {
                number: '0',
                index: 0,
            };

            match s.find("one") {
                Some(found_index) => {
                    if found_index < first_number.index {
                        first_number = FoundNumber {
                            number: '1',
                            index: found_index,
                        }
                    }
                }
                None => {}
            };
            match s.rfind("one") {
                Some(found_index) => {
                    if found_index > last_number.index {
                        last_number = FoundNumber {
                            number: '1',
                            index: found_index,
                        }
                    }
                }
                None => {}
            };

            match s.find("two") {
                Some(found_index) => {
                    if found_index < first_number.index {
                        first_number = FoundNumber {
                            number: '2',
                            index: found_index,
                        }
                    }
                }
                None => {}
            };
            match s.rfind("two") {
                Some(found_index) => {
                    if found_index > last_number.index {
                        last_number = FoundNumber {
                            number: '2',
                            index: found_index,
                        }
                    }
                }
                None => {}
            };

            match s.find("three") {
                Some(found_index) => {
                    if found_index < first_number.index {
                        first_number = FoundNumber {
                            number: '3',
                            index: found_index,
                        }
                    }
                }
                None => {}
            };
            match s.rfind("three") {
                Some(found_index) => {
                    if found_index > last_number.index {
                        last_number = FoundNumber {
                            number: '3',
                            index: found_index,
                        }
                    }
                }
                None => {}
            };

            match s.find("four") {
                Some(found_index) => {
                    if found_index < first_number.index {
                        first_number = FoundNumber {
                            number: '4',
                            index: found_index,
                        }
                    }
                }
                None => {}
            };
            match s.rfind("four") {
                Some(found_index) => {
                    if found_index > last_number.index {
                        last_number = FoundNumber {
                            number: '4',
                            index: found_index,
                        }
                    }
                }
                None => {}
            };

            match s.find("five") {
                Some(found_index) => {
                    if found_index < first_number.index {
                        first_number = FoundNumber {
                            number: '5',
                            index: found_index,
                        }
                    }
                }
                None => {}
            };
            match s.rfind("five") {
                Some(found_index) => {
                    if found_index > last_number.index {
                        last_number = FoundNumber {
                            number: '5',
                            index: found_index,
                        }
                    }
                }
                None => {}
            };

            match s.find("six") {
                Some(found_index) => {
                    if found_index < first_number.index {
                        first_number = FoundNumber {
                            number: '6',
                            index: found_index,
                        }
                    }
                }
                None => {}
            };
            match s.rfind("six") {
                Some(found_index) => {
                    if found_index > last_number.index {
                        last_number = FoundNumber {
                            number: '6',
                            index: found_index,
                        }
                    }
                }
                None => {}
            };

            match s.find("seven") {
                Some(found_index) => {
                    if found_index < first_number.index {
                        first_number = FoundNumber {
                            number: '7',
                            index: found_index,
                        }
                    }
                }
                None => {}
            };
            match s.rfind("seven") {
                Some(found_index) => {
                    if found_index > last_number.index {
                        last_number = FoundNumber {
                            number: '7',
                            index: found_index,
                        }
                    }
                }
                None => {}
            };

            match s.find("eight") {
                Some(found_index) => {
                    if found_index < first_number.index {
                        first_number = FoundNumber {
                            number: '8',
                            index: found_index,
                        }
                    }
                }
                None => {}
            };
            match s.rfind("eight") {
                Some(found_index) => {
                    if found_index > last_number.index {
                        last_number = FoundNumber {
                            number: '8',
                            index: found_index,
                        }
                    }
                }
                None => {}
            };

            match s.find("nine") {
                Some(found_index) => {
                    if found_index < first_number.index {
                        first_number = FoundNumber {
                            number: '9',
                            index: found_index,
                        }
                    }
                }
                None => {}
            };
            match s.rfind("nine") {
                Some(found_index) => {
                    if found_index > last_number.index {
                        last_number = FoundNumber {
                            number: '9',
                            index: found_index,
                        }
                    }
                }
                None => {}
            };

            for n in ['1', '2', '3', '4', '5', '6', '7', '8', '9'] {
                match s.find(n) {
                    Some(found_index) => {
                        if found_index < first_number.index {
                            first_number = FoundNumber {
                                number: n,
                                index: found_index,
                            }
                        }
                    }
                    None => {}
                };
                match s.rfind(n) {
                    Some(found_index) => {
                        if found_index > last_number.index {
                            last_number = FoundNumber {
                                number: n,
                                index: found_index,
                            }
                        }
                    }
                    None => {}
                };
            }

            if last_number.index == 0 {
                last_number = FoundNumber {
                    number: first_number.number,
                    index: first_number.index,
                };
            }

            format!("{}{}", first_number.number, last_number.number)
                .parse::<usize>()
                .unwrap()
        })
        .sum();
    (Solution::UInt(part_1), Solution::UInt(part_2))
}
