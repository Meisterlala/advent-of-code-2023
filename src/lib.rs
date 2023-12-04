mod day_01a;
mod day_01b;

mod day_02a;
mod day_02b;

mod day_03a;
mod day_03b;

mod day_04a;

pub static DAYS: &[Day] = &[
    Day {
        day: 1,
        part1: Some(&day_01a::Day01a),
        part2: Some(&day_01b::Day01b),
    },
    Day {
        day: 2,
        part1: Some(&day_02a::Day02a),
        part2: Some(&day_02b::Day02b),
    },
    Day {
        day: 3,
        part1: Some(&day_03a::Day03a),
        part2: Some(&day_03b::Day03b),
    },
    Day {
        day: 4,
        part1: Some(&day_04a::Day04a),
        part2: None,
    },
];

pub trait Solution: Send + Sync {
    fn solve(&self) -> String;
}

pub struct Day<'a> {
    pub day: u32,
    pub part1: Option<&'a dyn Solution>,
    pub part2: Option<&'a dyn Solution>,
}
