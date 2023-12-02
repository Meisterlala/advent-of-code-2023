fn main() {
    println!("sum: {}", solve(include_str!("../input.txt")));
}

fn solve(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let mut numbers: Vec<u32> = Vec::new();

        // Iterate over all substrings of line
        for sub_start in 0..line.len() {
            // with any length
            for sub_end in sub_start..line.len() {
                let substring = &line[sub_start..(sub_end + 1)];

                // If substring is a number, add it to the list of numbers
                if let Some(number) = parse_substring(substring) {
                    numbers.push(number);
                    break;
                }
            }
        }

        assert!(!numbers.is_empty(), "No numbers found in line: {}", line);
        // The first and last numbers are the ones we want to add
        let line_sum: u32 = numbers.first().unwrap() * 10 + numbers.last().unwrap();
        sum += line_sum;
    }
    sum
}

fn spelled_digit(input: &str) -> Option<u32> {
    match input {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn parse_substring(input: &str) -> Option<u32> {
    if input.len() == 1 {
        input.parse::<u32>().ok()
    } else {
        spelled_digit(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_29() {
        assert_eq!(solve("two1nine"), 29);
    }

    #[test]
    fn test_83() {
        assert_eq!(solve("eightwothree"), 83);
    }

    #[test]
    fn test_13() {
        assert_eq!(solve("abcone2threexyz"), 13);
    }

    #[test]
    fn test_24() {
        assert_eq!(solve("xtwone3four"), 24);
    }

    #[test]
    fn test_42() {
        assert_eq!(solve("4nineeightseven2"), 42);
    }

    #[test]
    fn test_14() {
        assert_eq!(solve("zoneight234"), 14);
    }

    #[test]
    fn test_76() {
        assert_eq!(solve("7pqrstsixteen"), 76);
    }

    #[test]
    fn test_sum() {
        assert_eq!(solve("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"), 281);
    }

    #[test]
    fn test_18() {
        assert_eq!(solve("oneight"), 18);
    }
}
