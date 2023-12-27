use std::borrow::Borrow;

use super::almanac::Almanac;
use super::builder::AlmanacBuilder;
use super::error::AlmanacError;
use crate::map::MapBuilder;
use crate::range::Range;
use file_reader::read_file;

enum Mode {
    Map(String, String),
    None,
}

pub struct AlmanacFileSource {
    file: String,
    mode: Mode,
}

impl AlmanacFileSource {
    pub fn new(file: &str) -> AlmanacFileSource {
        AlmanacFileSource {
            file: file.to_string(),
            mode: Mode::None,
        }
    }

    pub fn read_from_file(&mut self) -> Result<Almanac, AlmanacError> {
        let lines = read_file(self.file.as_str())?;
        let mut builder = AlmanacBuilder::new();
        for line in lines {
            self.parse_next_line(&mut builder, line?)?;
        }
        Ok(builder.build())
    }

    fn parse_next_line(
        &mut self,
        builder: &mut AlmanacBuilder,
        line: String,
    ) -> Result<(), AlmanacError> {
        match self.mode.borrow() {
            Mode::Map(source, target) => {
                if line == "" {
                    self.mode = Mode::None;
                    return Ok(());
                }
                let range = self.parse_map_range(line)?;
                return builder.add_map_range(source.as_str(), target.as_str(), range);
            }
            Mode::None => {
                if line.starts_with("seeds:") {
                    builder.set_seeds(self.parse_seeds(line)?)
                } else if line.ends_with(" map:") {
                    let map_name = line.strip_suffix(" map:").ok_or_else(|| AlmanacError {
                        message: format!(
                            "Invalid map name: \"{}\", should have \" map:\" at the end",
                            line
                        ),
                    })?;
                    let (from, to) = map_name.split_once("-to-").ok_or_else(|| AlmanacError {
                        message: format!(
                            "Invalid map name: \"{}\", should have '-to-' in the middle",
                            map_name
                        ),
                    })?;
                    self.mode = Mode::Map(from.to_string(), to.to_string());
                    let map_builder = MapBuilder::new(from, to);
                    builder.add_new_map(map_builder)
                } else if line == "" {
                    Ok(())
                } else {
                    Err(AlmanacError {
                        message: format!("Unable to continue parsing. The line has some content but it's not a seed line, not a map name and neither map content. Line: \"{}\"", line),
                    })
                }
            }
        }
    }

    fn parse_map_range(&self, line: String) -> Result<Range, AlmanacError> {
        let elements: Vec<&str> = line.splitn(3, " ").collect();
        if elements.len() != 3 {
            return Err(AlmanacError {
                message: format!("Invalid map range line, it should contain three numbers separated with space: \"{}\"", line),
            });
        }
        Ok(Range {
            destination_range_start: elements[0].parse::<u64>()?,
            source_range_start: elements[1].parse::<u64>()?,
            range_size: elements[2].parse::<u64>()?,
        })
    }

    fn parse_seeds(&self, line: String) -> Result<Vec<u64>, AlmanacError> {
        let (separator_index, _) = line
            .chars()
            .enumerate()
            .find(|(_, c)| *c == ':')
            .ok_or_else(|| AlmanacError {
                message: format!("No separator ':' found in seeds line: \"{}\"", line),
            })?;

        line.chars()
            .skip(separator_index + 1)
            .collect::<String>()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|seed_str| {
                seed_str.parse::<u64>().map_err(|err| AlmanacError {
                    message: format!("Invalid seed value {} {}", seed_str, err),
                })
            })
            .collect()
    }
}
