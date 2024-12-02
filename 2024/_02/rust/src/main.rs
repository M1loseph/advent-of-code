use std::fs::read_to_string;
use std::time::SystemTime;

fn measure_execution_time<F>(closure: F)
where
    F: Fn() -> (),
{
    let begin = SystemTime::now();
    closure();
    let end = SystemTime::now();
    let diff = end.duration_since(begin).unwrap();
    println!("It took {} ns to execute the closure", diff.as_nanos());
}

fn puzzle_1(reports: &Vec<Vec<i64>>) {
    let safe_reports = reports
        .iter()
        .map(|report| {
            let mut increasing = false;

            for i in 0..report.len() - 1 {
                let diff = (report[i] - report[i + 1]).abs();
                if diff == 0 || diff > 3 {
                    return false;
                }

                if i == 0 {
                    if report[i] < report[i + 1] {
                        increasing = true;
                    }
                    continue;
                }
                if increasing {
                    if report[i] > report[i + 1] {
                        return false;
                    }
                } else {
                    if report[i] < report[i + 1] {
                        return false;
                    }
                }
            }
            return true;
        })
        .filter(|safe| *safe)
        .count();

    println!("There are {safe_reports} safe reports");
}

fn puzzle_2(reports: &Vec<Vec<i64>>) {
    fn safe_report_algorith(report: &[i64]) -> bool {
        let mut increasing = false;
        for i in 0..report.len() - 1 {
            let diff = (report[i] - report[i + 1]).abs();
            if diff == 0 || diff > 3 {
                return false;
            }

            if i == 0 {
                if report[i] < report[i + 1] {
                    increasing = true;
                }
                continue;
            }
            if increasing {
                if report[i] > report[i + 1] {
                    return false;
                }
            } else {
                if report[i] < report[i + 1] {
                    return false;
                }
            }
        }
        return true;
    }
    let safe_reports = reports
        .iter()
        .map(|report| {
            if safe_report_algorith(&report) {
                return true;
            }
            for i in 0..report.len() {
                let filtered_report = report
                    .iter()
                    .enumerate()
                    .filter_map(|(j, e)| {
                        return if j == i { None } else { Some(*e) };
                    })
                    .collect::<Vec<i64>>();
                if safe_report_algorith(&filtered_report) {
                    return true;
                }
            }
            return false;
        })
        .filter(|safe| *safe)
        .count();

    println!("There are {safe_reports} safe reports when applying tolerance");
}

fn main() {
    let file_content = read_to_string("input.txt").unwrap();

    let reports = file_content
        .lines()
        .map(|line| {
            return line
                .split(" ")
                .filter(|report_part| !report_part.is_empty())
                .map(|report_part| report_part.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
        })
        .collect::<Vec<Vec<i64>>>();

    measure_execution_time(|| puzzle_1(&reports));
    measure_execution_time(|| puzzle_2(&reports));
}
