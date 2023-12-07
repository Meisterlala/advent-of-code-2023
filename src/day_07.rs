pub struct Day07a;

impl crate::Solution for Day07a {
    fn solve(&self) -> String {
        format!("{}", solve_a(include_str!("../inputs/day07")))
    }
}

fn solve_a(input: &str) -> u64 {
    todo!()
}

struct Card {
    label: char,
    strength: u64,
}

impl Card {
    fn new(label: char) -> Self {
        let strength = match label {
            '2' => 0,
            '3' => 1,
            '4' => 2,
            '5' => 3,
            '6' => 4,
            '7' => 5,
            '8' => 6,
            '9' => 7,
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => panic!("Invalid card label: {}", label),
        };
        Self { label, strength }
    }
}
