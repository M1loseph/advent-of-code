use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone)]
enum NumericPadKeys {
    A,
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    Gap,
}

impl NumericPadKeys {
    fn position(&self) -> (usize, usize) {
        match self {
            Self::A => (2, 3),
            Self::_0 => (1, 3),
            Self::_1 => (0, 2),
            Self::_2 => (1, 2),
            Self::_3 => (2, 2),
            Self::_4 => (0, 1),
            Self::_5 => (1, 1),
            Self::_6 => (2, 1),
            Self::_7 => (0, 0),
            Self::_8 => (1, 0),
            Self::_9 => (2, 0),
            Self::Gap => (0, 3),
        }
    }

    fn to_char(&self) -> char {
        match self {
            NumericPadKeys::A => 'A',
            NumericPadKeys::_0 => '0',
            NumericPadKeys::_1 => '1',
            NumericPadKeys::_2 => '2',
            NumericPadKeys::_3 => '3',
            NumericPadKeys::_4 => '4',
            NumericPadKeys::_5 => '5',
            NumericPadKeys::_6 => '6',
            NumericPadKeys::_7 => '7',
            NumericPadKeys::_8 => '8',
            NumericPadKeys::_9 => '9',
            NumericPadKeys::Gap => 'G',
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum DirectionPadKeys {
    Left,
    Right,
    Up,
    Down,
    A,
    Gap,
}

impl DirectionPadKeys {
    fn position(&self) -> (usize, usize) {
        match self {
            Self::Left => (0, 1),
            Self::Right => (2, 1),
            Self::Up => (1, 0),
            Self::Down => (1, 1),
            Self::A => (2, 0),
            Self::Gap => (0, 0),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Left => '<',
            Self::Right => '>',
            Self::Up => '^',
            Self::Down => 'v',
            Self::A => 'A',
            Self::Gap => 'G',
        }
    }
}

fn diff(from: (usize, usize), to: (usize, usize)) -> (isize, isize) {
    let (fx, fy) = from;
    let (tx, ty) = to;
    (tx as isize - fx as isize, ty as isize - fy as isize)
}

fn equals_forbidden_point(from: (usize, usize), delta: (isize, isize), forbidden: (usize, usize)) -> bool {
    let result = (from.0.checked_add_signed(delta.0).unwrap(), from.1.checked_add_signed(delta.1).unwrap());
    result == forbidden
}

fn buttons_to_move_path(
    from: (usize, usize),
    to: (usize, usize),
    forbidden_point: (usize, usize),
) -> Vec<Vec<DirectionPadKeys>> {
    let (dx, dy) = diff(from, to);
    let vertical_button = if dy > 0 {
        DirectionPadKeys::Down
    } else {
        DirectionPadKeys::Up
    };
    let horizontal_button = if dx > 0 {
        DirectionPadKeys::Right
    } else {
        DirectionPadKeys::Left
    };
    let vertical = vec![vertical_button; dy.abs() as usize];
    let horizontal = vec![horizontal_button; dx.abs() as usize];
    if dx != 0 && dy != 0 {
        let mut result = Vec::new();
        if !equals_forbidden_point(from, (dx, 0), forbidden_point) {
            let mut horizontal_first = Vec::new();
            horizontal_first.extend(horizontal.iter());
            horizontal_first.extend(vertical.iter());
            horizontal_first.push(DirectionPadKeys::A);
            result.push(horizontal_first);
        }
        if !equals_forbidden_point(from, (0, dy), forbidden_point) {
            let mut vertical_first = Vec::new();
            vertical_first.extend(vertical.iter());
            vertical_first.extend(horizontal.iter());
            vertical_first.push(DirectionPadKeys::A);
            result.push(vertical_first);
        }
        result
    } else {
        let mut result = Vec::new();
        result.extend(horizontal.iter());
        result.extend(vertical.iter());
        result.push(DirectionPadKeys::A);

        vec![result]
    }
}

fn moves_required(code: &Vec<NumericPadKeys>, robots_between: usize) -> usize {
    fn recurr(
        robots_between: usize,
        nth_robot: usize,
        from: (usize, usize),
        to: (usize, usize),
        cache: &mut HashMap<(usize, (usize, usize), (usize, usize)), usize>
    ) -> usize {
        let key = (nth_robot, from, to);
        if let Some(result) = cache.get(&key) {
            return *result;
        }
        let forbidden_point = if nth_robot == 0 {
            NumericPadKeys::Gap.position()
        } else {
            DirectionPadKeys::Gap.position()
        };
        let all_possible_moves = buttons_to_move_path(from, to, forbidden_point);
        if robots_between == nth_robot {
            // min should not be required, all paths have equal length
            return all_possible_moves
                .iter()
                .map(|moves| moves.len())
                .min()
                .unwrap();
        }
        let result = all_possible_moves
            .into_iter()
            .map(|mut moves| {
                assert_ne!(moves.len(), 0);
                let mut accumulator = 0;
                moves.insert(0, DirectionPadKeys::A);
                for i in 0..moves.len() - 1 {
                    let current_position = moves[i];
                    let next_position = moves[i + 1];
                    accumulator += recurr(
                        robots_between,
                        nth_robot + 1,
                        current_position.position(),
                        next_position.position(),
                        cache
                    );
                }
                accumulator
            })
            .min()
            .unwrap();
        cache.insert(key, result);
        result
    }

    let code: Vec<NumericPadKeys> = vec![NumericPadKeys::A]
        .into_iter()
        .chain(code.clone().into_iter())
        .collect();
    let mut acc = 0;
    for i in 0..code.len() - 1 {
        acc += recurr(
            robots_between,
            0,
            code[i].position(),
            code[i + 1].position(),
            &mut HashMap::new()
        );
    }
    acc
}

fn sum_complexities(codes: &Vec<Vec<NumericPadKeys>>, robots_between: usize) -> u64 {
    let moves: Vec<usize> = codes
        .iter()
        .map(|code| moves_required(code, robots_between))
        .collect();
    codes
        .iter()
        .zip(moves.iter())
        .map(|(code, solution_size)| {
            let code: String = code.iter().map(|code| code.to_char()).collect();
            println!("{} {}", code, solution_size);
            let code = code.replace("A", "").parse::<u64>().unwrap();
            code * (*solution_size) as u64
        })
        .sum()
}

fn puzzle_1(codes: &Vec<Vec<NumericPadKeys>>) {
    let puzzle_result = sum_complexities(codes, 2);
    println!("Sum of complexieties is {puzzle_result}");
}

fn puzzle_2(codes: &Vec<Vec<NumericPadKeys>>) {
    let puzzle_result = sum_complexities(codes, 25);
    println!("Sum of complexieties is {puzzle_result}");
}

fn main() {
    let file_content = read_to_string("input.txt").unwrap();
    let codes: Vec<Vec<NumericPadKeys>> = file_content
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    'A' => NumericPadKeys::A,
                    '0' => NumericPadKeys::_0,
                    '1' => NumericPadKeys::_1,
                    '2' => NumericPadKeys::_2,
                    '3' => NumericPadKeys::_3,
                    '4' => NumericPadKeys::_4,
                    '5' => NumericPadKeys::_5,
                    '6' => NumericPadKeys::_6,
                    '7' => NumericPadKeys::_7,
                    '8' => NumericPadKeys::_8,
                    '9' => NumericPadKeys::_9,
                    _ => panic!("Unexpected char {char}"),
                })
                .collect()
        })
        .collect();
    puzzle_1(&codes);
    puzzle_2(&codes);
}
