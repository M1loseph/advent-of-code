use std::fs::read_to_string;

enum Direction {
    Left,
    Right,
}

struct DialMove {
    direction: Direction,
    value: i64,
}

fn puzzle_1(moves: &Vec<DialMove>) {
    let mut dial_position = 50i64;
    let mut points_at_zero = 0;
    for dial_move in moves {
        let direction = &dial_move.direction;
        let value = dial_move.value;

        match direction {
            Direction::Right => dial_position += value,
            Direction::Left => dial_position -= value,
        };
        dial_position = dial_position % 100;
        if dial_position == 0 {
            points_at_zero += 1;
        }
        if dial_position < 0 {
            dial_position += 100;
        }
    }
    println!("Puzzle 1 result: {}", points_at_zero);
}

fn puzzle_2(moves: &Vec<DialMove>) {
    let mut dial_position = 50i64;
    let mut points_at_zero = 0;

    for dial_move in moves {
        let direction = &dial_move.direction;
        let value = dial_move.value;

        for _ in 0..value {
            match direction {
                Direction::Right => dial_position += 1,
                Direction::Left => dial_position -= 1,
            };
            dial_position = dial_position % 100;
            if dial_position < 0 {
                dial_position += 100;
            }
            if dial_position == 0 {
                points_at_zero += 1;
            }
        }
    }
    println!("Puzzle 2 result: {}", points_at_zero);
}

fn main() {
    let file_content = read_to_string("input.txt").unwrap();
    let moves: Vec<DialMove> = file_content
        .lines()
        .map(|line| {
            let direction = line.chars().next().unwrap();
            let direction = match direction {
                'R' => Direction::Right,
                'L' => Direction::Left,
                _ => panic!("Unknown direction {}", direction),
            };
            let value = line
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<i64>()
                .unwrap();
            DialMove { direction, value }
        })
        .collect();

    puzzle_1(&moves);
    puzzle_2(&moves);
}
