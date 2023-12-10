use std::collections::HashMap;

use nom::{
    character::complete::{self, line_ending, one_of, space0, space1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{pair, terminated},
    IResult,
};

pub struct Day07a;
pub struct Day07b;

impl crate::Solution for Day07a {
    fn solve(&self) -> String {
        format!("{}", solve_a(include_str!("../inputs/day07")))
    }
}

impl crate::Solution for Day07b {
    fn solve(&self) -> String {
        format!("{}", solve_b(include_str!("../inputs/day07")))
    }
}

fn solve_a(input: &str) -> u64 {
    let mut hands = parse_a(input).expect("Could not parse input").1;
    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u64 + 1))
        .sum()
}

fn solve_b(input: &str) -> u64 {
    let mut hands = parse_b(input).expect("Could not parse input").1;
    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u64 + 1))
        .sum()
}

fn parse_a(input: &str) -> IResult<&str, Vec<Hand>> {
    parse(input, Card::part_a, Hand::part_a)
}

fn parse_b(input: &str) -> IResult<&str, Vec<Hand>> {
    parse(input, Card::part_b, Hand::part_b)
}

fn parse(
    input: &str,
    card: fn(char) -> Card,
    hand: fn(Vec<Card>, u64) -> Hand,
) -> IResult<&str, Vec<Hand>> {
    let hand = map(
        pair(
            terminated(many1(map(one_of("23456789TJQKA"), card)), space1),
            terminated(complete::u64, space0),
        ),
        |(cards, bid)| hand(cards, bid),
    );

    let (input, hands) = separated_list1(line_ending, hand)(input)?;
    Ok((input, hands))
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: u64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn part_a(cards: Vec<Card>, bid: u64) -> Self {
        // Insert cards into card_count and increment count
        let mut card_count = HashMap::new();
        for card in &cards {
            *card_count.entry(card.label).or_insert(0) += 1u8;
        }

        // Check for Type
        let hand_type = match card_count.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if card_count.values().any(|&c| c == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if card_count.values().any(|&c| c == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Invalid hand"),
        };

        Self {
            cards,
            hand_type,
            bid,
        }
    }

    fn part_b(cards: Vec<Card>, bid: u64) -> Self {
        // Insert cards into card_count and increment count
        let mut new_cards = vec![];
        let mut card_count = HashMap::new();

        for card in &cards {
            if card.label != 'J' {
                *card_count.entry(card).or_insert(0) += 1u8;
            }
        }

        // Find card with highest count
        if let Some((highest_card, _)) = card_count.iter().max_by_key(|(_, count)| *count) {
            for card in &cards {
                if card.label == 'J' {
                    new_cards.push(Card::joker(highest_card.label));
                } else {
                    new_cards.push(*card);
                }
            }
        } else {
            new_cards = vec![Card::joker('J'); 5];
        }

        Self::part_a(new_cards, bid)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare hand types
        let hand_type_cmp = self.hand_type.cmp(&other.hand_type);
        if hand_type_cmp != std::cmp::Ordering::Equal {
            hand_type_cmp
        } else {
            // Compare cards
            self.cards.iter().cmp(other.cards.iter())
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Card {
    label: char,
    strength: u64,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.strength.cmp(&other.strength)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Card {
    fn part_a(label: char) -> Self {
        let strength = match label {
            '2' => 0,
            '3' => 1,
            '4' => 2,
            '5' => 3,
            '6' => 4,
            '7' => 5,
            '8' => 6,
            '9' => 7,
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => panic!("Invalid card label: {}", label),
        };
        Self { label, strength }
    }

    fn part_b(label: char) -> Self {
        let strength = match label {
            'J' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            '9' => 8,
            'T' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => panic!("Invalid card label: {}", label),
        };
        Self { label, strength }
    }

    fn joker(label: char) -> Self {
        Self { label, strength: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn example_a() {
        assert_eq!(solve_a(EXAMPLE), 6440);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(EXAMPLE), 5905);
    }

    #[test]
    fn compare_card() {
        let card_1 = Card::part_a('2');
        let card_2 = Card::part_a('3');
        assert!(card_1 < card_2);

        let card_1 = Card::part_a('A');
        let card_2 = Card::part_a('A');
        assert!(card_1 == card_2);
    }

    #[test]
    fn compare_hand() {
        let h1 = Hand::part_a(
            vec![
                Card::part_a('J'),
                Card::part_a('J'),
                Card::part_a('5'),
                Card::part_a('7'),
                Card::part_a('A'),
            ],
            69,
        );
        let h2 = Hand::part_a(
            vec![
                Card::part_a('K'),
                Card::part_a('K'),
                Card::part_a('K'),
                Card::part_a('7'),
                Card::part_a('A'),
            ],
            0,
        );
        assert!(h1 < h2);
    }

    #[test]
    fn parse_a() {
        match super::parse_a(EXAMPLE) {
            Ok((input, hands)) => {
                assert_eq!(input, "");
                assert_eq!(hands.len(), 5);
                assert!(hands.iter().all(|h| h.cards.len() == 5));
                assert_eq!(hands[0].bid, 765);
                assert_eq!(hands[1].bid, 684);
                assert_eq!(hands[2].bid, 28);
                assert_eq!(hands[3].bid, 220);
                assert_eq!(hands[4].bid, 483);
                assert_eq!(
                    hands[2].cards,
                    vec![
                        Card::part_a('K'),
                        Card::part_a('K'),
                        Card::part_a('6'),
                        Card::part_a('7'),
                        Card::part_a('7')
                    ]
                );
                assert_eq!(hands[2].hand_type, HandType::TwoPair);
            }
            Err(e) => {
                dbg!(e);
                panic!("Could not parse example");
            }
        }
    }

    #[test]
    fn parse_b() {
        match super::parse_b(EXAMPLE) {
            Ok((input, hands)) => {
                assert_eq!(input, "");
                assert_eq!(hands.len(), 5);
                assert!(hands.iter().all(|h| h.cards.len() == 5));
                assert_eq!(hands[0].bid, 765);
                assert_eq!(hands[1].bid, 684);
                assert_eq!(hands[2].bid, 28);
                assert_eq!(hands[3].bid, 220);
                assert_eq!(hands[4].bid, 483);

                assert_eq!(
                    hands[0].cards,
                    vec![
                        Card::part_b('3'),
                        Card::part_b('2'),
                        Card::part_b('T'),
                        Card::part_b('3'),
                        Card::part_b('K')
                    ]
                );

                assert_eq!(
                    hands[1].cards,
                    vec![
                        Card::part_b('T'),
                        Card::part_b('5'),
                        Card::part_b('5'),
                        Card::joker('5'),
                        Card::part_b('5')
                    ]
                );

                assert_eq!(hands[0].hand_type, HandType::OnePair);
                assert_eq!(hands[1].hand_type, HandType::FourOfAKind);
                assert_eq!(hands[2].hand_type, HandType::TwoPair);
                assert_eq!(hands[3].hand_type, HandType::FourOfAKind);
                assert_eq!(hands[4].hand_type, HandType::FourOfAKind);
            }
            Err(e) => {
                dbg!(e);
                panic!("Could not parse example");
            }
        }
    }
}
