use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// TODO: maybe try to remove unwrap() from all 'impl' blocks?
#[derive(Debug)]
struct Game {
    game_id: u32,
    noticed_cubes: Vec<NoticesCubes>,
}

#[derive(Debug)]
struct NoticesCubes {
    red: u32,
    blue: u32,
    green: u32,
}

impl Game {
    fn parse_from(input: &str) -> Game {
        let split_input = input
            .strip_prefix("Game ")
            .unwrap()
            .split_once(':')
            .unwrap();

        let game_id = split_input.0.parse::<u32>().unwrap();

        let noticed_cubes = split_input
            .1
            .split(';')
            .map(|game| {
                let game_dictionary: HashMap<&str, u32> = game
                    .split(',')
                    .map(|single_cube_data| single_cube_data.trim())
                    .filter_map(|single_cube_data| single_cube_data.split_once(' '))
                    .map(|(times, color)| (color, times.parse::<u32>().unwrap()))
                    .collect();
                NoticesCubes {
                    red: *game_dictionary.get("red").unwrap_or(&0),
                    blue: *game_dictionary.get("blue").unwrap_or(&0),
                    green: *game_dictionary.get("green").unwrap_or(&0),
                }
            })
            .collect();

        Game {
            game_id,
            noticed_cubes,
        }
    }

    fn get_max_of_each_color(&self) -> (u32, u32, u32) {
        let max_red = self.noticed_cubes.iter().map(|nc| nc.red).max().unwrap();
        let max_blue = self.noticed_cubes.iter().map(|nc| nc.blue).max().unwrap();
        let max_green = self.noticed_cubes.iter().map(|nc| nc.green).max().unwrap();
        (max_red, max_blue, max_green)
    }

    fn is_possible(&self) -> bool {
        let (max_red, max_blue, max_green) = self.get_max_of_each_color();
        max_red <= 12 && max_blue <= 14 && max_green <= 13
    }

    fn calculate_power(&self) -> u32 {
        let (max_red, max_blue, max_green) = self.get_max_of_each_color();
        max_red * max_blue * max_green
    }
}

fn puzzle_1() {
    let file = File::open("input/input.txt").unwrap();
    let result: u32 = BufReader::new(file)
        .lines()
        .filter_map(|c| c.ok())
        .map(|line| Game::parse_from(&line))
        .filter(|game| game.is_possible())
        .map(|game| game.game_id)
        .sum();
    println!("Puzzle 1 result => {result}");
}

fn puzzle_2() {
    let file = File::open("input/input.txt").unwrap();
    let result: u32 = BufReader::new(file)
        .lines()
        .filter_map(|c| c.ok())
        .map(|line| Game::parse_from(&line))
        .map(|game| game.calculate_power())
        .sum();
    println!("Puzzle 2 result => {result}");
}

fn main() {
    puzzle_1();
    puzzle_2();
}
