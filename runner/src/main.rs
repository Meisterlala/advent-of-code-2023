use std::time::{Duration, Instant};

pub struct Day {
    pub day: u8,
    pub part1: Option<fn() -> ()>,
    pub part2: Option<fn() -> ()>,
}

impl Day {
    const fn new(day: u8, part1: Option<fn() -> ()>, part2: Option<fn() -> ()>) -> Day {
        Day { day, part1, part2 }
    }
}

pub static DAYS: &[Day] = &[
    Day::new(1, Some(day01a::main), Some(day01b::main)),
    Day::new(2, Some(day02a::main), Some(day02b::main)),
    Day::new(3, Some(day03a::main), None),
];

#[allow(dead_code)]
fn main() {
    println!("Running Advent of Code 2023...");

    let mut timings: Vec<String> = vec![];

    for day in DAYS {
        let mut running_time = Duration::new(0, 0);

        if let Some(p1) = day.part1 {
            print!("Day {:2} Part 1:  ", day.day);
            let timer = Instant::now();
            (p1)();
            running_time += timer.elapsed();
            println!()
        }

        if let Some(p2) = day.part2 {
            print!("Day {:2} Part 2:  ", day.day);
            let timer = Instant::now();
            (p2)();
            running_time += timer.elapsed();
            println!()
        }

        timings.push(format!("Day {}: {} ms", day.day, format_time(running_time)));
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
