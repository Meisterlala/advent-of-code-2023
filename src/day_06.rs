use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace0, one_of, space0, space1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, pair, preceded},
    IResult,
};

pub struct Day06a;

impl crate::Solution for Day06a {
    fn solve(&self) -> String {
        format!("{}", solve_a(include_str!("../inputs/day06")))
    }
}

pub struct Day06b;

impl crate::Solution for Day06b {
    fn solve(&self) -> String {
        format!("{}", solve_b(include_str!("../inputs/day06")))
    }
}

fn solve_a(input: &str) -> usize {
    let (_, records) = parse_a(input).expect("Could not parse input");
    records.iter().map(|r| r.possible_wins().len()).product()
}

fn solve_b(input: &str) -> usize {
    let (_, records) = parse_b(input).expect("Could not parse input");
    records.iter().map(|r| r.possible_wins().len()).sum()
}

#[derive(Debug)]
struct Record {
    time: u64,
    distance: u64,
}

impl Record {
    /// List of possible starting times to beat the record
    fn possible_wins(&self) -> Vec<u64> {
        (0..self.time)
            .map(|h| (h, self.time_to_win(h)))
            .filter(|(_, t)| *t < self.time)
            .map(|(h, _)| h)
            .collect()
    }

    fn time_to_win(&self, hold_time: u64) -> u64 {
        if hold_time == 0 {
            return self.distance;
        }
        hold_time + self.distance / hold_time
    }
}

fn parse_a(input: &str) -> IResult<&str, Vec<Record>> {
    let mut numbers = separated_list1(space1, preceded(space0, complete::u64));
    let (input, time) = delimited(pair(tag("Time:"), space1), &mut numbers, line_ending)(input)?;
    let (input, distance) = delimited(pair(tag("Distance:"), space1), numbers, multispace0)(input)?;

    let records = time
        .into_iter()
        .zip(distance.into_iter())
        .map(|(time, distance)| Record { time, distance })
        .collect();

    Ok((input, records))
}

fn parse_b(input: &str) -> IResult<&str, Vec<Record>> {
    let mut numbers = many1(delimited(
        space0,
        map(one_of("0123456789"), |c| c.to_digit(10).unwrap() as u64),
        space0,
    ));
    let (input, time) = delimited(pair(tag("Time:"), space1), &mut numbers, line_ending)(input)?;
    let (input, distance) = delimited(pair(tag("Distance:"), space1), numbers, multispace0)(input)?;

    let convert = |v: Vec<u64>| {
        v.into_iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, s)| acc + s * 10u64.pow(i as u32))
    };

    Ok((
        input,
        vec![Record {
            time: convert(time),
            distance: convert(distance),
        }],
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn example_a() {
        assert_eq!(solve_a(EXAMPLE), 288);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(EXAMPLE), 71503);
    }

    #[test]
    fn parse_a() {
        match super::parse_a(EXAMPLE) {
            Ok((input, records)) => {
                assert_eq!(input, "");
                assert_eq!(records.len(), 3);
                assert_eq!(records[0].time, 7);
                assert_eq!(records[0].distance, 9);
                assert_eq!(records[1].time, 15);
                assert_eq!(records[1].distance, 40);
                assert_eq!(records[2].time, 30);
                assert_eq!(records[2].distance, 200);
            }
            Err(e) => {
                dbg!(e);
                panic!("Count not parse example");
            }
        }
    }

    #[test]
    fn parse_b() {
        match super::parse_b(EXAMPLE) {
            Ok((input, records)) => {
                assert_eq!(input, "");
                assert_eq!(records.len(), 1);
                assert_eq!(records[0].time, 71530);
                assert_eq!(records[0].distance, 940200);
            }
            Err(e) => {
                dbg!(e);
                panic!("Count not parse example");
            }
        }
    }

    #[test]
    fn record_1() {
        let record = Record {
            time: 7,
            distance: 9,
        };
        assert_eq!(record.possible_wins(), vec![2, 3, 4, 5]);
    }

    #[test]
    fn record_2() {
        let record = Record {
            time: 15,
            distance: 40,
        };
        assert_eq!(record.possible_wins(), vec![4, 5, 6, 7, 8, 9, 10, 11]);
    }

    #[test]
    fn record_3() {
        let record = Record {
            time: 30,
            distance: 200,
        };
        assert_eq!(
            record.possible_wins(),
            vec![11, 12, 13, 14, 15, 16, 17, 18, 19]
        );
    }
}
