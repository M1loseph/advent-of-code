use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::Lines;
use std::result::Result;

pub fn read_input_file(file: &str) -> Result<Lines<BufReader<File>>, Error> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}
