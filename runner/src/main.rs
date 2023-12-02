use took::Timer;

struct Day {
    day: u8,
    part1: fn() -> (),
    part2: fn() -> (),
}

static DAYS: &[Day] = &[Day {
    day: 1,
    part1: day01a::main,
    part2: day01b::main,
}];

fn main() {
    println!("Running Advent of Code 2023...");

    let mut timings: Vec<String> = vec![];

    for d in DAYS {
        let timer = Timer::new();

        print!("Day {:2} Part 1:  ", d.day);
        (d.part1)();

        print!("\nDay {:2} Part 2:  ", d.day);
        (d.part2)();

        timings.push(format!("Day {}: {}", d.day, timer));
        println!();
    }

    println!("\nTimings:");
    for t in timings {
        println!("{}", t);
    }
}
