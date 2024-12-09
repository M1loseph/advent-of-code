use super::card::Card;
use super::hand::Hand;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum HandKind {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

pub fn first_puzzle_hand_kind(hand: &Hand) -> HandKind {
    let mut card_count = HashMap::new();
    for card in &hand.cards {
        card_count
            .entry(card)
            .and_modify(|count| *count += 1)
            .or_insert(1u32);
    }

    let count: Vec<u32> = card_count.values().copied().collect();

    return if count.contains(&5) {
        HandKind::FiveOfAKind
    } else if count.contains(&4) {
        HandKind::FourOfAKind
    } else if count.contains(&3) && count.contains(&2) {
        HandKind::FullHouse
    } else if count.contains(&3) {
        HandKind::ThreeOfAKind
    } else if count.iter().filter(|v| **v == 2).count() == 2 {
        HandKind::TwoPair
    } else if count.contains(&2) {
        HandKind::OnePair
    } else {
        HandKind::HighCard
    };
}

pub fn second_puzzle_hand_kind(hand: &Hand) -> HandKind {
    let jacks = hand.cards.iter().filter(|c| c == &&Card::Jack).count() as u32;
    let mut counts: Vec<u32> = hand
        .cards
        .iter()
        .filter(|c| c != &&Card::Jack)
        .into_group_map_by(|c| **c)
        .values()
        .into_iter()
        .map(|g| g.len() as u32)
        .collect();

    counts.sort();
    let max = if counts.is_empty() {
        0
    } else {
        counts[counts.len() - 1]
    };

    return if max + jacks == 5 {
        HandKind::FiveOfAKind
    } else if max + jacks == 4 {
        HandKind::FourOfAKind
    } else if counts.iter().filter(|c| **c == 2).count() == 2 && jacks == 1
        || counts.contains(&3) && counts.contains(&2)
    {
        HandKind::FullHouse
    } else if max + jacks == 3 {
        HandKind::ThreeOfAKind
    } else if counts.iter().filter(|v| **v == 2).count() as u32 + jacks == 2 {
        HandKind::TwoPair
    } else if counts.contains(&2) || jacks == 1 {
        HandKind::OnePair
    } else {
        HandKind::HighCard
    };
}

mod test {
    #[test]
    fn test_hand_kind_ordering() {
        use super::HandKind::*;
        assert!(FiveOfAKind > FourOfAKind);
        assert!(FourOfAKind > FullHouse);
        assert!(FullHouse > ThreeOfAKind);
        assert!(ThreeOfAKind > TwoPair);
        assert!(TwoPair > OnePair);
        assert!(OnePair > HighCard);
    }

    #[test]
    fn test_first_puzzle_hand_kind() {
        use super::super::card::Card;
        use super::super::hand::Hand;
        use super::first_puzzle_hand_kind;
        use super::HandKind::*;

        assert_eq!(
            first_puzzle_hand_kind(&Hand::create(
                [Card::As, Card::As, Card::As, Card::As, Card::As,],
                1,
            )),
            FiveOfAKind
        );
        assert_eq!(
            first_puzzle_hand_kind(&Hand::create(
                [Card::As, Card::As, Card::As, Card::As, Card::King],
                1
            )),
            FourOfAKind
        );
        assert_eq!(
            first_puzzle_hand_kind(&Hand::create(
                [Card::As, Card::As, Card::As, Card::King, Card::King],
                1
            )),
            FullHouse
        );
        assert_eq!(
            first_puzzle_hand_kind(&Hand::create(
                [Card::As, Card::As, Card::As, Card::King, Card::Queen],
                1
            )),
            ThreeOfAKind
        );
        assert_eq!(
            first_puzzle_hand_kind(&Hand::create(
                [Card::As, Card::As, Card::King, Card::King, Card::Queen],
                1
            )),
            TwoPair
        );
        assert_eq!(
            first_puzzle_hand_kind(&Hand::create(
                [Card::As, Card::As, Card::King, Card::Queen, Card::Jack],
                1
            )),
            OnePair
        );
        assert_eq!(
            first_puzzle_hand_kind(&Hand::create(
                [Card::As, Card::King, Card::Queen, Card::Jack, Card::Ten],
                1
            )),
            HighCard
        );
    }

    #[test]
    fn test_second_puzzle_hand_kind() {
        use super::super::card::Card;
        use super::super::hand::Hand;
        use super::second_puzzle_hand_kind;
        use super::HandKind::*;

        assert_eq!(
            second_puzzle_hand_kind(&Hand::create(
                [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King,],
                1,
            )),
            OnePair,
        );

        assert_eq!(
            second_puzzle_hand_kind(&Hand::create(
                [Card::King, Card::King, Card::Six, Card::Seven, Card::Seven,],
                1,
            )),
            TwoPair,
        );

        assert_eq!(
            second_puzzle_hand_kind(&Hand::create(
                [Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five,],
                1,
            )),
            FourOfAKind,
        );

        assert_eq!(
            second_puzzle_hand_kind(&Hand::create(
                [Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten,],
                1,
            )),
            FourOfAKind,
        );

        assert_eq!(
            second_puzzle_hand_kind(&Hand::create(
                [Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::As,],
                1,
            )),
            FourOfAKind,
        );

        assert_eq!(
            second_puzzle_hand_kind(&Hand::create(
                [Card::Two, Card::Two, Card::Three, Card::Jack, Card::Five,],
                1,
            )),
            ThreeOfAKind,
        );
    }
}
