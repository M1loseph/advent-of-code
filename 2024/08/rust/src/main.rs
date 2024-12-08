use std::fs::read_to_string;

#[derive(PartialEq, Eq, Clone)]
enum Tile {
    Antena {
        frequency: char,
        underlying_tile: Box<Tile>,
    },
    Antinode,
    RegularField,
}

// TODO: try to implement with with a flat array and have some indexing operator
#[derive(Clone)]
struct MapWithAntenas {
    tiles: Vec<Vec<Tile>>,
}

impl MapWithAntenas {

    // TODO: this could return enum Result { OUT_OF_MAP, ALREADY_UPDATED, UPDATED etc. } - this would be way more readable
    fn safe_set_antinode(&mut self, x: &i64, y: &i64) -> bool {
        if (0..self.tiles.len() as i64).contains(&y)
            && (0..self.tiles[*y as usize].len() as i64).contains(&x)
        {
            let x = *x as usize;
            let y = *y as usize;
            match &mut self.tiles[y][x] {
                Tile::Antena {
                    underlying_tile, ..
                } => {
                    *underlying_tile = Box::new(Tile::Antinode);
                }
                Tile::RegularField => {
                    self.tiles[y][x] = Tile::Antinode;
                }
                Tile::Antinode => {},
            }
            return true;
        }
        false
    }

    fn fill_antinodes(&mut self, resonant: bool) {
        for y1 in 0..self.tiles.len() {
            for x1 in 0..self.tiles[y1].len() {
                match self.tiles[y1][x1] {
                    Tile::Antena {
                        frequency: freq1, ..
                    } => {
                        for y2 in 0..self.tiles.len() {
                            for x2 in 0..self.tiles[y2].len() {
                                match self.tiles[y2][x2] {
                                    Tile::Antena {
                                        frequency: freq2, ..
                                    } => {
                                        if freq1 == freq2 && (x1 != x2 || y1 != y2) {
                                            let dx = x1 as i64 - x2 as i64;
                                            let dy = y1 as i64 - y2 as i64;

                                            if resonant {
                                                let mut up_x = x1 as i64;
                                                let mut up_y = y1 as i64;
                                                while self.safe_set_antinode(&up_x, &up_y) {
                                                    up_x += dx;
                                                    up_y += dy;
                                                }

                                                let mut down_x = x1 as i64;
                                                let mut down_y = y1 as i64;
                                                while self.safe_set_antinode(&down_x, &down_y) {
                                                    down_x -= dx;
                                                    down_y -= dy;
                                                }
                                            } else {
                                                let new_x1 = x1 as i64 + dx;
                                                let new_y1 = y1 as i64 + dy;

                                                let new_x2 = x2 as i64 - dx;
                                                let new_y2 = y2 as i64 - dy;

                                                self.safe_set_antinode(&new_x1, &new_y1);
                                                self.safe_set_antinode(&new_x2, &new_y2);
                                            }
                                        }
                                    }
                                    Tile::Antinode => {}
                                    Tile::RegularField => {}
                                }
                            }
                        }
                    }
                    Tile::Antinode => {}
                    Tile::RegularField => {}
                }
            }
        }
    }

    fn count_antinodes(&self) -> usize {
        self.tiles
            .iter()
            .flat_map(|row| row.iter())
            .filter(|tile| match tile {
                Tile::Antena {
                    underlying_tile, ..
                } => *underlying_tile.as_ref() == Tile::Antinode,
                Tile::Antinode => true,
                Tile::RegularField => false,
            })
            .count()
    }

    fn print(&self) {
        for line in &self.tiles {
            for tile in line {
                match tile {
                    Tile::Antena { frequency, .. } => print!("{}", frequency),
                    Tile::Antinode => print!("#"),
                    Tile::RegularField => print!("."),
                }
            }
            println!();
        }
    }
}

fn puzzle_1(map_with_antenas: &MapWithAntenas) {
    let mut copy = map_with_antenas.clone();

    copy.fill_antinodes(false);
    copy.print();
    let number_of_antinodes = copy.count_antinodes();
    println!("There are {} antinodes on the map", number_of_antinodes);
}

fn puzzle_2(map_with_antenas: &MapWithAntenas) {
    let mut copy = map_with_antenas.clone();

    copy.fill_antinodes(true);
    copy.print();
    let number_of_antinodes = copy.count_antinodes();
    println!("There are {} antinodes on the map if you take into consideration resonan harmonics", number_of_antinodes);
}

fn main() {
    let input_file = read_to_string("input.txt").unwrap();
    let tiles_matrix = input_file
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => Tile::RegularField,
                    _ => Tile::Antena {
                        frequency: char,
                        underlying_tile: Box::new(Tile::RegularField),
                    },
                })
                .collect()
        })
        .collect();

    let map = MapWithAntenas {
        tiles: tiles_matrix,
    };
    puzzle_1(&map);
    puzzle_2(&map);
}
