use std::{
    collections::HashMap,
    fs::read_to_string,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum TowelColor {
    White,
    Blue,
    Black,
    Red,
    Green,
}

fn printable(colors: &Vec<TowelColor>) -> String {
    colors.iter().map(|color| {
        match color {
            TowelColor::White => 'w',
            TowelColor::Blue => 'u',
            TowelColor::Black => 'b',
            TowelColor::Red => 'r',
            TowelColor::Green => 'g',
        }
    }).collect()
}

impl TowelColor {
    fn from_char(char: char) -> Self {
        match char {
            'w' => Self::White,
            'u' => Self::Blue,
            'b' => Self::Black,
            'r' => Self::Red,
            'g' => Self::Green,
            _ => panic!("Unexpected character {}", char),
        }
    }
}

type Pattern = Vec<TowelColor>;

fn starts_with(pattern: &Vec<TowelColor>, current_pattern: &Vec<TowelColor>) -> bool {
    &pattern[0..current_pattern.len()] == &current_pattern[..]
}

fn count_towels_arrangements(
    pattern: &Pattern,
    towels: &Vec<Pattern>,
    current_pattern: &mut Vec<Pattern>,
    cache: &mut HashMap<Pattern, u64>,
) -> u64 {
    let mut accumulator = 0;
    for towel in towels {
        current_pattern.push(towel.clone());
        let flattened = current_pattern.concat();
        if flattened.len() <= pattern.len() {
            match cache.get(&flattened) {
                Some(value) => {
                    accumulator += *value;
                }
                None => {
                    if flattened == *pattern {
                        accumulator += 1;
                    } else if starts_with(pattern, &flattened) {
                        accumulator += count_towels_arrangements(
                            pattern,
                            towels,
                            current_pattern,
                            cache,
                        );
                    }
                }
            }
        }
        current_pattern.pop();
    }
    cache.insert(current_pattern.concat(), accumulator);
    accumulator
}

fn puzzle_1(towels: &Vec<Pattern>, patterns: &Vec<Pattern>) {
    let possible_designs = patterns
        .iter()
        .map(|pattern| {
            let result =
                count_towels_arrangements(pattern, towels, &mut Vec::new(), &mut HashMap::new());
            if result > 0 {
                println!("✅ {}", printable(pattern));
            } else {
                println!("❌ {:?}", printable(pattern));
            }
            result
        })
        .filter(|result| *result > 0)
        .count();
    println!("{possible_designs} can be made from towels");
}

fn puzzle_2(towels: &Vec<Pattern>, patterns: &Vec<Pattern>) {
    let all_possible_designs = patterns
        .iter()
        .map(|pattern| {
            let result =
                count_towels_arrangements(pattern, towels, &mut Vec::new(), &mut HashMap::new());
            if result > 0 {
                println!("{result:<16} {}", printable(pattern));
            } else {
                println!("{result:<16} {}", printable(pattern));
            }
            result
        })
        .sum::<u64>();
    println!("{all_possible_designs} can be made from towels");
}

fn main() {
    let file_content = read_to_string("input.txt").unwrap();
    let mut empty_line_spotted = false;
    let mut towels: Vec<Pattern> = Vec::new();
    let mut patterns: Vec<Pattern> = Vec::new();
    file_content.lines().for_each(|line| {
        if !empty_line_spotted {
            if line.is_empty() {
                empty_line_spotted = true;
                return;
            }
            towels.append(
                &mut line
                    .split(",")
                    .map(|pattern| {
                        pattern
                            .trim()
                            .chars()
                            .map(|char| TowelColor::from_char(char))
                            .collect()
                    })
                    .collect(),
            );
        } else {
            patterns.push(
                line.chars()
                    .map(|char| TowelColor::from_char(char))
                    .collect(),
            );
        }
    });
    puzzle_1(&towels, &patterns);
    puzzle_2(&towels, &patterns);
}
