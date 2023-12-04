pub struct Day04a;
pub struct Day04b;

impl crate::Solution for Day04a {
    fn solve(&self) -> String {
        format!("{}", solve_a(include_str!("../inputs/day04")))
    }
}

impl crate::Solution for Day04b {
    fn solve(&self) -> String {
        format!("{}", solve_b(include_str!("../inputs/day04")))
    }
}

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, terminated},
    IResult,
};

fn solve_a(input: &str) -> u32 {
    let cards = parse_cards(input).expect("Failed to parse cards").1;

    let mut score: u32 = 0;
    for card in cards {
        let winning_cards = card
            .numbers
            .iter()
            .filter(|number| card.winning.contains(number))
            .count() as u32;

        if winning_cards > 0 {
            score += 2u32.pow(winning_cards - 1);
        }
    }

    score
}

fn solve_b(input: &str) -> u32 {
    let mut cards = parse_cards(input).expect("Failed to parse cards").1;

    for i in 0..cards.len() {
        let winning_cards = cards[i]
            .numbers
            .iter()
            .filter(|number| cards[i].winning.contains(number))
            .count();

        let amount_won = cards[i].count;
        for card in cards.iter_mut().skip(i + 1).take(winning_cards) {
            card.count += amount_won;
        }
    }

    cards.iter().map(|card| card.count).sum()
}

#[derive(Debug)]
struct Card {
    #[allow(dead_code)]
    id: u32,
    count: u32,
    winning: Vec<u32>,
    numbers: Vec<u32>,
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, cards) = separated_list1(multispace1, parse_card)(input)?;
    Ok((input, cards))
}

/// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = terminated(tag("Card"), multispace0)(input)?;
    let (input, id) = map_res(digit1, str::parse::<u32>)(input)?;
    let (input, _) = delimited(multispace0, tag(":"), multispace0)(input)?;
    let (input, winning) = separated_list1(multispace1, map_res(digit1, str::parse::<u32>))(input)?;
    let (input, _) = delimited(multispace0, tag("|"), multispace0)(input)?;
    let (input, numbers) = separated_list1(multispace1, map_res(digit1, str::parse::<u32>))(input)?;

    Ok((
        input,
        Card {
            id,
            count: 1,
            winning,
            numbers,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn a_parse() {
        let (input, cards) = parse_cards(EXAMPLE).unwrap();
        dbg!(&cards);

        assert_eq!(input, "");
        assert_eq!(cards.len(), 6);
    }

    #[test]
    fn a_card1() {
        let l1 = EXAMPLE.lines().next().unwrap();
        assert_eq!(solve_a(l1), 8);
    }

    #[test]
    fn a_card2() {
        let l2 = EXAMPLE.lines().nth(1).unwrap();
        assert_eq!(solve_a(l2), 2);
    }

    #[test]
    fn a_card3() {
        let l3 = EXAMPLE.lines().nth(2).unwrap();
        assert_eq!(solve_a(l3), 2);
    }

    #[test]
    fn a_card4() {
        let l4 = EXAMPLE.lines().nth(3).unwrap();
        assert_eq!(solve_a(l4), 1);
    }

    #[test]
    fn a_card5() {
        let l5 = EXAMPLE.lines().nth(4).unwrap();
        assert_eq!(solve_a(l5), 0);
    }

    #[test]
    fn a_card6() {
        let l6 = EXAMPLE.lines().nth(5).unwrap();
        assert_eq!(solve_a(l6), 0);
    }

    #[test]
    fn a_example() {
        assert_eq!(solve_a(EXAMPLE), 13);
    }

    #[test]
    fn b_example() {
        assert_eq!(solve_b(EXAMPLE), 30);
    }
}
