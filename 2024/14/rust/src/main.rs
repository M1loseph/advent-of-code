use std::fs::read_to_string;

#[derive(Debug)]
struct Robot {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

impl Robot {
    fn position_after_n_seconds(&self, seconds: i64, width: i64, height: i64) -> (i64, i64) {
        let distance_x = seconds * self.vx;
        let distance_y = seconds * self.vy;

        let mut tx = self.x + distance_x;
        let mut ty = self.y + distance_y;
        if tx < 0 {
            tx = width + tx % width;
        }
        if tx >= width {
            tx = tx % width;
        }
        if ty < 0 {
            ty = height + ty % height;
        }

        if ty >= height {
            ty = ty % height;
        }
        (tx, ty)
    }
}

fn puzzle_1(robots: &Vec<Robot>) {
    let seconds = 100;
    let width = 101;
    let height = 103;

    let mut quadrants = [0, 0, 0, 0];
    for robot in robots {
        let (x, y) = robot.position_after_n_seconds(seconds, width, height);
        if x < width / 2 && y < height / 2 {
            quadrants[0] += 1;
        } else if x > width / 2 && y < height / 2 {
            quadrants[1] += 1;
        } else if x < width / 2 && y > height / 2 {
            quadrants[2] += 1;
        } else if x > width / 2 && y > height / 2 {
            quadrants[3] += 1;
        }
    }
    let safety_factor = quadrants.iter().fold(1, |acc, next| acc * next);
    println!("Safety factor is {safety_factor}");
}

// TODO: finish me
fn puzzle_2(robots: &Vec<Robot>) {

    let mut seconds = 0;
    loop {
        let width = 101;
        let height = 103;

        let mut map = vec![vec!['.'; width as usize]; height as usize];
        for robot in robots {
            let (x, y) = robot.position_after_n_seconds(seconds, width, height);
            map[y as usize][x as usize] = '*'
        }

        for line in map {
            for char in line {
                print!("{char}");
            }
            println!()
        }

        seconds += 1;
    }
}

fn main() {
    let file = read_to_string("input.txt").unwrap();
    let robots: Vec<Robot> = file
        .lines()
        .map(|line| {
            let (position, velocity) = line.split_once(" ").unwrap();
            let position = position.replace("p=", "");
            let velocity = velocity.replace("v=", "");
            let (x, y) = position.split_once(",").unwrap();
            let (vx, vy) = velocity.split_once(",").unwrap();
            Robot {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                vx: vx.parse().unwrap(),
                vy: vy.parse().unwrap(),
            }
        })
        .collect();
    puzzle_1(&robots);
    puzzle_2(&robots);
}
