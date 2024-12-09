use file_reader::read_file;

struct Report {
    history: Vec<i64>,
}
struct OASIS {
    reports: Vec<Report>,
}

fn read_input(file: &str) -> Result<OASIS, std::io::Error> {
    read_file(file)?.into_iter().try_into()
}

fn main() {
    println!("Hello, world!");
}
