use super::card::Card;

pub struct Hand {
    pub cards: [Card; 5],
    pub bid: u32,
}

impl Hand {
    pub fn create(cards: [Card; 5], bid: u32) -> Self {
        Self { cards, bid }
    }
}
