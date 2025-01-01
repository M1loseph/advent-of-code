use std::fs::read_to_string;

fn vertical_sizes(specification: &[Vec<char>]) -> Vec<u8> {
    let rows = specification.len();
    let cols = specification[0].len();

    let mut result = Vec::new();
    for col in 0..cols {
        let mut height = 0;
        for row in 0..rows {
            if specification[row][col] == '#' {
                height += 1;
            }
        }
        result.push(height);
    }
    result
}

fn puzzle_1(keys: &Vec<Vec<u8>>, locks: &Vec<Vec<u8>>) {
    let mut matching_keys = 0;
    for lock in locks {
        'keys: for key in keys {
            assert_eq!(lock.len(), key.len());
            for i in 0..key.len() {
                if lock[i] + key[i] > 5 {
                    continue 'keys;
                }
            }
            matching_keys += 1;
        }
    }
    println!("First puzzle result is {matching_keys}");
}

fn main() {
    let file_content = read_to_string("input.txt").unwrap();
    let mut keys: Vec<Vec<u8>> = Vec::new();
    let mut locks: Vec<Vec<u8>> = Vec::new();

    let mut buffer: Vec<Vec<char>> = Vec::new();
    file_content.lines().chain(vec![""].into_iter()).for_each(|line| {
        if !line.is_empty() {
            buffer.push(line.chars().collect());
            return;
        }
        let first_line = buffer.first().unwrap();
        let last_line = buffer.last().unwrap();
        if first_line.iter().filter(|c| **c == '#').count() == first_line.len()
            && last_line.iter().filter(|c| **c == '.').count() == last_line.len()
        {
            let lock_specification = &buffer[1..6];
            locks.push(vertical_sizes(lock_specification));
        } else if first_line.iter().filter(|c| **c == '.').count() == first_line.len()
            && last_line.iter().filter(|c| **c == '#').count() == last_line.len()
        {
            let key_specification = &buffer[1..6];
            keys.push(vertical_sizes(key_specification));
        } else {
            panic!("Unable to parse {:?}", buffer);
        }
        buffer.clear();
    });
    puzzle_1(&keys, &locks);
}
