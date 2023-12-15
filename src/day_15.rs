crate::solution!(15, solve_a);

pub fn solve_a(input: &str) -> u64 {
    let (rest, patterns) = parse(input).unwrap();
    debug_assert!(rest.is_empty());

    patterns.iter().map(|&pattern| hash(pattern)).sum()
}

fn hash(input: &str) -> u64 {
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as u64) * 17) % 256)
}

fn parse(input: &str) -> nom::IResult<&str, Vec<&str>> {
    use nom::{
        bytes::complete::{is_not, tag},
        character::complete::multispace0,
        multi::separated_list1,
        sequence::delimited,
    };

    delimited(
        multispace0,
        separated_list1(tag(","), is_not(",\n\r")),
        multispace0,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn example_a() {
        assert_eq!(solve_a(EXAMPLE), 1320);
    }

    #[test]
    fn hash() {
        let (rest, patterns) = parse(EXAMPLE).unwrap();
        assert_eq!(rest, "");
        assert_eq!(patterns.len(), 11);

        assert_eq!(super::hash(patterns[0]), 30);
        assert_eq!(super::hash(patterns[1]), 253);
        assert_eq!(super::hash(patterns[2]), 97);
        assert_eq!(super::hash(patterns[3]), 47);
        assert_eq!(super::hash(patterns[4]), 14);
        assert_eq!(super::hash(patterns[5]), 180);
        assert_eq!(super::hash(patterns[6]), 9);
        assert_eq!(super::hash(patterns[7]), 197);
        assert_eq!(super::hash(patterns[8]), 48);
        assert_eq!(super::hash(patterns[9]), 214);
        assert_eq!(super::hash(patterns[10]), 231);
    }
}
