use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/// TODO: difference between impl and dyn in the method signature
fn find_first_digit(iter: impl Iterator<Item=char>) -> u32 {
    iter.filter(|c| c.is_digit(10))
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap()
}

fn main() {
    let file = match File::open("input/input.txt") {
        Ok(result) => result,
        Err(_) => panic!("Should have opened a file!"),
    };

    let sum: u32 = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let first_digit = find_first_digit(line.chars());
            let last_digit = find_first_digit(line.chars().rev());
            println!("{} -> {}{}", line, first_digit, last_digit);
            first_digit * 10 + last_digit
        })
        .sum();
    println!("===================================");
    println!("Sum -> {sum}");
}
