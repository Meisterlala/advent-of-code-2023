use std::collections::VecDeque;

use nom::{
    character::complete::{line_ending, none_of},
    multi::{many1, separated_list1},
    IResult,
};

pub struct Day10a;
pub struct Day10b;

impl crate::Solution for Day10a {
    fn solve(&self) -> String {
        format!("{}", solve_a(include_str!("../inputs/day10")))
    }
}

impl crate::Solution for Day10b {
    fn solve(&self) -> String {
        format!("{}", solve_b(include_str!("../inputs/day10")))
    }
}

fn solve_a(input: &str) -> u32 {
    let (_, graph) = parse_input(input).expect("Failed to parse input");

    // Find the starting point
    let start = graph
        .iter()
        .enumerate()
        .find_map(|(x, row)| row.iter().position(|&c| c == 'S').map(|y| (x, y)))
        .expect("No starting point found");

    let distances = bfs(&graph, start);

    distances
        .into_iter()
        .flatten()
        .max()
        .unwrap()
        .saturating_sub(1)
}

fn solve_b(input: &str) -> u32 {
    let (_, graph) = parse_input(input).expect("Failed to parse input");

    // Find the starting point
    let start = graph
        .iter()
        .enumerate()
        .find_map(|(x, row)| row.iter().position(|&c| c == 'S').map(|y| (x, y)))
        .expect("No starting point found");

    let distances = bfs(&graph, start);

    // All free spaces
    let possible_inside = distances.iter().enumerate().flat_map(|(x, row)| {
        row.iter()
            .enumerate()
            .filter_map(move |(y, d)| if *d == 0 { Some((x, y)) } else { None })
    });

    // Find all free spaces that are inside
    let inside = possible_inside.filter_map(|(x, y)| {
        let crossings_right = graph[x]
            .iter()
            .enumerate()
            .skip(y)
            .filter(|(g_y, c)| distances[x][*g_y] > 0 && !matches!(**c, '-' | 'J' | 'L'))
            .count();

        if crossings_right % 2 == 1 {
            Some((x, y))
        } else {
            None
        }
    });

    // Find all free spaces that are inside
    inside.count() as u32
}

fn bfs(graph: &[Vec<char>], start: (usize, usize)) -> Vec<Vec<u32>> {
    let mut positions = VecDeque::from([start]);
    let mut distances: Vec<Vec<u32>> = vec![vec![0; graph[0].len()]; graph.len()];

    distances[start.0][start.1] = 1;

    while let Some((start_x, start_y)) = positions.pop_front() {
        let distance = distances[start_x][start_y];
        let current = graph[start_x][start_y];

        if start_y > 0 {
            // Left
            let (n_x, n_y) = (start_x, start_y - 1);
            if distances[n_x][n_y] == 0
                && matches!(
                    (current, graph[n_x][n_y]),
                    ('S', '-')
                        | ('S', 'F')
                        | ('S', 'L')
                        | ('-', '-')
                        | ('-', 'F')
                        | ('-', 'L')
                        | ('J', '-')
                        | ('J', 'F')
                        | ('J', 'L')
                        | ('7', '-')
                        | ('7', 'F')
                        | ('7', 'L')
                )
            {
                distances[n_x][n_y] = distance + 1;
                positions.push_back((n_x, n_y));
            }
        }
        if (start_y + 1) < graph[0].len() {
            // Right
            let (n_x, n_y) = (start_x, start_y + 1);
            if distances[n_x][n_y] == 0
                && matches!(
                    (current, graph[n_x][n_y]),
                    ('S', '-')
                        | ('S', 'J')
                        | ('S', '7')
                        | ('-', '-')
                        | ('-', 'J')
                        | ('-', '7')
                        | ('F', '-')
                        | ('F', 'J')
                        | ('F', '7')
                        | ('L', '-')
                        | ('L', 'J')
                        | ('L', '7')
                )
            {
                distances[n_x][n_y] = distance + 1;
                positions.push_back((n_x, n_y));
            }
        }
        if start_x > 0 {
            // Up
            let (n_x, n_y) = (start_x - 1, start_y);
            if distances[n_x][n_y] == 0
                && matches!(
                    (current, graph[n_x][n_y]),
                    ('S', '|')
                        | ('S', 'F')
                        | ('S', '7')
                        | ('|', '|')
                        | ('|', 'F')
                        | ('|', '7')
                        | ('L', '|')
                        | ('L', 'F')
                        | ('L', '7')
                        | ('J', '|')
                        | ('J', 'F')
                        | ('J', '7')
                )
            {
                distances[n_x][n_y] = distance + 1;
                positions.push_back((n_x, n_y));
            }
        }
        if (start_x + 1) < graph.len() {
            // Down
            let (n_x, n_y) = (start_x + 1, start_y);
            if distances[n_x][n_y] == 0
                && matches!(
                    (current, graph[n_x][n_y]),
                    ('S', '|')
                        | ('S', 'J')
                        | ('S', 'L')
                        | ('|', '|')
                        | ('|', 'J')
                        | ('|', 'L')
                        | ('F', '|')
                        | ('F', 'J')
                        | ('F', 'L')
                        | ('7', '|')
                        | ('7', 'J')
                        | ('7', 'L')
                )
            {
                distances[n_x][n_y] = distance + 1;
                positions.push_back((n_x, n_y));
            }
        }
    }

    distances
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(line_ending, many1(none_of("\n\r")))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...";
        assert_eq!(solve_a(input), 8);
    }

    #[test]
    fn example_2() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(solve_b(input), 10);
    }

    #[test]
    fn example_3() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(solve_b(input), 8);
    }

    #[test]
    fn example_4() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(solve_b(input), 4);
    }

    #[test]
    fn parse() {
        let input = "-L|F7\n7S-7|\nL|7||\n-L-J|\nL|-JF";
        let expected = vec![
            vec!['-', 'L', '|', 'F', '7'],
            vec!['7', 'S', '-', '7', '|'],
            vec!['L', '|', '7', '|', '|'],
            vec!['-', 'L', '-', 'J', '|'],
            vec!['L', '|', '-', 'J', 'F'],
        ];
        assert_eq!(parse_input(input), Ok(("", expected)));
    }
}
