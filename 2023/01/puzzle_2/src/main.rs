use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn find_first_digit(string: &String, digit_names: &HashMap<&str, u32>) -> u32 {
    let (mut position, mut current_value) = string
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_digit(10))
        .map(|(i, c)| (i, c.to_digit(10).unwrap()))
        .next()
        .unwrap();

    for (name, value) in digit_names {
        if let Some(name_position) = string.find(name) {
            if name_position < position {
                position = name_position;
                current_value = *value;
            }
        }
    }
    current_value
}

fn find_last_digit(string: &String, digit_names: &HashMap<&str, u32>) -> u32 {
    let (mut position, mut current_value) = string
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_digit(10))
        .map(|(i, c)| (i, c.to_digit(10).unwrap()))
        .last()
        .unwrap();

    for (name, value) in digit_names {
        if let Some(name_position) = string.rfind(name) {
            if name_position > position {
                position = name_position;
                current_value = *value;
            }
        }
    }
    current_value
}

fn main() {
    let file = match File::open("input/input.txt") {
        Ok(result) => result,
        Err(_) => panic!("Should have opened a file!"),
    };

    let digit_names = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let sum: u32 = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let first_digit = find_first_digit(&line, &digit_names);
            let last_digit = find_last_digit(&line, &digit_names);
            println!("{line} -> {first_digit}{last_digit}");
            first_digit * 10 + last_digit
        })
        .sum();
    println!("===================================");
    println!("Sum -> {sum}");
}
