crate::solution!(6, solve_a, solve_b);

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace0, one_of, space0, space1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, pair, preceded},
    IResult,
};

pub fn solve_a(input: &str) -> u64 {
    let (_, records) = parse_a(input).expect("Could not parse input");
    records.iter().map(|r| r.win_count()).product()
}

pub fn solve_b(input: &str) -> u64 {
    let (_, records) = parse_b(input).expect("Could not parse input");
    records.iter().map(|r| r.win_count()).sum()
}

#[derive(Debug)]
struct Record {
    time: u64,
    distance: u64,
}

impl Record {
    #[allow(dead_code)]
    fn possible_wins(&self) -> Vec<u64> {
        // d > h * (t - h)
        let mut res = vec![];
        for h in 0..self.time {
            let d = self.distance_traveled(h);
            if d > self.distance {
                res.push(h);
            } else if !res.is_empty() {
                break;
            }
        }
        res
    }

    fn win_count(&self) -> u64 {
        let (min, max) = self.min_max_time();
        max - min + 1
    }

    fn distance_traveled(&self, hold_time: u64) -> u64 {
        // d = h * (t - h)
        hold_time * (self.time - hold_time)
    }

    // https://www.wikiwand.com/en/Quadratic_programming
    fn min_max_time(&self) -> (u64, u64) {
        // distance = hold * (time - hold)
        // hold_1 = (time - sqrt(time^2 - 4*distance)) / 2
        // hold_2 = (time + sqrt(time^2 - 4*distance)) / 2

        let sqrt = ((self.time.pow(2) - (4 * self.distance)) as f64).sqrt();
        let hold_1 = ((self.time as f64 - sqrt) / 2.0).floor() as u64;
        let hold_2 = ((self.time as f64 + sqrt) / 2.0).ceil() as u64;

        (hold_1 + 1, hold_2 - 1)
    }
}

fn parse_a(input: &str) -> IResult<&str, Vec<Record>> {
    let mut numbers = separated_list1(space1, preceded(space0, complete::u64));
    let (input, time) = delimited(pair(tag("Time:"), space1), &mut numbers, line_ending)(input)?;
    let (input, distance) = delimited(pair(tag("Distance:"), space1), numbers, multispace0)(input)?;

    let records = time
        .into_iter()
        .zip(distance)
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
        assert_eq!(record.min_max_time(), (2, 5));
        assert_eq!(record.win_count(), 4);
    }

    #[test]
    fn record_2() {
        let record = Record {
            time: 15,
            distance: 40,
        };
        assert_eq!(record.possible_wins(), vec![4, 5, 6, 7, 8, 9, 10, 11]);
        assert_eq!(record.min_max_time(), (4, 11));
        assert_eq!(record.win_count(), 8);
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
        assert_eq!(record.min_max_time(), (11, 19));
        assert_eq!(record.win_count(), 9);
    }
}
