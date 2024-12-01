use std::fs::read_to_string;

fn puzzle_1() {
    let file_content = read_to_string("input.txt").unwrap();

    let mut left_list = Vec::<u64>::new();
    let mut right_list = Vec::<u64>::new();

    for line in file_content.lines() {
        let (left, right) = line.split_once("   ").unwrap();

        left_list.push(left.parse().unwrap());
        right_list.push(right.parse().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let mut accumulator = 0u64;

    for (left, right) in left_list.iter().zip(right_list.iter()) {
        if left > right {
            accumulator += left - right;
        } else if right > left {
            accumulator += right - left;
        }
    }

    println!("Difference in distance is {accumulator}")
}

fn puzzle_2() {
    let file_content = read_to_string("input.txt").unwrap();

    let mut left_list = Vec::<u64>::new();
    let mut right_list = Vec::<u64>::new();

    for line in file_content.lines() {
        let (left, right) = line.split_once("   ").unwrap();

        left_list.push(left.parse().unwrap());
        right_list.push(right.parse().unwrap());
    }

    let mut accumulator = 0u64;

    for left_number in left_list {
        let occurences = right_list
            .iter()
            .filter(|right| **right == left_number)
            .count() as u64;
        accumulator += left_number * occurences;
    }

    println!("Difference in distance is {accumulator}")
}

fn main() {
    puzzle_1();
    puzzle_2();
}
