use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    IResult,
};

pub struct Day09a;
pub struct Day09b;

impl crate::Solution for Day09a {
    fn solve(&self) -> String {
        format!("{}", solve_a(include_str!("../inputs/day09")))
    }
}

impl crate::Solution for Day09b {
    fn solve(&self) -> String {
        format!("{}", solve_b(include_str!("../inputs/day09")))
    }
}

fn solve_a(input: &str) -> i64 {
    let (_, history) = parse_input(input).expect("Failed to parse input");
    history.iter().map(|line| next_value(line)).sum()
}

fn solve_b(input: &str) -> i64 {
    let (_, history) = parse_input(input).expect("Failed to parse input");
    history.iter().map(|line| prev_value(line)).sum()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    let history = separated_list1(space1, complete::i64);
    separated_list1(newline, history)(input)
}

fn next_value(values: &[i64]) -> i64 {
    if values.len() < 2 {
        return 0;
    }
    let differences: Vec<_> = values.windows(2).map(|w| w[1] - w[0]).collect();
    if differences.iter().all(|d| *d == 0) {
        *values.last().unwrap()
    } else {
        next_value(&differences) + values.last().unwrap()
    }
}

fn prev_value(values: &[i64]) -> i64 {
    if values.len() < 2 {
        return 0;
    }
    let differences: Vec<_> = values.windows(2).map(|w| w[1] - w[0]).collect();
    if differences.iter().all(|d| *d == 0) {
        *values.first().unwrap()
    } else {
        values.first().unwrap() - prev_value(&differences)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn example_a() {
        assert_eq!(solve_a(EXAMPLE), 114);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(EXAMPLE), 2);
    }

    #[test]
    fn history_1_a() {
        assert_eq!(next_value(&[0, 3, 6, 9, 12, 15]), 18);
    }

    #[test]
    fn history_1_b() {
        assert_eq!(prev_value(&[0, 3, 6, 9, 12, 15]), -3);
    }

    #[test]
    fn history_2_a() {
        assert_eq!(next_value(&[1, 3, 6, 10, 15, 21]), 28);
    }

    #[test]
    fn history_2_b() {
        assert_eq!(prev_value(&[1, 3, 6, 10, 15, 21]), 0);
    }

    #[test]
    fn history_3_a() {
        assert_eq!(next_value(&[10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn history_3_b() {
        assert_eq!(prev_value(&[10, 13, 16, 21, 30, 45]), 5);
    }

    #[test]
    fn parse() {
        assert_eq!(
            parse_input(EXAMPLE),
            Ok((
                "",
                vec![
                    vec![0, 3, 6, 9, 12, 15],
                    vec![1, 3, 6, 10, 15, 21],
                    vec![10, 13, 16, 21, 30, 45]
                ]
            ))
        );
    }
}
