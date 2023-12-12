use advent_of_code_2023::{Day, DAYS};

mod download_input;

fn main() {
    println!("Running Advent of Code 2023...\n");
    println!("  Day  |  Part  | Solution");
    println!("-------+--------+------------------");

    if let Some(day) = std::env::args().nth(1) {
        if let Some(day) = parse_day(&day) {
            if let Some(day) = DAYS.iter().find(|d| d.day == day) {
                run_day(day);
            } else {
                println!("Day {} not found", day);
            }
            return;
        }
    }

    run_all();
}

fn run_day(day: &Day) {
    if !download_input::check_if_present(day.day) {
        println!("Day {:2} | Downloading Input ...", day.day);
        if let Err(e) = download_input::download_input(day.day) {
            let err = format!("Day {:2} | Failed to download input", day.day);
            println!("{err}: {e}");
            return;
        }
    }

    if let Some(p1) = day.part1 {
        println!("Day {:2} | Part 1 | {}", day.day, p1.solve());
    }
    if let Some(p2) = day.part2 {
        println!("Day {:2} | Part 2 | {}", day.day, p2.solve());
    }
}

fn run_all() {
    for day in DAYS {
        run_day(day);
        println!("-------+--------+------------------");
    }
}

fn parse_day(input: &str) -> Option<u32> {
    let only_numbers: String = input.chars().filter(|c| c.is_numeric()).collect();
    only_numbers.parse::<u32>().ok()
}
