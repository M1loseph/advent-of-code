mod game;
mod card;

use game::Game;

fn puzzle_1() {
    let game = Game::read_from_file("input/input.txt").unwrap();
    let result = game.calculate_total_points();
    println!("Total points: {}", result);
}

fn puzzle_2() {
    let game = Game::read_from_file("input/input.txt").unwrap();
    let result = game.perform_puzzle_2_game();
    println!("Total cards: {}", result);
}

fn main() {
    puzzle_1();
    puzzle_2();
}
