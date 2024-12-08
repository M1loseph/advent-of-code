use std::fs::read_to_string;

fn puzzle_1(letters_matrix: &Vec<Vec<char>>) {
    let searched_word = "XMAS";
    let searched_letters = searched_word.chars().collect::<Vec<char>>();

    let mut xmas_count = 0u64;

    for y in 0..letters_matrix.len() {
        for x in 0..letters_matrix[y].len() {
            let iteration_destination = [
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ];

            let y_range = 0..(letters_matrix.len() as i64);
            let x_range = 0..(letters_matrix[y].len() as i64);

            for (xdir, ydir) in iteration_destination {
                let mut correct_string = true;
                let mut x_offset = 0i64;
                let mut y_offset = 0i64;
                for letter in &searched_letters {
                    let x = x as i64 + x_offset;
                    let y = y as i64 + y_offset;

                    if !x_range.contains(&x) || !y_range.contains(&y) {
                        correct_string = false;
                        break;
                    }
                    if *letter != letters_matrix[y as usize][x as usize] {
                        correct_string = false;
                        break;
                    }

                    x_offset += xdir;
                    y_offset += ydir;
                }

                if correct_string {
                    xmas_count += 1;
                }
            }
        }
    }
    println!("XMAS count {xmas_count}");
}

fn puzzle_2(letters_matrix: &Vec<Vec<char>>) {
    let searched_word = "MAS";
    let searched_letters = searched_word.chars().collect::<Vec<char>>();

    let mut x_mas_count = 0u64;

    let is_searched_word = |x: i64, y: i64, xdir: i64, ydir: i64| -> bool {
        let mut x_offset = 0i64;
        let mut y_offset = 0i64;
        for letter in &searched_letters {
            let x = x + x_offset;
            let y = y + y_offset;

            if *letter != letters_matrix[y as usize][x as usize] {
                return false;
            }

            x_offset += xdir;
            y_offset += ydir;
        }
        true
    };

    for y in 0..letters_matrix.len() - searched_letters.len() + 1 {
        for x in 0..letters_matrix[y].len() - searched_letters.len() + 1 {
            let offset = searched_letters.len() as i64 - 1;

            let first_diagonal_ok = is_searched_word(x as i64, y as i64, 1, 1)
                || is_searched_word(x as i64 + offset, y as i64 + offset, -1, -1);

            let second_diagonal_ok = is_searched_word(x as i64 + offset, y as i64, -1, 1)
                || is_searched_word(x as i64, y as i64 + offset, 1, -1);

            if first_diagonal_ok && second_diagonal_ok {
                x_mas_count += 1;
            }
        }
    }
    println!("X-MAS count {x_mas_count}");
}

fn main() {
    let file_content = read_to_string("input.txt").unwrap();
    let letters_matrix = file_content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    puzzle_1(&letters_matrix);
    puzzle_2(&letters_matrix);
}
