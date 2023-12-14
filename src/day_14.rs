use itertools::Itertools;
use nom::Parser;

crate::solution!(14, solve_a);

pub fn solve_a(input: &str) -> usize {
    let (_, patterns) = parse(input).unwrap();

    let line_len = patterns.len();
    let binding = transpose(&patterns);
    let trans = binding.iter().map(|line| line.chars());

    tilt_left(trans)
        .map(|collum| {
            collum
                .enumerate()
                .filter(|(_, c)| *c == 'O')
                .map(|(i, _)| line_len - i)
                .sum::<usize>()
        })
        .sum()
}

fn parse(input: &str) -> nom::IResult<&str, Vec<String>> {
    use nom::{bytes::complete::is_a, character::complete::line_ending, multi::separated_list1};

    separated_list1(line_ending, is_a("O.#").map(String::from))(input)
}

fn _print_tiltet(pattern: &[String]) {
    dbg!(tilt_left(pattern.iter().map(|line| line.chars()))
        .map(|line| line.collect())
        .collect::<Vec<String>>());
}

fn transpose(pattern: &[String]) -> Vec<String> {
    (0..pattern[0].len())
        .map(|i| {
            pattern
                .iter()
                .map(|n| n.chars().nth(i).unwrap())
                .collect::<String>()
        })
        .collect()
}

// OO.O.O..##
// ...OO....O
// .O...#O..O
// .O.#......
// .#.O......
// #.#..O#.##
// ..#...O.#.
// ....O#.O#.
// ....#.....
// .#.O.#O...

fn tilt_left<'a>(
    pattern: impl Iterator<Item = impl Iterator<Item = char>> + 'a,
) -> impl Iterator<Item = impl Iterator<Item = char> + 'a> + 'a {
    pattern.map(|line| tilt_line_left(line))
}

fn tilt_line_left<'a>(line: impl Iterator<Item = char>) -> impl Iterator<Item = char> + 'a {
    line.group_by(|c| *c != '#')
        .into_iter()
        .flat_map(move |(free_space, g)| {
            if free_space {
                let (free_spaces, round_rock): (Vec<char>, Vec<char>) = g.partition(|c| *c == '.');
                round_rock
                    .into_iter()
                    .chain(free_spaces)
                    .collect::<Vec<char>>()
            } else {
                g.collect::<Vec<char>>()
            }
        })
        .collect::<Vec<char>>()
        .into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn example_a() {
        assert_eq!(solve_a(EXAMPLE), 136);
    }

    #[test]
    fn parse() {
        let (i, patterns) = super::parse(EXAMPLE).unwrap();
        assert!(i.is_empty());
        assert_eq!(patterns.len(), 10);
        assert_eq!(patterns[0], "O....#....");
        assert_eq!(patterns[9], "#OO..#....");
    }

    #[test]
    fn transpose() {
        let (i, patterns) = super::parse(EXAMPLE).unwrap();
        assert!(i.is_empty());

        dbg!(super::transpose(&patterns));

        assert_eq!(super::transpose(&super::transpose(&patterns)), patterns);
    }
}
