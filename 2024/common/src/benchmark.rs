use std::time::SystemTime;

pub enum TimeUnit {
    NANOSECONDS,
    MILLISECONDS,
    MICROSECONDS,
    SECONDS,
}

impl TimeUnit {
    fn short_name(&self) -> &str {
        match self {
            TimeUnit::NANOSECONDS => "ns",
            TimeUnit::MILLISECONDS => "ms",
            TimeUnit::MICROSECONDS => "μs",
            TimeUnit::SECONDS => "s",
        }
    }
}

pub fn benchmark<T>(benchmarked: T, unit: TimeUnit)
where
    T: Fn() -> (),
{
    let start = SystemTime::now();
    benchmarked();
    let elapsed = start.elapsed().unwrap();
    let elapsed = match unit {
        TimeUnit::NANOSECONDS => elapsed.as_nanos(),
        TimeUnit::MILLISECONDS => elapsed.as_millis(),
        TimeUnit::MICROSECONDS => elapsed.as_micros(),
        TimeUnit::SECONDS => elapsed.as_secs() as u128,
    };
    println!("It took {elapsed}{} to execute benchmarked function", unit.short_name());
}
