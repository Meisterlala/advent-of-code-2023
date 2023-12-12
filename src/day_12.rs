use std::usize;

use nom::{
    branch::alt,
    character::complete::char,
    character::complete::{line_ending, space1, u16},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
};

pub struct Day12a;
pub struct Day12b;

impl crate::Solution for Day12a {
    fn solve(&self) -> String {
        format!("{}", solve_a(include_str!("../inputs/day12")))
    }
}

impl crate::Solution for Day12b {
    fn solve(&self) -> String {
        // format!("{}", solve_b(include_str!("../inputs/day11"), 1_000_000))
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

fn solve_a(input: &str) -> u64 {
    let (_, springs) = parse(input).expect("Failed to parse input");

    springs
        .iter()
        .map(|(conditions, broken)| arrangements(conditions, broken))
        .sum()
}

type Broken = u16;
type Springs = (Vec<Condition>, Vec<Broken>);

fn parse_springs(input: &str) -> nom::IResult<&str, Springs> {
    let condition = alt((
        map(char('.'), |_| Condition::Operational),
        map(char('#'), |_| Condition::Damaged),
        map(char('?'), |_| Condition::Unknown),
    ));

    separated_pair(many1(condition), space1, separated_list1(char(','), u16))(input)
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Springs>> {
    separated_list1(line_ending, parse_springs)(input)
}

fn arrangements(conditions: &[Condition], broken: &[Broken]) -> u64 {
    // If the there not supposed to be any more broken springs, and all conditions are operational or unknown
    if broken.is_empty() {
        // We have a valid arrangement
        if conditions
            .iter()
            .all(|&c| matches!(c, Condition::Operational | Condition::Unknown))
        {
            return 1;
        } else {
            return 0;
        }
    // If there are supposed the same amount of broken springs as conditions, and all conditions are damaged or unknown
    } else if broken[0] == conditions.len() as u16 {
        if conditions
            .iter()
            .all(|&c| matches!(c, Condition::Damaged | Condition::Unknown))
        {
            return 1;
        } else {
            return 0;
        }
    }

    // If to many springs are broken, we dont have any valid arrangements
    if broken.iter().map(|i| *i as usize).sum::<usize>() + broken.len().saturating_sub(1)
        > conditions.len()
    {
        return 0;
    }

    // If there are no more inputs, we dont have any more arangements
    if conditions.is_empty() {
        return 0;
    }

    let mut valid = 0;

    // Try to skip spring at current location, assuming that its operational
    if matches!(conditions[0], Condition::Operational | Condition::Unknown) {
        valid += arrangements(&conditions[1..], broken);
    }

    // Try to fit springs at current location, knowing that the next spring after the group is operational
    if (broken[0] as usize) < conditions.len()
        && conditions
            .iter()
            .take(broken[0] as usize)
            .all(|&c| matches!(c, Condition::Damaged | Condition::Unknown))
        && matches!(
            conditions[broken[0] as usize],
            Condition::Operational | Condition::Unknown
        )
    {
        valid += arrangements(&conditions[broken[0] as usize + 1..], &broken[1..]);
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn example_a() {
        assert_eq!(solve_a(EXAMPLE), 21);
    }

    #[test]
    fn arrage_1() {
        let (_, (springs, broken)) = parse_springs("???.### 1,1,3").unwrap();
        assert_eq!(arrangements(&springs, &broken), 1);
    }

    #[test]
    fn arrage_2() {
        let (_, (springs, broken)) = parse_springs(".??..??...?##. 1,1,3").unwrap();
        assert_eq!(arrangements(&springs, &broken), 4);
    }

    #[test]
    fn arrage_3() {
        let (_, (springs, broken)) = parse_springs("?#?#?#?#?#?#?#? 1,3,1,6").unwrap();
        assert_eq!(arrangements(&springs, &broken), 1);
    }

    #[test]
    fn arrage_4() {
        let (_, (springs, broken)) = parse_springs("????.#...#... 4,1,1").unwrap();
        assert_eq!(arrangements(&springs, &broken), 1);
    }

    #[test]
    fn arrage_5() {
        let (_, (springs, broken)) = parse_springs("????.######..#####. 1,6,5").unwrap();
        assert_eq!(arrangements(&springs, &broken), 4);
    }

    #[test]
    fn arrage_6() {
        let (_, (springs, broken)) = parse_springs("?###???????? 3,2,1").unwrap();
        assert_eq!(arrangements(&springs, &broken), 10);
    }

    #[test]
    fn parse() {
        let parsed = super::parse(include_str!("../inputs/day12"));
        assert!(parsed.is_ok());
        let (rest, _) = parsed.unwrap();
        assert_eq!(rest, "");
    }
}
