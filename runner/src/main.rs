use std::time::{Duration, Instant};

struct Day {
    day: u8,
    part1: fn() -> (),
    part2: fn() -> (),
}

impl Day {
    const fn new(day: u8, part1: fn() -> (), part2: fn() -> ()) -> Day {
        Day { day, part1, part2 }
    }
}

static DAYS: &[Day] = &[
    Day::new(1, day01a::main, day01b::main),
    Day::new(2, day02a::main, day02b::main),
];

fn main() {
    println!("Running Advent of Code 2023...");

    let mut timings: Vec<String> = vec![];

    for d in DAYS {
        let timer = Instant::now();

        print!("Day {:2} Part 1:  ", d.day);
        (d.part1)();

        print!("\nDay {:2} Part 2:  ", d.day);
        (d.part2)();

        timings.push(format!(
            "Day {}: {} ms",
            d.day,
            format_time(timer.elapsed())
        ));
        println!();
    }

    println!("\nTimings:");
    for t in timings {
        println!("{}", t);
    }
}

fn format_time(d: Duration) -> String {
    let time = d.as_millis() as f64 + (d.as_micros() - d.as_millis() * 1000) as f64 / 1000.0;
    format!("{:>8.3}", time)
}
