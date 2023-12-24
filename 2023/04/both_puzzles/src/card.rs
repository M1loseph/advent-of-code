use std::collections::HashSet;

#[derive(Debug)]
pub struct Card {
    card_number: u32,
    winning_numbers: HashSet<u32>,
    numbers_you_picked: HashSet<u32>,
}

impl Card {
    pub fn parse_line(line: String) -> Option<Card> {
        let lines_without = line.strip_prefix("Card")?;
        let game_separator_index = lines_without.find(":")?;
        let card_number = &lines_without[0..game_separator_index]
            .trim()
            .parse::<u32>()
            .ok()?;
        let (winning_numbers_line, numbers_you_picked_line) =
            lines_without[game_separator_index + 1..].split_once("|")?;
        let winning_numbers = Card::parse_numbers(winning_numbers_line)?;
        let picked_numbers = Card::parse_numbers(numbers_you_picked_line)?;
        Some(Card {
            card_number: *card_number,
            winning_numbers,
            numbers_you_picked: picked_numbers,
        })
    }

    pub fn calculate_card_points(&self) -> u32 {
        let hits = self.number_of_matches();
        if hits == 0 {
            0
        } else {
            2u32.pow(hits - 1)
        }
    }

    pub fn number_of_matches(&self) -> u32 {
        self.winning_numbers
            .intersection(&self.numbers_you_picked)
            .count() as u32
    }

    fn parse_numbers(line: &str) -> Option<HashSet<u32>> {
        line.trim()
            .split(" ")
            .filter(|potential_number| !potential_number.is_empty())
            .map(|number| number.parse::<u32>().ok())
            .collect()
    }
}
