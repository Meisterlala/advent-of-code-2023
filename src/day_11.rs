use std::collections::VecDeque;

use itertools::Itertools;
use nom::{
    character::complete::{line_ending, none_of},
    multi::{many1, separated_list1},
    ErrorConvert, IResult,
};

pub struct Day11a;
pub struct Day11b;

impl crate::Solution for Day11a {
    fn solve(&self) -> String {
        format!("{}", solve_a(include_str!("../inputs/day11")))
    }
}

impl crate::Solution for Day11b {
    fn solve(&self) -> String {
        todo!();
        // format!("{}", solve_b(include_str!("../inputs/day11")))
    }
}

fn solve_a(input: &str) -> u64 {
    let (_, map) = parse_input(input).expect("Failed to parse input");

    // Expand empty rows and columns
    let expanded = expand(map);

    let galaxies = expanded.iter().enumerate().flat_map(|(x, row)| {
        row.iter()
            .enumerate()
            .filter_map(move |(y, c)| if *c == '#' { Some((x, y)) } else { None })
    });

    let distances = galaxies.tuple_combinations().map(|((x1, y1), (x2, y2))| {
        let x = (x1 as i64 - x2 as i64).abs();
        let y = (y1 as i64 - y2 as i64).abs();
        (x + y) as u64
    });

    distances.sum()
}

fn expand(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let empty_columns: Vec<_> = (0..map[0].len())
        .filter(|y| map.iter().all(|row| row[*y] == '.'))
        .rev()
        .collect();

    map.iter_mut()
        .flat_map(|row| {
            empty_columns.iter().for_each(|&y| row.insert(y, '.'));
            if row.iter().all(|&c| c == '.') {
                vec![row.clone(), row.clone()]
            } else {
                vec![row.clone()]
            }
        })
        .collect()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(line_ending, many1(none_of("\n\r")))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_A: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn example_a() {
        assert_eq!(solve_a(EXAMPLE_A), 374);
    }

    #[test]
    fn expand() {
        let map = vec![
            vec!['#', '.', '.'],
            vec!['.', '.', '.'],
            vec!['.', '.', '#'],
        ];
        assert_eq!(
            super::expand(map),
            vec![
                vec!['#', '.', '.', '.'],
                vec!['.', '.', '.', '.'],
                vec!['.', '.', '.', '.'],
                vec!['.', '.', '.', '#'],
            ]
        );
    }

    #[test]
    fn expand_example() {
        let example = parse_input(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        )
        .unwrap()
        .1;

        let output = parse_input(
            "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......",
        )
        .unwrap()
        .1;

        assert_eq!(super::expand(example), output);
    }
}
