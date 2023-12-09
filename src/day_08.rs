use std::collections::HashMap;

use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, alphanumeric1, line_ending, multispace1},
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

impl crate::Solution for Day08b {
    fn solve(&self) -> String {
        format!("{}", solve_b(include_str!("../inputs/day08")))
    }
}

fn solve_a(input: &str) -> u64 {
    let (_, game) = parse_input(input).expect("Failed to parse input");

    let mut current = "AAA";
    let mut steps = 0;
    let mut instructions = game.instructions.chars().cycle();

    while current != "ZZZ" {
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

fn solve_b(input: &str) -> u64 {
    let (_, game) = parse_input(input).expect("Failed to parse input");

    let start_nodes = game.nodes.keys().filter(|&k| k.ends_with('A'));

    let cycle_count = start_nodes.map(|node| {
        let mut instructions = game.instructions.chars().cycle();

        let mut step_count = 0;
        let mut current = node;
        while !current.ends_with('Z') {
            let (left, right) = game.nodes.get(current).expect("No node");
            current = if instructions.next().expect("No instructions") == 'R' {
                right
            } else {
                left
            };
            step_count += 1;
        }
        step_count
    });

    lcm(cycle_count.collect::<Vec<_>>().as_slice())
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, instructions) = terminated(is_a("RL"), multispace1)(input)?;

    let destinations = delimited(
        tag("("),
        separated_pair(alphanumeric1, tag(", "), alphanumeric1),
        tag(")"),
    );

    let line = pair(terminated(alphanumeric1, tag(" = ")), destinations);

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
            nodes: map,
        },
    ))
}

#[derive(Debug, PartialEq)]
struct Input<'a> {
    instructions: String,
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
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

    static EXAMPLE_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn example_a_1() {
        assert_eq!(solve_a(EXAMPLE_1), 2);
    }

    #[test]
    fn example_a_2() {
        assert_eq!(solve_a(EXAMPLE_2), 6);
    }

    #[test]
    fn example_b_1() {
        assert_eq!(solve_b(EXAMPLE_3), 6);
    }

    #[test]
    fn parse_a() {
        let (input, p) = parse_input(EXAMPLE_1).unwrap();
        assert_eq!(input, "");
        assert_eq!(
            p,
            Input {
                instructions: "RL".to_string(),
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

    #[test]
    fn parse_b() {
        let (input, p) = parse_input(EXAMPLE_3).unwrap();
        assert_eq!(input, "");
        assert_eq!(
            p,
            Input {
                instructions: "LR".to_string(),
                nodes: vec![
                    ("11A", ("11B", "XXX")),
                    ("11B", ("XXX", "11Z")),
                    ("11Z", ("11B", "XXX")),
                    ("22A", ("22B", "XXX")),
                    ("22B", ("22C", "22C")),
                    ("22C", ("22Z", "22Z")),
                    ("22Z", ("22B", "22B")),
                    ("XXX", ("XXX", "XXX")),
                ]
                .into_iter()
                .collect(),
            }
        );
        assert_eq!(input, "");
    }
}
