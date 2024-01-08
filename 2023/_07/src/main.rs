mod hand;

use std::io::{Error, ErrorKind, Result};

use file_reader::read_file;
use hand::card::first_puzzle_card_strength_order;
use hand::kind::first_puzzle_hand_kind;
use hand::{card::UnknownCardError, *};

use crate::hand::card::second_puzzle_card_strength_order;
use crate::hand::kind::second_puzzle_hand_kind;

type CardStrengthRule = fn(&Card) -> u32;
type HandKindRule = fn(&Hand) -> HandKind;

struct GameRules {
    pub card_strength_rule: CardStrengthRule,
    pub hand_kind_rule: HandKindRule,
    pub hands: Vec<Hand>,
}

impl GameRules {
    pub fn play(&mut self) -> u64 {
        self.hands.sort_by(|a, b| {
            GameRules::compare(a, b, &self.hand_kind_rule, &self.card_strength_rule)
        });
        self.hands
            .iter()
            .enumerate()
            .map(|(index, card)| (index + 1) as u64 * card.bid as u64)
            .sum()
    }

    fn compare(
        left: &Hand,
        right: &Hand,
        hand_kind_rule: &HandKindRule,
        card_strength_rule: &CardStrengthRule,
    ) -> std::cmp::Ordering {
        let kind_comparison = hand_kind_rule(left).cmp(&hand_kind_rule(right));
        if kind_comparison != std::cmp::Ordering::Equal {
            return kind_comparison;
        }

        for (left, right) in left.cards.iter().zip(right.cards.iter()) {
            let left_strength = card_strength_rule(left);
            let right_strength = card_strength_rule(right);

            let comparison = left_strength.cmp(&right_strength);
            if comparison != std::cmp::Ordering::Equal {
                return comparison;
            }
        }

        std::cmp::Ordering::Equal
    }
}
impl From<UnknownCardError> for std::io::Error {
    fn from(_: UnknownCardError) -> Self {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "the card name is not valid",
        )
    }
}

fn line_to_hand_parser(line: String) -> Result<Hand> {
    let (hand, bid) = line.split_once(" ").ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "the line should be separated by one space",
        )
    })?;

    if hand.len() != 5 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "The hand should consist out of 5 cards",
        ));
    }

    let cards: [Card; 5] = hand
        .chars()
        .map(|card_name| Card::card_factory(&card_name).map_err(|e| std::io::Error::from(e)))
        .collect::<Result<Vec<Card>>>()?
        .try_into()
        .map_err(|_| Error::new(ErrorKind::InvalidData, "some error"))?;

    Ok(Hand::create(
        cards,
        bid.parse::<u32>()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?,
    ))
}

fn read_all_cards() -> Result<Vec<Hand>> {
    read_file("input/input.txt")?
        .map(|line| line_to_hand_parser(line?))
        .collect()
}

fn puzzle_1() {
    let mut game = GameRules {
        card_strength_rule: first_puzzle_card_strength_order,
        hand_kind_rule: first_puzzle_hand_kind,
        hands: read_all_cards().unwrap(),
    };

    let sum = game.play();

    println!("Puzzle 1: {}", sum);
}

fn puzzle_2() {
    let mut game = GameRules {
        card_strength_rule: second_puzzle_card_strength_order,
        hand_kind_rule: second_puzzle_hand_kind,
        hands: read_all_cards().unwrap(),
    };

    let sum = game.play();

    println!("Puzzle 2: {}", sum);
}

fn main() {
    puzzle_1();
    puzzle_2();
}
