crate::solution!(3, solve_a, solve_b);

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    multi::{many0, many0_count},
    IResult,
};

pub fn solve_a(input: &str) -> u32 {
    let arr: Vec<Vec<char>> = to_array(input);

    let valid_numbers = all_valid_numbers(&arr);
    valid_numbers.iter().sum::<u32>() as u32
}

pub fn solve_b(input: &str) -> u32 {
    // Sum of all products between two numbers that are connected by a gear ('*')
    let mut sum = 0;

    // Find all numbers adjacent to the gear
    let mut adjacent_numbers = Vec::with_capacity(2);

    // Iterate over the lines in the input
    for (current_line_index, current_line_content) in input.lines().enumerate() {
        // Determine the index of the lines before and after the gear
        let line_above_index = current_line_index.checked_sub(1);
        let line_below_index = current_line_index + 1;

        // Iterate over the characters in the line and find the gears
        let char_indices_peekable = current_line_content.char_indices().peekable();
        for (index, c) in char_indices_peekable {
            // Skip non-gear characters
            if c != '*' {
                continue;
            }

            // Search for numbers to the left of the gear
            if let Some(number) = current_line_content[..index]
                .char_indices()
                .rev()
                .take_while(|(_, c)| c.is_ascii_digit())
                .last()
                .and_then(|(index, _)| {
                    current_line_content[index..]
                        .chars()
                        .take_while(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<u32>()
                        .ok()
                })
            {
                adjacent_numbers.push(number)
            }

            // Search for numbers to the right of the gear
            if let Ok(number) = current_line_content[(index + 1)..]
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u32>()
            {
                adjacent_numbers.push(number)
            }

            // Search for numbers above the gear
            if let Some(line_above_index) = line_above_index {
                // Find the left-most digit above the gear
                let numbers_start_index = input
                    .lines()
                    .nth(line_above_index)
                    .and_then(|line| {
                        line[..=index.saturating_sub(1)]
                            .char_indices()
                            .rev()
                            .take_while(|(_, c)| c.is_ascii_digit())
                            .last()
                            .map(|(i, _)| i)
                    })
                    .unwrap_or(index.saturating_sub(1));
                // Find the right-most digit above the gear
                let numbers_end_index = input
                    .lines()
                    .nth(line_above_index)
                    .and_then(|line| {
                        line[index.saturating_add(1)..]
                            .char_indices()
                            .take_while(|(_, c)| c.is_ascii_digit())
                            .last()
                            .map(|(i, _)| index.saturating_add(2) + i)
                    })
                    .unwrap_or(index.saturating_add(1));

                // Parse all numbers above the gear
                if let Some(line) = input.lines().nth(line_above_index) {
                    line[numbers_start_index..numbers_end_index]
                        .split(|c: char| !c.is_ascii_digit())
                        .filter_map(|number| number.parse::<u32>().ok())
                        .for_each(|number| adjacent_numbers.push(number))
                }
            };

            // Search for numbers below the gear
            {
                // Find the left-most digit below the gear
                let numbers_start_index = input
                    .lines()
                    .nth(line_below_index)
                    .and_then(|line| {
                        line[..=index.saturating_sub(1)]
                            .char_indices()
                            .rev()
                            .take_while(|(_, c)| c.is_ascii_digit())
                            .last()
                            .map(|(i, _)| i)
                    })
                    .unwrap_or(index.saturating_sub(1));
                // Find the right-most digit below the gear
                let numbers_end_index = input
                    .lines()
                    .nth(line_below_index)
                    .and_then(|line| {
                        line[index.saturating_add(1)..]
                            .char_indices()
                            .take_while(|(_, c)| c.is_ascii_digit())
                            .last()
                            .map(|(i, _)| index.saturating_add(2) + i)
                    })
                    .unwrap_or(index.saturating_add(1));

                // Parse all numbers below the gear
                if let Some(line) = input.lines().nth(line_below_index) {
                    line[numbers_start_index..numbers_end_index]
                        .split(|c: char| !c.is_ascii_digit())
                        .filter_map(|number| number.parse::<u32>().ok())
                        .for_each(|number| adjacent_numbers.push(number))
                }
            }

            // Add the product of the adjacent numbers to the sum if there are exactly two adjacent numbers
            if adjacent_numbers.len() == 2 {
                sum += adjacent_numbers[0] * adjacent_numbers[1];
            }
            adjacent_numbers.clear();
        }
    }
    sum
}

fn all_valid_numbers(arr: &Vec<Vec<char>>) -> Vec<u32> {
    let mut valid = Vec::new();

    for x in 0..arr.len() {
        let line: String = arr[x].iter().collect();
        let numbers = parse_numbers(&line).unwrap().1;
        for number in numbers {
            let index2d: Vec<_> = number.indexes.iter().map(|i| (x, *i)).collect();
            if number_has_adjacent_symbol(arr, &index2d) {
                valid.push(number.number);
            }
        }
    }
    valid
}

#[derive(Debug, PartialEq)]
struct Number {
    indexes: Vec<usize>,
    parsed_length: usize,
    number: u32,
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<Number>> {
    let (input, numbers) = many0(parse_number)(input)?;

    let mut new_numbers = Vec::new();
    let mut last_end = 0;

    // Update indexes
    for n in numbers {
        new_numbers.push(Number {
            indexes: n.indexes.iter().map(|i| i + last_end).collect(),
            parsed_length: n.parsed_length,
            number: n.number,
        });
        last_end += n.parsed_length;
    }

    Ok((input, new_numbers))
}

fn parse_number(input: &str) -> IResult<&str, Number> {
    let (i, pre) = many0_count(alt((tag("."), tag("x"))))(input)?;
    let (i, number) = map_res(digit1, str::parse::<u32>)(i)?;
    let (i, suf) = many0_count(alt((tag("."), tag("x"))))(i)?;

    let indexs = (pre..(input.chars().count() - i.chars().count() - suf)).collect();

    Ok((
        i,
        Number {
            indexes: indexs,
            parsed_length: input.chars().count() - i.chars().count(),
            number,
        },
    ))
}

fn number_has_adjacent_symbol(arr: &[Vec<char>], pos: &[(usize, usize)]) -> bool {
    pos.iter().any(|p| has_adjacent_symbol(arr, *p))
}

fn has_adjacent_symbol(arr: &[Vec<char>], pos: (usize, usize)) -> bool {
    // 3x3 grid around pos (check for out of bounds)
    let start_x = if pos.0 > 0 { pos.0 - 1 } else { 0 };
    let start_y = if pos.1 > 0 { pos.1 - 1 } else { 0 };

    arr.iter()
        .skip(start_x)
        .take(3)
        .any(|row| row.iter().skip(start_y).take(3).any(|c| *c == 'x'))
}

/// Converts the input to a 2D array of chars, replacing all symbols with 'x'.
fn to_array(input: &str) -> Vec<Vec<char>> {
    let mut chars = Vec::new();
    for line in input.lines() {
        let mut line_chars = Vec::new();
        for c in line.trim().chars() {
            if c != '.' && !c.is_numeric() {
                line_chars.push('x');
            } else {
                line_chars.push(c);
            }
        }
        chars.push(line_chars);
    }
    chars
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn example_a() {
        assert_eq!(solve_a(EXAMPLE), 4361);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(EXAMPLE), 467835);
    }

    #[test]
    fn array() {
        assert_eq!(
            to_array(EXAMPLE),
            vec![
                vec!['4', '6', '7', '.', '.', '1', '1', '4', '.', '.'],
                vec!['.', '.', '.', 'x', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '3', '5', '.', '.', '6', '3', '3', '.'],
                vec!['.', '.', '.', '.', '.', '.', 'x', '.', '.', '.'],
                vec!['6', '1', '7', 'x', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', 'x', '.', '5', '8', '.'],
                vec!['.', '.', '5', '9', '2', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '7', '5', '5', '.'],
                vec!['.', '.', '.', 'x', '.', 'x', '.', '.', '.', '.'],
                vec!['.', '6', '6', '4', '.', '5', '9', '8', '.', '.'],
            ]
        );
    }

    #[test]
    fn parsing() {
        assert_eq!(
            parse_number("..123.x14.155"),
            Ok((
                "14.155",
                Number {
                    indexes: vec![2, 3, 4],
                    parsed_length: "..123.x14.155".chars().count() - "14.155".chars().count(),
                    number: 123
                }
            ))
        );
    }

    #[test]
    fn parse_number2() {
        assert_eq!(
            parse_number("...123.x14.155"),
            Ok((
                "14.155",
                Number {
                    indexes: vec![3, 4, 5],
                    parsed_length: "...123.x14.155".chars().count() - "14.155".chars().count(),
                    number: 123
                }
            ))
        );
    }

    #[test]
    fn parse_multible_numbers() {
        assert_eq!(
            parse_numbers("...123.x14.155"),
            Ok((
                "",
                vec![
                    Number {
                        indexes: vec![3, 4, 5],
                        parsed_length: 8,
                        number: 123
                    },
                    Number {
                        indexes: vec![8, 9],
                        parsed_length: 3,
                        number: 14
                    },
                    Number {
                        indexes: vec![11, 12, 13],
                        parsed_length: 3,
                        number: 155
                    }
                ]
            ))
        );
    }
}
