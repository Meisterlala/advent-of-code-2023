mod day_01a;
mod day_01b;
mod day_02a;
mod day_02b;
mod day_03a;
mod day_03b;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;

mod download_input;

pub static DAYS: &[&Day] = &[
    &day_01a::SOLUTION,
    &day_02a::SOLUTION,
    &day_03a::SOLUTION,
    &day_04::SOLUTION,
    &day_05::SOLUTION,
    &day_06::SOLUTION,
    &day_07::SOLUTION,
    &day_08::SOLUTION,
    &day_09::SOLUTION,
    &day_10::SOLUTION,
    &day_11::SOLUTION,
    &day_12::SOLUTION,
];

pub trait Solution: Send + Sync {
    fn solve(&self) -> String;
}

pub fn get_input(day: u32) -> String {
    std::fs::read_to_string(format!("inputs/day{:02}", day)).unwrap()
}

pub struct Day<'a> {
    pub day: u32,
    pub part1: Option<&'a dyn Solution>,
    pub part2: Option<&'a dyn Solution>,
}

#[macro_export]
macro_rules! solution {
    ($day:expr, $part1:expr) => {
        struct Part1;
        impl $crate::Solution for Part1 {
            fn solve(&self) -> String {
                debug_assert!($day >= 1 && $day <= 25);
                format!("{}", $part1(&($crate::get_input($day))))
            }
        }

        pub static SOLUTION: $crate::Day = $crate::Day {
            day: $day,
            part1: Some(&Part1),
            part2: None,
        };
    };

    ($day:expr, $part1:expr, $part2:expr) => {
        struct Part1;
        impl $crate::Solution for Part1 {
            fn solve(&self) -> String {
                debug_assert!($day >= 1 && $day <= 25);
                format!("{}", $part1(&($crate::get_input($day))))
            }
        }

        struct Part2;
        impl $crate::Solution for Part2 {
            fn solve(&self) -> String {
                debug_assert!($day >= 1 && $day <= 25);
                format!("{}", $part2(&($crate::get_input($day))))
            }
        }

        pub static SOLUTION: $crate::Day = $crate::Day {
            day: $day,
            part1: Some(&Part1),
            part2: Some(&Part2),
        };
    };
}
