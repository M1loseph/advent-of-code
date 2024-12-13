use std::fs::read_to_string;

#[derive(Debug)]
struct Game {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
}

#[derive(PartialEq)]
enum Button {
    A { tried_a: bool, tried_b: bool },
    B { tried_b: bool },
}

const A_COST: u64 = 3;
const B_COST: u64 = 1;

impl Game {
    fn recur(&self, x: u64, y: u64, cost: u64, solutions: &mut Vec<u64>, try_with_a: bool) {
        if (x, y) == self.prize {
            solutions.push(cost);
            return;
        }
        let (tx, ty) = self.prize;
        if tx < x || ty < y {
            return;
        }
        let (xa, ya) = self.a;
        let (xb, yb) = self.b;
        if try_with_a {
            self.recur(x + xa, y + ya, cost + 3, solutions, true);
        }
        self.recur(x + xb, y + yb, cost + 1, solutions, false);
    }

    fn smallest_number_of_tokens_recursive(&self) -> Option<u64> {
        let mut solutions = Vec::new();
        self.recur(0, 0, 0, &mut solutions, true);
        solutions.into_iter().min()
    }

    // Unused implementation - I thought that using iterations instead of recursion would solve the problem.
    // It didn't.
    fn smallest_number_of_tokens_iterative(&self) -> Option<u64> {
        let mut solutions = Vec::new();
        let mut stack = Vec::new();

        let mut cost = 0;
        let mut x = 0;
        let mut y = 0;

        self.push(Button::A {
            tried_a: false,
            tried_b: false,
        }, &mut stack, &mut cost, &mut x, &mut y );

        while !stack.is_empty() {
            if (x, y) == self.prize {
                solutions.push(cost);
                self.pop(&mut stack, &mut cost, &mut x, &mut y);
                continue;
            }
            let (tx, ty) = self.prize;
            if tx < x || ty < y {
                self.pop(&mut stack, &mut cost, &mut x, &mut y);
                continue;
            }
            match stack.last_mut().unwrap() {
                Button::A { tried_a, tried_b } => {
                    if !*tried_a {
                        *tried_a = true;
                        self.push(Button::A {
                            tried_a: false,
                            tried_b: false,
                        }, &mut stack, &mut cost, &mut x, &mut y);
                    } else if !*tried_b {
                        *tried_b = true;
                        self.push(Button::B {
                            tried_b: false,
                        }, &mut stack, &mut cost, &mut x, &mut y);
                    } else {
                        self.pop(&mut stack, &mut cost, &mut x, &mut y);
                    }
                }
                Button::B { tried_b } => {
                    if !*tried_b {
                        *tried_b = true;
                        self.push(Button::B { tried_b: false }, &mut stack, &mut cost, &mut x, &mut y);
                    } else {
                        self.pop(&mut stack, &mut cost, &mut x, &mut y);
                    }
                }
            }
        }

        solutions.into_iter().min()
    }

    fn pop(&self, stack: &mut Vec<Button>, cost: &mut u64, x: &mut u64, y: &mut u64) {
        match stack.pop().unwrap() {
            Button::A { .. } => {
                *cost -= A_COST;
                let (xa, ya) = self.a;
                *x -= xa;
                *y -= ya;
            }
            Button::B { .. } => {
                *cost -= B_COST;
                let (xb, yb) = self.b;
                *x -= xb;
                *y -= yb;
            }
        };
    }

    fn push(&self, button: Button, stack: &mut Vec<Button>, cost: &mut u64, x: &mut u64, y: &mut u64) {
        match button {
            Button::A { .. } => {
                *cost += A_COST;
                let (xa, ya) = self.a;
                *x += xa;
                *y += ya;
            }
            Button::B { .. } => {
                *cost += B_COST;
                let (xb, yb) = self.b;
                *x += xb;
                *y += yb;
            }
        }
        stack.push(button);
    }

    fn smallest_number_of_tokens_algebra(&self) {

    }

    fn fix_unit(&mut self) {
        let adjustement = 10000000000000;
        let (xp, yp) = self.prize;
        self.prize = (xp + adjustement, yp + adjustement);
    }
}

struct GameBuilder {
    a: Option<(u64, u64)>,
    b: Option<(u64, u64)>,
    prize: Option<(u64, u64)>,
}

impl GameBuilder {
    fn new() -> Self {
        GameBuilder {
            a: None,
            b: None,
            prize: None,
        }
    }

    fn set_a(&mut self, a: (u64, u64)) -> &mut Self {
        self.a = Some(a);
        self
    }

    fn set_b(&mut self, b: (u64, u64)) -> &mut Self {
        self.b = Some(b);
        self
    }

    fn set_prize(&mut self, prize: (u64, u64)) -> &mut Self {
        self.prize = Some(prize);
        self
    }

    fn build(self) -> Game {
        Game {
            a: self.a.expect("Button A was never set"),
            b: self.b.expect("Button B was never set"),
            prize: self.prize.expect("Prize was never set"),
        }
    }

    fn parse_line(&mut self, line: &str) {
        fn parse_numeric(str: &str) -> u64 {
            str.trim()
                .chars()
                .into_iter()
                .skip(2)
                .collect::<String>()
                .parse()
                .unwrap()
        }

        if line.starts_with("Button A") {
            let (left, right) = line.split_once(":").unwrap().1.split_once(",").unwrap();
            let a = (parse_numeric(left), parse_numeric(right));
            self.set_a(a);
        } else if line.starts_with("Button B") {
            let (left, right) = line.split_once(":").unwrap().1.split_once(",").unwrap();
            let b = (parse_numeric(left), parse_numeric(right));
            self.set_b(b);
        } else if line.starts_with("Prize") {
            let (left, right) = line.split_once(":").unwrap().1.split_once(",").unwrap();
            let prize = (parse_numeric(left), parse_numeric(right));
            self.set_prize(prize);
        } else {
            panic!("Unknown line {line}");
        }
    }
}

fn puzzle_1(games: &Vec<Game>) {
    let total: u64 = games
        .iter()
        .filter_map(|game| game.smallest_number_of_tokens_recursive())
        .sum();
    println!("Sum of smalles possible solutions is {total}");
}

fn puzzle_2(games: &mut Vec<Game>) {
    let total: u64 = games
        .into_iter()
        .map(|game| {
            game.fix_unit();
            game
        })
        .filter_map(|game| game.smallest_number_of_tokens_iterative())
        .sum();
    println!("Sum of smalles possible solutions is {total}");
}

fn main() {
    let file_content = read_to_string("input.txt").unwrap();
    let mut games = Vec::new();
    let mut game_builder = GameBuilder::new();
    for line in file_content.lines() {
        if line.is_empty() {
            games.push(game_builder.build());
            game_builder = GameBuilder::new();
            continue;
        }
        game_builder.parse_line(line);
    }
    games.push(game_builder.build());
    puzzle_1(&games);
    puzzle_2(&mut games);
}
