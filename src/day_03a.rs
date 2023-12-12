crate::solution!(3, solve, crate::day_03b::solve);

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    multi::{many0, many0_count},
    IResult,
};

pub fn solve(input: &str) -> u32 {
    let arr: Vec<Vec<char>> = to_array(input);

    let valid_numbers = all_valid_numbers(&arr);
    valid_numbers.iter().sum::<u32>() as u32
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
    fn example() {
        assert_eq!(solve(EXAMPLE), 4361);
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
