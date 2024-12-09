mod race;
use file_reader::read_file;
use race::{Race, RaceBuilder, RaceError};

fn read_races_from_file() -> Result<Vec<Race>, RaceError> {
    let mut lines = read_file("input/input.txt")?;
    let times = lines
        .next()
        .ok_or_else(|| RaceError::new("Missing times line"))??
        .strip_prefix("Time: ")
        .ok_or_else(|| RaceError::new("Missing \"Time: \" prefix"))?
        .to_string();
    let distances = lines
        .next()
        .ok_or_else(|| RaceError::new("Missing distances line"))??
        .strip_prefix("Distance: ")
        .ok_or_else(|| RaceError::new("Missing \"Distance: \" prefix"))?
        .to_string();

    let times_iter = times.split(" ").filter(|time| !time.is_empty());
    let distances_iter = distances.split(" ").filter(|distance| !distance.is_empty());

    let races = times_iter
        .zip(distances_iter)
        .map(|(time, distance)| {
            RaceBuilder::new()
                .with_time_str(time)?
                .with_record_distance_str(distance)?
                .build()
        })
        .collect::<Result<Vec<Race>, RaceError>>()?;
    Ok(races)
}

fn read_single_race_from_file() -> Result<Race, RaceError> {
    let mut lines = read_file("input/input.txt")?;
    let time = lines
        .next()
        .ok_or_else(|| RaceError::new("Missing times line"))??
        .strip_prefix("Time: ")
        .ok_or_else(|| RaceError::new("Missing \"Time: \" prefix"))?
        .replace(" ", "");
    let record_distance = lines
        .next()
        .ok_or_else(|| RaceError::new("Missing distances line"))??
        .strip_prefix("Distance: ")
        .ok_or_else(|| RaceError::new("Missing \"Distance: \" prefix"))?
        .replace(" ", "");

    RaceBuilder::new()
        .with_time_str(&time)?
        .with_record_distance_str(&record_distance)?
        .build()
}

fn puzzle_1() {
    let races = read_races_from_file().unwrap();
    let result = races
        .into_iter()
        .map(|race| race.count_all_winning_button_hold_time())
        .fold(1u64, |acc, result| acc * result);

    println!("Puzzle 1 result: {}", result);
}

fn puzzle_2() {
    let race = read_single_race_from_file().unwrap();
    let result = race.count_all_winning_button_hold_time();

    println!("Puzzle 2 result: {}", result);
}

fn main() {
    puzzle_1();
    puzzle_2();
}
