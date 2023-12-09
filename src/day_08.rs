use std::collections::HashMap;

use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, line_ending, multispace1},
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair, terminated},
    IResult,
};

pub struct Day08a;
pub struct Day08b;

impl crate::Solution for Day08a {
    fn solve(&self) -> String {
        format!("{}", solve_a(include_str!("../inputs/day08")))
    }
}

fn solve_a(input: &str) -> u64 {
    let (_, game) = parse_input(input).expect("Failed to parse input");

    let mut current = game.start;
    let mut steps = 0;
    let mut instructions = game.instructions.chars().cycle();

    while current != game.end {
        let (left, right) = game.nodes.get(current).expect("No node");

        current = if instructions.next().expect("No instructions") == 'R' {
            right
        } else {
            left
        };
        steps += 1;
    }

    steps
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, instructions) = terminated(is_a("RL"), multispace1)(input)?;

    let destinations = delimited(
        tag("("),
        separated_pair(alpha1, tag(", "), alpha1),
        tag(")"),
    );

    let line = pair(terminated(alpha1, tag(" = ")), destinations);

    let (input, nodes) = separated_list1(line_ending, line)(input)?;

    let map = nodes
        .into_iter()
        .fold(HashMap::new(), |mut acc, (source, destinations)| {
            acc.insert(source, destinations);
            acc
        });

    Ok((
        input,
        Input {
            instructions: instructions.to_string(),
            start: "AAA",
            end: "ZZZ",
            nodes: map,
        },
    ))
}

#[derive(Debug, PartialEq)]
struct Input<'a> {
    instructions: String,
    start: &'a str,
    end: &'a str,
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_1: &str = "RL
    
AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    static EXAMPLE_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn example_a_1() {
        assert_eq!(solve_a(EXAMPLE_1), 2);
    }

    #[test]
    fn example_a_2() {
        assert_eq!(solve_a(EXAMPLE_2), 6);
    }

    #[test]
    fn parse_a() {
        let (input, p) = parse_input(EXAMPLE_1).unwrap();
        assert_eq!(
            p,
            Input {
                instructions: "RL".to_string(),
                start: "AAA",
                end: "ZZZ",
                nodes: vec![
                    ("AAA", ("BBB", "CCC")),
                    ("BBB", ("DDD", "EEE")),
                    ("CCC", ("ZZZ", "GGG")),
                    ("DDD", ("DDD", "DDD")),
                    ("EEE", ("EEE", "EEE")),
                    ("GGG", ("GGG", "GGG")),
                    ("ZZZ", ("ZZZ", "ZZZ")),
                ]
                .into_iter()
                .collect(),
            }
        );
        assert_eq!(input, "");
    }
}
