crate::solution!(11, solve_a, solve_b_million);

use itertools::Itertools;
use nom::{
    character::complete::{line_ending, none_of},
    multi::{many1, separated_list1},
    IResult,
};

pub fn solve_a(input: &str) -> u64 {
    let (_, mut map) = parse_input(input).expect("Failed to parse input");

    // Expand empty rows and columns
    let empty_columns: Vec<_> = (0..map[0].len())
        .filter(|y| map.iter().all(|row| row[*y] == '.'))
        .rev()
        .collect();

    let expanded = map
        .iter_mut()
        .flat_map(|row| {
            empty_columns.iter().for_each(|&y| row.insert(y, '.'));
            if row.iter().all(|&c| c == '.') {
                vec![row.clone(), row.clone()]
            } else {
                vec![row.clone()]
            }
        })
        .collect::<Vec<Vec<char>>>();

    let galaxies = expanded
        .iter()
        .enumerate()
        .flat_map(|(x, r)| {
            r.iter().enumerate().filter_map(move |(y, c)| {
                if *c == '#' {
                    Some((x as i64, y as i64))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<(i64, i64)>>();

    let distances = galaxies
        .iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| {
            let x = (x1 - x2).abs();
            let y = (y1 - y2).abs();
            (x + y) as u64
        });

    distances.sum()
}

pub fn solve_b_million(input: &str) -> u64 {
    solve_b(input, 1_000_000)
}

pub fn solve_b(input: &str, expansion: u64) -> u64 {
    let (_, map) = parse_input(input).expect("Failed to parse input");

    // Expand empty rows and columns
    let empty_columns: Vec<_> = (0..map[0].len())
        .filter(|y| map.iter().all(|row| row[*y] == '.'))
        .map(|y| y as u64)
        .collect();

    let empty_row: Vec<_> = map
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&c| c == '.'))
        .map(|(x, _)| x as u64)
        .collect();

    let galaxies = map
        .iter()
        .enumerate()
        .flat_map(|(x, r)| {
            r.iter().enumerate().filter_map(move |(y, c)| {
                if *c == '#' {
                    Some((x as u64, y as u64))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<(u64, u64)>>();

    // 1   29
    // 5 18 100

    let distances = galaxies
        .iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| {
            let y_expanded = empty_columns
                .iter()
                .filter(|&y| y > y1.min(y2) && y < y2.max(y1))
                .count() as u64;
            let x_expanded = empty_row
                .iter()
                .filter(|&x| x > x1.min(x2) && x < x2.max(x1))
                .count() as u64;

            let x_dist = x1.max(x2) - x1.min(x2);
            let y_dist = y1.max(y2) - y1.min(y2);

            let scaled_x = x_dist - x_expanded + (x_expanded * expansion);
            let scaled_y = y_dist - y_expanded + (y_expanded * expansion);
            scaled_x + scaled_y
        });

    distances.sum()
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
    fn example_b_1() {
        assert_eq!(solve_b(EXAMPLE_A, 10), 1030);
    }

    #[test]
    fn example_b_2() {
        assert_eq!(solve_b(EXAMPLE_A, 100), 8410);
    }
}
