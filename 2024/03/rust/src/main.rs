use regex::Regex;
use std::fs::read_to_string;

fn puzzle_1(instructions: &str) {
    let regex = Regex::new(r##"mul\(\d{1,3},\d{1,3}\)"##).unwrap();
    let found_instructions = regex.find_iter(&instructions);

    let result = found_instructions
        .map(|instruction| {
            let as_string = instruction.as_str();
            let (left_number, right_number) =
                as_string[4..as_string.len() - 1].split_once(",").unwrap();
            return left_number.parse::<i64>().unwrap() * right_number.parse::<i64>().unwrap();
        })
        .sum::<i64>();

    println!("First part result {result}");
}

fn puzzle_2(instructions: &str) {
    let regex = Regex::new(r##"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don\'t\(\))"##).unwrap();
    let found_instructions = regex.find_iter(&instructions);

    let mut enabled = true;
    let mut accumulator = 0i64;

    for instruction in found_instructions {
        match instruction.as_str() {
            "don't()" => {
                enabled = false;
            }
            "do()" => {
                enabled = true;
            }
            _ => {
                if !enabled {
                    continue;
                }
                let as_string = instruction.as_str();
                let (left_number, right_number) =
                    as_string[4..as_string.len() - 1].split_once(",").unwrap();
                accumulator +=
                    left_number.parse::<i64>().unwrap() * right_number.parse::<i64>().unwrap();
            }
        }
    }
    println!("Second part result {accumulator}");
}

fn main() {
    let instructions = read_to_string("input.txt").unwrap();
    puzzle_1(&instructions);
    puzzle_2(&instructions);
}
