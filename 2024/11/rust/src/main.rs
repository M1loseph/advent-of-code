use std::{collections::HashMap, fs::read_to_string};
use common::benchmark::{benchmark, TimeUnit};

#[derive(Clone, Copy)]
struct Rock {
    number: u64,
}

impl Rock {
    fn digits(&self) -> u32 {
        let mut current = self.number;
        if current == 0 {
            return 1;
        }
        let mut digits = 0;
        while current > 0 {
            current /= 10;
            digits += 1;
        }
        digits
    }

    fn split_in_two(&self) -> (Rock, Rock) {
        let digits = self.digits();
        let divider = 10u64.pow(digits / 2);
        let left_number = self.number / divider;
        let right_number = self.number % divider;
        (
            Rock {
                number: left_number,
            },
            Rock {
                number: right_number,
            },
        )
    }
}

#[derive(Clone)]
struct Rocks {
    rocks: Vec<Rock>,
}

type Cache = HashMap<(u64, u64), u64>;

impl Rocks {
    fn blink_recursive(&self, rock: Rock, blinks: u64, blink: u64, cache: &mut Cache) -> u64 {
        if blink == blinks {
            return 1;
        }

        let key = (blink, rock.number);

        if cache.contains_key(&key) {
            return cache[&key];
        }

        if rock.number == 0 {
            let rock = Rock { number: 1 };
            let result = self.blink_recursive(rock, blinks, blink + 1, cache);
            cache.insert(key, result);
            result
        } else if rock.digits() % 2 == 0 {
            let (left, right) = rock.split_in_two();
            let result_left = self.blink_recursive(left, blinks, blink + 1, cache);
            let result_right = self.blink_recursive(right, blinks, blink + 1, cache);
            let sum = result_left + result_right;
            cache.insert(key, sum);
            sum
        } else {
            let rock = Rock {
                number: rock.number * 2024,
            };
            let result = self.blink_recursive(rock, blinks, blink + 1, cache);
            cache.insert(key, result);
            result
        }
    }

    fn blink_tree(&self, blinks: u64) -> u64 {
        let mut cache = Cache::new();
        self.rocks
            .iter()
            .map(move |rock| self.blink_recursive(*rock, blinks, 0, &mut cache))
            .sum()
    }

    fn blink(&mut self) {
        let mut i = 0;
        while i < self.rocks.len() {
            let current_rock = &mut self.rocks[i];
            if current_rock.number == 0 {
                current_rock.number = 1;
            } else if current_rock.digits() % 2 == 0 {
                let (left, right) = current_rock.split_in_two();
                self.rocks[i] = left;
                self.rocks.insert(i + 1, right);
                i += 1;
            } else {
                current_rock.number *= 2024;
            }

            i += 1;
        }
    }

    fn len(&self) -> usize {
        self.rocks.len()
    }
}

fn puzzle_1(rocks: &Rocks) {
    let mut rocks = rocks.clone();
    let blinks = 25;
    for _ in 0..blinks {
        rocks.blink();
    }
    println!("After {blinks} blinks there will be {} rocks", rocks.len())
}

fn puzzle_2(rocks: &Rocks) {
    let blinks = 75;
    let result = rocks.blink_tree(blinks);
    println!("After {blinks} blinks there will be {} rocks", result)
}

fn main() {
    let file_content = read_to_string("input.txt").unwrap();
    let rocks = file_content
        .split(" ")
        .map(|rock| Rock {
            number: rock.parse().unwrap(),
        })
        .collect::<Vec<Rock>>();
    let rocks = Rocks { rocks };
    benchmark(|| puzzle_1(&rocks), TimeUnit::MICROSECONDS);
    benchmark(||puzzle_2(&rocks), TimeUnit::MICROSECONDS);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_split_in_two() {
        let rock = Rock { number: 1234 };
        let (left, right) = rock.split_in_two();
        assert_eq!(left.number, 12);
        assert_eq!(right.number, 34);
    }

    #[test]
    fn should_calculate_digits() {
        assert_eq!(Rock { number: 1234 }.digits(), 4);
        assert_eq!(Rock { number: 234 }.digits(), 3);
        assert_eq!(Rock { number: 1 }.digits(), 1);
        assert_eq!(Rock { number: 0 }.digits(), 1);
    }
}
