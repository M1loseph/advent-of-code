use std::collections::HashMap;

#[derive(Debug, Hash, Eq, Clone, Copy, PartialEq)]
pub enum Card {
    As,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Debug)]
pub struct UnknownCardError {
    pub message: String,
}

impl UnknownCardError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl Card {
    pub fn card_factory(card_name: &char) -> Result<Card, UnknownCardError> {
        match card_name {
            'A' => Ok(Card::As),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => Err(UnknownCardError::new(format!("Unknown card: {card_name}"))),
        }
    }
}

// TODO: find a way to create this map once or find a better key to value algorithm
pub fn first_puzzle_card_strength_order(card: &Card) -> u32 {
    use Card::*;

    let strengths = HashMap::from([
        (As, 15),
        (King, 14),
        (As, 13),
        (King, 12),
        (Queen, 11),
        (Jack, 10),
        (Ten, 9),
        (Nine, 8),
        (Eight, 7),
        (Seven, 6),
        (Six, 5),
        (Five, 4),
        (Four, 3),
        (Three, 2),
        (Two, 1),
    ]);

    strengths[card]
}

pub fn second_puzzle_card_strength_order(card: &Card) -> u32 {
    use Card::*;

    let strengths = HashMap::from([
        (As, 15),
        (King, 14),
        (As, 13),
        (King, 12),
        (Queen, 11),
        (Ten, 10),
        (Nine, 9),
        (Eight, 8),
        (Seven, 7),
        (Six, 6),
        (Five, 5),
        (Four, 4),
        (Three, 3),
        (Two, 2),
        (Jack, 1),
    ]);

    strengths[card]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_strength_first_puzzle() {
        use super::first_puzzle_card_strength_order;
        use super::Card::*;

        assert!(first_puzzle_card_strength_order(&As) > first_puzzle_card_strength_order(&King));
        assert!(first_puzzle_card_strength_order(&King) > first_puzzle_card_strength_order(&Queen));
        assert!(first_puzzle_card_strength_order(&Queen) > first_puzzle_card_strength_order(&Jack));
        assert!(first_puzzle_card_strength_order(&Jack) > first_puzzle_card_strength_order(&Ten));
        assert!(first_puzzle_card_strength_order(&Ten) > first_puzzle_card_strength_order(&Nine));
        assert!(first_puzzle_card_strength_order(&Nine) > first_puzzle_card_strength_order(&Eight));
        assert!(
            first_puzzle_card_strength_order(&Eight) > first_puzzle_card_strength_order(&Seven)
        );
        assert!(first_puzzle_card_strength_order(&Seven) > first_puzzle_card_strength_order(&Six));
        assert!(first_puzzle_card_strength_order(&Six) > first_puzzle_card_strength_order(&Five));
        assert!(first_puzzle_card_strength_order(&Five) > first_puzzle_card_strength_order(&Four));
        assert!(first_puzzle_card_strength_order(&Four) > first_puzzle_card_strength_order(&Three));
        assert!(first_puzzle_card_strength_order(&Three) > first_puzzle_card_strength_order(&Two));
    }

    #[test]
    fn test_strength_second_puzzle() {
        use super::second_puzzle_card_strength_order;
        use super::Card::*;

        assert!(second_puzzle_card_strength_order(&As) > second_puzzle_card_strength_order(&King));
        assert!(
            second_puzzle_card_strength_order(&King) > second_puzzle_card_strength_order(&Queen)
        );
        assert!(
            second_puzzle_card_strength_order(&Queen) > second_puzzle_card_strength_order(&Ten)
        );
        assert!(second_puzzle_card_strength_order(&Ten) > second_puzzle_card_strength_order(&Nine));
        assert!(
            second_puzzle_card_strength_order(&Nine) > second_puzzle_card_strength_order(&Eight)
        );
        assert!(
            second_puzzle_card_strength_order(&Eight) > second_puzzle_card_strength_order(&Seven)
        );
        assert!(
            second_puzzle_card_strength_order(&Seven) > second_puzzle_card_strength_order(&Six)
        );
        assert!(second_puzzle_card_strength_order(&Six) > second_puzzle_card_strength_order(&Five));
        assert!(
            second_puzzle_card_strength_order(&Five) > second_puzzle_card_strength_order(&Four)
        );
        assert!(
            second_puzzle_card_strength_order(&Four) > second_puzzle_card_strength_order(&Three)
        );
        assert!(
            second_puzzle_card_strength_order(&Three) > second_puzzle_card_strength_order(&Two)
        );
        assert!(second_puzzle_card_strength_order(&Two) > second_puzzle_card_strength_order(&Jack));
    }
}
