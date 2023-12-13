use nom::{
    bytes::complete::is_a,
    character::complete::{line_ending, multispace1},
    multi::separated_list1,
    IResult, Parser,
};

crate::solution!(13, solve_a, solve_b);

pub fn solve_a(input: &str) -> u64 {
    let (_, patterns) = parse(input).unwrap();
    patterns
        .iter()
        .map(|p| {
            if let Some(i) = mirrors(&p.content).next() {
                i * 100
            } else if let Some(i) = mirrors(&transpose(&p.content)).next() {
                i
            } else {
                panic!("No mirror found")
            }
        })
        .sum::<usize>() as u64
}

pub fn solve_b(input: &str) -> u64 {
    let (_, patterns) = parse(input).unwrap();
    patterns
        .iter()
        .map(|p| {
            if let Some(i) = mirrors_smudged(&p.content).next() {
                i * 100
            } else if let Some(i) = mirrors_smudged(&transpose(&p.content)).next() {
                i
            } else {
                panic!("No smudged mirror found")
            }
        })
        .sum::<usize>() as u64
}

struct Pattern {
    content: Vec<String>,
}

fn parse(input: &str) -> IResult<&str, Vec<Pattern>> {
    let pattern =
        separated_list1(line_ending, is_a("#.").map(String::from)).map(|v| Pattern { content: v });
    separated_list1(multispace1, pattern)(input)
}

fn transpose(pattern: &[String]) -> Vec<String> {
    let mut iters: Vec<_> = pattern.iter().map(|n| n.chars()).collect();
    (0..pattern[0].len())
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<String>()
        })
        .collect()
}

fn mirrors(input: &Vec<String>) -> impl Iterator<Item = usize> + '_ {
    (1..input.len()).filter(|ix| {
        let left: Vec<_> = input.iter().take(*ix).rev().collect();
        let right = &input[*ix..];
        let size = left.len().min(right.len());

        (0..size).all(|pos| *left[pos] == right[pos])
    })
}

/// Returns the sorted list of (differences, index) pairs
fn mirrors_smudged(input: &Vec<String>) -> impl Iterator<Item = usize> + '_ {
    (1..input.len()).filter(|ix| {
        let left: Vec<_> = input.iter().take(*ix).rev().collect();
        let right = &input[*ix..];
        let size = left.len().min(right.len());

        // Exit early if there are more than one different rows
        let same_rows = (0..size).filter(|&pos| *left[pos] == right[pos]).count();
        if size - same_rows != 1 {
            return false;
        }

        debug_assert!(right[0].len() == left[0].len());

        // Check if the rest is the same
        let difference_chars: usize = (0..size)
            .map(|pos| {
                left[pos]
                    .chars()
                    .zip(right[pos].chars())
                    .filter(|(l, r)| l != r)
                    .count()
            })
            .sum();

        difference_chars == 1
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn example_a() {
        assert_eq!(solve_a(EXAMPLE), 405);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(EXAMPLE), 400);
    }

    #[test]
    fn mirror() {
        let (rest, patterns) = super::parse(EXAMPLE).unwrap();
        assert_eq!(rest, "");
        assert!(patterns.len() >= 2);

        let p1 = &patterns[1].content;

        assert_eq!(mirrors(p1).next(), Some(4));
    }

    #[test]
    fn transpose() {
        let (rest, patterns) = super::parse(EXAMPLE).unwrap();
        assert_eq!(rest, "");
        assert!(patterns.len() >= 2);

        let p1 = &patterns[0].content;
        let transposed = super::transpose(p1);

        assert_eq!(transposed.len(), 7);
        assert_eq!(mirrors(&transposed).next(), Some(5));
    }

    #[test]
    fn parse() {
        let (rest, patterns) = super::parse(EXAMPLE).unwrap();
        assert_eq!(rest, "");
        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns[0].content.len(), 7);
        assert_eq!(patterns[1].content.len(), 7);
    }
}
