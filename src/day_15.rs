crate::solution!(15, solve_a, solve_b);

use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, digit1, multispace0},
    multi::separated_list1,
    sequence::{delimited, pair, preceded},
    Parser,
};

#[derive(Debug)]
struct Sequence<'a> {
    label: &'a str,
    pattern: Operation,
}

#[derive(Debug)]
enum Operation {
    Remove(),
    Insert(u8),
}

#[derive(Debug, Default, Clone)]
struct Box {
    lenses: Vec<Lens>,
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u8,
}

pub fn solve_a(input: &str) -> u64 {
    let (rest, patterns) = parse_a(input).unwrap();
    debug_assert!(rest.is_empty());

    patterns
        .into_iter()
        .map(|pattern| hash(pattern) as u64)
        .sum()
}

pub fn solve_b(input: &str) -> u64 {
    let (rest, patterns) = parse_b(input).unwrap();
    debug_assert!(rest.is_empty());

    let mut boxes = vec![Box::default(); 256];
    patterns.into_iter().for_each(|p| {
        let index = hash(p.label);
        match p.pattern {
            Operation::Remove() => {
                let b = &mut boxes[index as usize];
                if let Some(lens_index) = b.lenses.iter().position(|l| l.label == p.label) {
                    b.lenses.remove(lens_index);
                };
            }
            Operation::Insert(focal_length) => {
                let b = &mut boxes[index as usize];
                if let Some(lens_index) = b.lenses.iter().position(|l| l.label == p.label) {
                    b.lenses.get_mut(lens_index).unwrap().focal_length = focal_length;
                } else {
                    b.lenses.push(Lens {
                        label: p.label.to_string(),
                        focal_length,
                    });
                }
            }
        };

        /*
        println!("After {:}:", p);
        boxes
            .iter()
            .enumerate()
            .filter(|(_, b)| !b.lenses.is_empty())
            .for_each(|(i, b)| println!("Box {i}: {b}"));
        println!();
        */
    });

    boxes
        .iter()
        .enumerate()
        .map(|(box_index, b)| {
            b.lenses
                .iter()
                .enumerate()
                .map(move |(lense_index, lense)| {
                    (box_index + 1) * (lense_index + 1) * lense.focal_length as usize
                })
                .sum::<usize>()
        })
        .sum::<usize>() as u64
}

fn hash(input: &str) -> u8 {
    input
        .chars()
        .fold(0, |acc, c| ((acc as u16 + c as u16) as u8).wrapping_mul(17))
}

fn parse_a(input: &str) -> nom::IResult<&str, Vec<&str>> {
    delimited(
        multispace0,
        separated_list1(tag(","), is_not(",\n\r")),
        multispace0,
    )(input)
}

fn parse_b(input: &str) -> nom::IResult<&str, Vec<Sequence>> {
    let sequence = pair(
        alpha1,
        alt((
            tag("-").map(|_| Operation::Remove()),
            preceded(
                tag("="),
                digit1.map(|s: &str| Operation::Insert(s.parse::<u8>().unwrap())),
            ),
        )),
    )
    .map(|(label, pattern)| Sequence { label, pattern });

    delimited(
        multispace0,
        separated_list1(tag(","), sequence),
        multispace0,
    )(input)
}

impl Display for Sequence<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.label, self.pattern)
    }
}

impl Display for Box {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for lens in &self.lenses {
            write!(f, "{} ", lens)?;
        }
        Ok(())
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Remove() => write!(f, "-"),
            Operation::Insert(focal_length) => write!(f, "={}", focal_length),
        }
    }
}

impl Display for Lens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.label, self.focal_length,)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn example_a() {
        assert_eq!(solve_a(EXAMPLE), 1320);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(EXAMPLE), 145);
    }

    #[test]
    fn hash() {
        let (rest, patterns) = parse_a(EXAMPLE).unwrap();
        assert_eq!(rest, "");
        assert_eq!(patterns.len(), 11);

        assert_eq!(super::hash(patterns[0]), 30);
        assert_eq!(super::hash(patterns[1]), 253);
        assert_eq!(super::hash(patterns[2]), 97);
        assert_eq!(super::hash(patterns[3]), 47);
        assert_eq!(super::hash(patterns[4]), 14);
        assert_eq!(super::hash(patterns[5]), 180);
        assert_eq!(super::hash(patterns[6]), 9);
        assert_eq!(super::hash(patterns[7]), 197);
        assert_eq!(super::hash(patterns[8]), 48);
        assert_eq!(super::hash(patterns[9]), 214);
        assert_eq!(super::hash(patterns[10]), 231);
    }
}
