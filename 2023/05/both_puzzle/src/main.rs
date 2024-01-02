mod almanac;
mod error;
mod file_source;
mod map;
mod range;
mod seeds;

use file_source::AlmanacFileSource;

fn puzzle_1() {
    let almanac = match AlmanacFileSource::seed_list("input/input.txt").read_from_file() {
        Ok(almanac) => almanac,
        Err(err) => {
            println!("Error: {}", err);
            std::process::exit(1);
        }
    };
    let location = almanac.find_lowest_location_for_all_seeds().unwrap();
    println!("The lowest location for all seeds is {}", location);
}

fn puzzle_2() {
    let almanac = match AlmanacFileSource::seed_ranges("input/input.txt").read_from_file() {
        Ok(almanac) => almanac,
        Err(err) => {
            println!("Error: {}", err);
            std::process::exit(1);
        }
    };
    let location = almanac.find_lowest_location_for_all_seeds().unwrap();
    println!("The lowest location for all seeds is {}", location);
}

fn main() {
    puzzle_1();
    puzzle_2();
}
