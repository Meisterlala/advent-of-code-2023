pub struct Day01a;

impl crate::Solution for Day01a {
    fn solve(&self) -> String {
        format!("{}", solve(include_str!("../inputs/day01")))
    }
}

fn solve(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let numbers: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        assert!(!numbers.is_empty(), "No numbers found in line: {}", line);

        sum += numbers.first().unwrap() * 10 + numbers.last().unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn individual() {
        assert_eq!(solve("1abc2"), 12);
        assert_eq!(solve("pqr3stu8vwx"), 38);
        assert_eq!(solve("a1b2c3d4e5f"), 15);
        assert_eq!(solve("treb7uchet"), 77);
    }

    #[test]
    fn multiple() {
        assert_eq!(solve("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"), 142);
    }
}
