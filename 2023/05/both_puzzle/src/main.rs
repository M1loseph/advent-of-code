mod almanac;
mod map;
mod range;

use almanac::almanac::Almanac;

use crate::almanac::file_source::AlmanacFileSource;

fn puzzle_1(almanac: &Almanac) {
    let location = almanac.find_lowest_location_for_all_seeds().unwrap();
    println!("The lowest location for all seeds is {}", location);
}

fn main() {
    let almanac = match AlmanacFileSource::new("input/input.txt").read_from_file() {
        Ok(almanac) => almanac,
        Err(err) => {
            println!("Error: {}", err);
            std::process::exit(1);
        }
    };
    puzzle_1(&almanac)
}
