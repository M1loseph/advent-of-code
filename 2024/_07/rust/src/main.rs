use std::fs::read_to_string;

struct Equation {
    test_result: i64,
    ingredients: Vec<i64>,
}

enum Operator {
    ADD,
    MULTIPLY,
    CONCATINATE,
}

impl Operator {
    fn apply(&self, lhs: i64, rhs: i64) -> i64 {
        return match self {
            Operator::ADD => lhs + rhs,
            Operator::MULTIPLY => lhs * rhs,
            Operator::CONCATINATE => {
                let mut tmp = rhs.clone();
                let mut digits = 0;
                while tmp > 0 {
                    digits += 1;
                    tmp = tmp / 10;
                }
                return lhs * 10i64.pow(digits) + rhs;
            }
        };
    }
}

impl Equation {
    fn recursive_function(&self, i: usize, acc: i64, operators: &Vec<Operator>) -> i64 {
        if i == self.ingredients.len() {
            return if acc == self.test_result { 1 } else { 0 };
        }
        operators
            .iter()
            .map(|op| self.recursive_function(i + 1, op.apply(acc, self.ingredients[i]), operators))
            .sum()
    }

    fn find_possible_solutions(&self, operators: &Vec<Operator>) -> i64 {
        return self.recursive_function(1, self.ingredients[0], operators);
    }
}

fn puzzle_1(equations: &Vec<Equation>) {
    let operators = vec![Operator::ADD, Operator::MULTIPLY];
    let sum_of_solvable_equtions: i64 = equations
        .iter()
        .filter(|eq| eq.find_possible_solutions(&operators) > 0)
        .map(|eq| eq.test_result)
        .sum();
    println!("Sum of solvable equations is {sum_of_solvable_equtions}")
}

fn puzzle_2(equations: &Vec<Equation>) {
    let operators = vec![Operator::ADD, Operator::MULTIPLY, Operator::CONCATINATE];
    let sum_of_solvable_equtions: i64 = equations
        .iter()
        .filter(|eq| eq.find_possible_solutions(&operators) > 0)
        .map(|eq| eq.test_result)
        .sum();
    println!("Sum of solvable equations when you add concatenation operator is {sum_of_solvable_equtions}")
}

fn main() {
    let file_contant = read_to_string("input.txt").unwrap();
    let mut equations = Vec::new();
    for line in file_contant.lines() {
        let (result, ingreditnts) = line.split_once(":").unwrap();
        let equation = Equation {
            test_result: result.parse().unwrap(),
            ingredients: ingreditnts
                .trim()
                .split(" ")
                .map(|i| i.parse().unwrap())
                .collect(),
        };
        equations.push(equation);
    }
    puzzle_1(&equations);
    puzzle_2(&equations);
}
