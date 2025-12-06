use std::fs::read_to_string;

#[derive(Clone, Copy)]
enum Operator {
    Mul,
    Add,
}

struct Problem {
    operator: Operator,
    arguments: Vec<i64>,
}

fn parse_file_puzzle2() -> Vec<Problem> {
    fn transpose(matrix: Vec<Vec<char>>) -> Vec<String> {
        let mut transposed = vec![];
        for x in (0..matrix[0].len()).rev() {
            let mut new_string = String::new();
            for y in 0..matrix.len() {
                new_string.push(matrix[y][x]);
            }
            transposed.push(new_string);
        }
        transposed
    }

    let lines = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let transposed = transpose(lines);

    let mut problems = vec![];
    let mut arguments = vec![];
    let mut operator = None;
    for line in transposed.iter().chain(vec!["".to_string()].iter()) {
        let mut line = line.trim();
        if line.is_empty() {
            problems.push(Problem {
                operator: operator.unwrap(),
                arguments: arguments,
            });
            operator = None;
            arguments = vec![];
            continue;
        }
        operator = match line.chars().last().unwrap() {
            '*' => Some(Operator::Mul),
            '+' => Some(Operator::Add),
            _ => None,
        };
        if operator.is_some() {
            let mut chars = line.chars();
            chars.next_back();
            line = chars.as_str().trim();
        }
        arguments.push(line.parse().unwrap());
    }
    problems
}

fn parse_file_puzzle1() -> Vec<Problem> {
    let file_content = read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = file_content.lines().collect();
    let mut arguments = vec![];
    let mut operators = vec![];
    for i in 0..lines.len() {
        let line = lines[i];
        if i == lines.len() - 1 {
            operators = line
                .split(" ")
                .filter(|e| !e.is_empty())
                .map(|e| match e {
                    "*" => Operator::Mul,
                    "+" => Operator::Add,
                    _ => panic!("Unsupported operator {e}"),
                })
                .collect();
            continue;
        }
        arguments.push(
            line.split(" ")
                .filter(|e| !e.is_empty())
                .map(|e| e.parse().unwrap())
                .collect::<Vec<i64>>(),
        );
    }
    let mut problems = vec![];
    for i in 0..operators.len() {
        let arguments = arguments.iter().map(|args| args[i]).collect();
        problems.push(Problem {
            operator: operators[i],
            arguments,
        });
    }
    problems
}

fn sum_problems(problems: &Vec<Problem>) -> i64 {
    let mut sum = 0;
    for problem in problems {
        let result = match problem.operator {
            Operator::Mul => problem.arguments.iter().fold(1, |acc, e| acc * e),
            Operator::Add => problem.arguments.iter().fold(0, |acc, e| acc + e),
        };
        sum += result;
    }
    sum
}

fn main() {
    let problems = parse_file_puzzle1();
    let sum = sum_problems(&problems);
    println!("[Puzzle 1] Result: {sum}");

    let problems = parse_file_puzzle2();
    let sum = sum_problems(&problems);
    println!("[Puzzle 2] Result: {sum}");
}
