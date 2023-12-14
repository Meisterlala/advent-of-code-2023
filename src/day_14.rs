use itertools::Itertools;
use ndarray::prelude::*;
use std::collections::HashMap;

crate::solution!(14, solve_a, solve_b);

type Pattern = Array<Symbol, Ix2>;

pub fn solve_a(input: &str) -> usize {
    let (_, patterns) = parse(input).unwrap();
    let mut arr = to_array(patterns);

    tilt_north(&mut arr.view_mut());
    load(&arr.view())
}

pub fn solve_b(input: &str) -> usize {
    let (_, patterns) = parse(input).unwrap();
    let mut arr = to_array(patterns);

    let mut seen: HashMap<Pattern, usize> = HashMap::new();

    for i in 1..=1_000_000_000 {
        tilt_cycle(&mut arr.view_mut());

        if let Some(cycle) = seen.get(&arr) {
            let cycle_len = i - cycle;
            let remaining = (1_000_000_000 - i) % cycle_len;
            for _ in 0..remaining {
                tilt_cycle(&mut arr.view_mut());
            }
            break;
        }
        seen.insert(arr.clone(), i);
    }

    load(&arr.view())
}

fn load(arr: &ArrayView2<Symbol>) -> usize {
    let rows = arr.len_of(Axis(0));
    arr.axis_iter(Axis(0))
        .enumerate()
        .map(|(i, row)| (rows - i) * row.iter().filter(|&v| *v == Symbol::Round).count())
        .sum()
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Symbol {
    Cube,
    Round,
    Empty,
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Vec<Symbol>>> {
    use nom::{
        character::complete::line_ending, character::complete::one_of, multi::many1,
        multi::separated_list1, Parser,
    };

    separated_list1(
        line_ending,
        many1(one_of("O.#").map(|c| match c {
            'O' => Symbol::Round,
            '.' => Symbol::Empty,
            '#' => Symbol::Cube,
            _ => unreachable!(),
        })),
    )(input)
}

fn to_array(patterns: Vec<Vec<Symbol>>) -> Array2<Symbol> {
    Pattern::from_shape_vec(
        (patterns.len(), patterns[0].len()),
        patterns.into_iter().flatten().collect::<Vec<_>>(),
    )
    .unwrap()
}

fn tilt_cycle(arr: &mut ArrayViewMut2<Symbol>) {
    // North
    tilt_north(arr);
    // West
    tilt_north(&mut arr.view_mut().reversed_axes());
    // South
    tilt_north(&mut arr.slice_mut(s![..;-1 ,..;-1]));
    // East
    tilt_north(&mut arr.slice_mut(s![..;-1 ,..;-1]).reversed_axes());
}

fn tilt_north(arr: &mut ArrayViewMut2<Symbol>) {
    // pattern.map(|line| tilt_line_west(line))
    arr.columns_mut().into_iter().for_each(|mut line| {
        for (free_sapce, group) in &line
            .iter_mut()
            .group_by(|c| matches!(c, Symbol::Round | Symbol::Empty))
        {
            if free_sapce {
                let mut all = group.collect::<Vec<_>>();
                let rocks = all.iter().filter(|c| ***c == Symbol::Round).count();
                for (i, c) in all.iter_mut().enumerate() {
                    if i < rocks {
                        **c = Symbol::Round;
                    } else {
                        **c = Symbol::Empty;
                    }
                }
            }
        }
    });
}

impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Cube => write!(f, "#"),
            Symbol::Round => write!(f, "O"),
            Symbol::Empty => write!(f, "."),
        }
    }
}

fn _to_string(arr: &ArrayView2<Symbol>) -> String {
    arr.axis_iter(Axis(0))
        .map(|line| line.iter().map(|c| format!("{:?}", c)).join(""))
        .join("\n")
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
    fn example_b() {
        assert_eq!(solve_b(EXAMPLE), 64);
    }

    #[test]
    fn parse() {
        let (i, patterns) = super::parse(EXAMPLE).unwrap();
        assert!(i.is_empty());
        assert_eq!(patterns.len(), 10);
    }

    #[test]
    fn one_cycle() {
        let (i, patterns) = super::parse(EXAMPLE).unwrap();
        assert!(i.is_empty());

        let mut arr = to_array(patterns);
        println!("Inital: \n{}\n", _to_string(&arr.view()));

        tilt_north(&mut arr.view_mut());
        println!("North Tilt: \n{}\n", _to_string(&arr.view()));

        tilt_north(&mut arr.view_mut().reversed_axes());
        println!("West Tilt: \n{}\n", _to_string(&arr.view()));

        tilt_north(&mut arr.slice_mut(s![..;-1 ,..;-1]));
        println!("South Tilt: \n{}\n", _to_string(&arr.view()));

        tilt_north(&mut arr.slice_mut(s![..;-1 ,..;-1]).reversed_axes());
        println!("East Tilt: \n{}\n", _to_string(&arr.view()));
    }

    #[test]
    fn many_cycle() {
        let (i, patterns) = super::parse(EXAMPLE).unwrap();
        assert!(i.is_empty());

        let mut arr = to_array(patterns);

        let first = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";
        tilt_cycle(&mut arr.view_mut());
        assert_eq!(&_to_string(&arr.view()), first);

        let second = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O";
        tilt_cycle(&mut arr.view_mut());
        assert_eq!(&_to_string(&arr.view()), second);

        let third = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";
        tilt_cycle(&mut arr.view_mut());
        assert_eq!(&_to_string(&arr.view()), third);
    }
}
