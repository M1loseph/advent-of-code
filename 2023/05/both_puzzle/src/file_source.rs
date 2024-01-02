use super::almanac::Almanac;
use super::error::AlmanacError;
use super::seeds::SeedListSourceStrategy;
use crate::almanac::AlmanacBuilder;
use crate::map::MapBuilder;
use crate::seeds::SeedRangeSourceStrategy;
use file_reader::read_file;

pub trait Strategy {
    /// Returns true if parsing is done, false otherwise
    fn parse_next(&mut self, line: String) -> Result<bool, AlmanacError>;
    fn add_to_builder(&self, builder: &mut AlmanacBuilder) -> Result<(), AlmanacError>;
}

type StrategyFactory = fn(&String) -> Result<Box<dyn Strategy>, AlmanacError>;

pub struct AlmanacFileSource {
    file: String,
    current_strategy: Option<Box<dyn Strategy>>,
    strategy_factory: StrategyFactory,
}

impl AlmanacFileSource {
    pub fn seed_list(file: &str) -> AlmanacFileSource {
        AlmanacFileSource {
            file: file.to_string(),
            current_strategy: None,
            strategy_factory: AlmanacFileSource::create_strategy_seed_list,
        }
    }

    pub fn seed_ranges(file: &str) -> AlmanacFileSource {
        AlmanacFileSource {
            file: file.to_string(),
            current_strategy: None,
            strategy_factory: AlmanacFileSource::create_strategy_seed_ranges,
        }
    }

    pub fn read_from_file(&mut self) -> Result<Almanac, AlmanacError> {
        let lines = read_file(self.file.as_str())?;
        let mut builder = AlmanacBuilder::new();
        for line in lines {
            let line = line?;
            if self.current_strategy.is_none() {
                self.current_strategy = Some((self.strategy_factory)(&line)?);
            }
            if self.current_strategy.as_mut().unwrap().parse_next(line)? {
                self.current_strategy
                    .take()
                    .unwrap()
                    .add_to_builder(&mut builder)?;
            }
        }

        match self.current_strategy.take() {
            Some(strategy) => strategy.add_to_builder(&mut builder)?,
            None => (),
        };

        builder.build()
    }

    fn create_strategy_seed_list(line: &String) -> Result<Box<dyn Strategy>, AlmanacError> {
        if line.starts_with("seeds:") {
            Ok(Box::new(SeedListSourceStrategy::new()))
        } else if line.ends_with(" map:") {
            Ok(Box::new(MapBuilder::new()))
        } else if line == "" {
            Ok(Box::new(NoopStrategy {}))
        } else {
            return Err(AlmanacError {
                message: format!("Cannot determine how to parse line \"{}\"", line),
            });
        }
    }

    fn create_strategy_seed_ranges(line: &String) -> Result<Box<dyn Strategy>, AlmanacError> {
        if line.starts_with("seeds:") {
            Ok(Box::new(SeedRangeSourceStrategy::new()))
        } else if line.ends_with(" map:") {
            Ok(Box::new(MapBuilder::new()))
        } else if line == "" {
            Ok(Box::new(NoopStrategy {}))
        } else {
            return Err(AlmanacError {
                message: format!("Cannot determine how to parse line \"{}\"", line),
            });
        }
    }
}

struct NoopStrategy {}

impl Strategy for NoopStrategy {
    fn parse_next(&mut self, _line: String) -> Result<bool, AlmanacError> {
        Ok(true)
    }

    fn add_to_builder(&self, _builder: &mut AlmanacBuilder) -> Result<(), AlmanacError> {
        Ok(())
    }
}
