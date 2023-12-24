use crate::card::Card;
use std::io::BufRead;

pub struct Game {
    cards: Vec<Card>,
}

impl Game {
    pub fn read_from_file(path: &str) -> std::io::Result<Game> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let cards: Option<Vec<Card>> = reader
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| Card::parse_line(line))
            .collect();
        cards.map(|cards| Game { cards }).ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid data, probably one of the lines is not in the correct format",
        ))
    }

    pub fn calculate_total_points(&self) -> u32 {
        self.cards
            .iter()
            .map(|card| card.calculate_card_points())
            .sum()
    }

    pub fn perform_puzzle_2_game(&self) -> u64 {
        let mut numbers_of_cards: Vec<u64> = self.cards.iter().map(|_| 1u64).collect();
        for (i, card) in self.cards.iter().enumerate() {
            let wins = card.number_of_matches();
            let cards_len = numbers_of_cards[i];
            for win in 1..=wins {
                match numbers_of_cards.get_mut(i + win as usize) {
                    Some(cards_number) => *cards_number += cards_len,
                    None => (),
                }
            }
        }
        numbers_of_cards.iter().sum()
    }
}
