crate::solution!(12, solve_a, solve_b);

use nom::{
    branch::alt,
    character::complete::char,
    character::complete::{line_ending, space1, u16},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
};

pub fn solve_a(input: &str) -> u64 {
    let (_, springs) = parse(input).expect("Failed to parse input");

    arrangements_sum(springs.into_iter())
}

pub fn solve_b(input: &str) -> u64 {
    let (_, springs) = parse(input).expect("Failed to parse input");

    arrangements_sum(springs.into_iter().map(expand))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

type Broken = u16;
type Springs = (Vec<Condition>, Vec<Broken>);

fn parse_springs(input: &str) -> nom::IResult<&str, Springs> {
    let condition = alt((
        map(char('.'), |_| Condition::Operational),
        map(char('#'), |_| Condition::Damaged),
        map(char('?'), |_| Condition::Unknown),
    ));

    separated_pair(many1(condition), space1, separated_list1(char(','), u16))(input)
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Springs>> {
    separated_list1(line_ending, parse_springs)(input)
}

fn expand(springs: Springs) -> Springs {
    let (conditions, broken) = springs;

    // Expand Vector to fit all possible combinations
    let ex_conditions_count = conditions.len() * 5 + 4;
    let ex_broken_count = broken.len() * 5;
    let mut ex_conditions = Vec::with_capacity(ex_conditions_count);
    let mut ex_broken = Vec::with_capacity(ex_broken_count);

    ex_conditions.extend(
        conditions
            .into_iter()
            .chain(std::iter::once(Condition::Unknown))
            .cycle()
            .take(ex_conditions_count),
    );
    ex_broken.extend(broken.into_iter().cycle().take(ex_broken_count));

    (ex_conditions, ex_broken)
}

type DpMap = Vec<Option<u64>>;
unsafe fn dp_arrangements<'a>(
    conditions: &'a [Condition],
    broken: &'a [Broken],
    dp: &mut DpMap,
    index_1: usize,
    index_2: usize,
    index_1_max: usize,
) -> u64 {
    // If the there not supposed to be any more broken springs, and all conditions are operational or unknown
    if broken.is_empty() {
        // We have a valid arrangement
        if conditions
            .iter()
            .all(|&c| matches!(c, Condition::Operational | Condition::Unknown))
        {
            return 1;
        } else {
            return 0;
        }
    // If there are supposed the same amount of broken springs as conditions, and all conditions are damaged or unknown
    } else if broken[0] == conditions.len() as u16 {
        if conditions
            .iter()
            .all(|&c| matches!(c, Condition::Damaged | Condition::Unknown))
        {
            return 1;
        } else {
            return 0;
        }
    }

    // If to many springs are broken, we dont have any valid arrangements
    if broken.iter().map(|i| *i as usize).sum::<usize>() + broken.len().saturating_sub(1)
        > conditions.len()
    {
        return 0;
    }

    // If there are no more inputs, we dont have any more arangements
    if conditions.is_empty() {
        return 0;
    }

    // If we have already calculated the number of arrangements for this state, return it
    debug_assert!(map_to_1d(index_1, index_2, index_1_max) < dp.len());
    if let Some(arrangements) = dp.get_unchecked(map_to_1d(index_1, index_2, index_1_max)) {
        return *arrangements;
    }

    let mut valid = 0;

    // Try to skip spring at current location, assuming that its operational
    if matches!(conditions[0], Condition::Operational | Condition::Unknown) {
        valid += dp_arrangements(
            &conditions[1..],
            broken,
            dp,
            index_1 + 1,
            index_2,
            index_1_max,
        );
    }

    // Try to fit springs at current location, knowing that the next spring after the group is operational
    if (broken[0] as usize) < conditions.len()
        && conditions
            .iter()
            .take(broken[0] as usize)
            .all(|&c| matches!(c, Condition::Damaged | Condition::Unknown))
        && matches!(
            conditions[broken[0] as usize],
            Condition::Operational | Condition::Unknown
        )
    {
        valid += dp_arrangements(
            &conditions[broken[0] as usize + 1..],
            &broken[1..],
            dp,
            index_1 + broken[0] as usize + 1,
            index_2 + 1,
            index_1_max,
        );
    }

    // Insert the number of arrangements for this state into the dp map
    dp.get_unchecked_mut(map_to_1d(index_1, index_2, index_1_max))
        .replace(valid);
    valid
}

fn map_to_1d(conditions: usize, broken: usize, conditions_max: usize) -> usize {
    broken * conditions_max + conditions
}

fn arrangements(
    conditions: &[Condition],
    broken: &[Broken],
    dp_buffer: &mut DpMap,
    max_conditions: usize,
) -> u64 {
    // Make sure that the dp buffer is large enough
    debug_assert!(dp_buffer.len() >= conditions.len() * broken.len());

    unsafe { dp_arrangements(conditions, broken, dp_buffer, 0, 0, max_conditions) }
}

fn arrangements_sum<I>(springs: I) -> u64
where
    I: Iterator<Item = Springs> + Clone,
{
    let max_broken = springs
        .clone()
        .map(|(_, broken)| broken.len())
        .max()
        .unwrap_or(0);
    let max_conditions = springs
        .clone()
        .map(|(conditions, _)| conditions.len())
        .max()
        .unwrap_or(0);

    let mut dp_buffer = vec![None; max_conditions * max_broken];

    springs
        .map(|(conditions, broken)| {
            // Clear the dp buffer
            dp_buffer.fill(None);
            arrangements(&conditions, &broken, &mut dp_buffer, max_conditions)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn example_a() {
        assert_eq!(solve_a(EXAMPLE), 21);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(EXAMPLE), 525152);
    }

    fn arrangements(conditions: &[Condition], broken: &[Broken]) -> u64 {
        // Make sure that the dp buffer is large enough
        let mut dp_buffer = vec![None; broken.len() * conditions.len()];

        unsafe { dp_arrangements(conditions, broken, &mut dp_buffer, 0, 0, conditions.len()) }
    }

    #[test]
    fn arrage_1() {
        let (_, (springs, broken)) = parse_springs("???.### 1,1,3").unwrap();
        assert_eq!(arrangements(&springs, &broken), 1);
    }

    #[test]
    fn arrage_2() {
        let (_, (springs, broken)) = parse_springs(".??..??...?##. 1,1,3").unwrap();
        assert_eq!(arrangements(&springs, &broken), 4);
    }

    #[test]
    fn arrage_3() {
        let (_, (springs, broken)) = parse_springs("?#?#?#?#?#?#?#? 1,3,1,6").unwrap();
        assert_eq!(arrangements(&springs, &broken), 1);
    }

    #[test]
    fn arrage_4() {
        let (_, (springs, broken)) = parse_springs("????.#...#... 4,1,1").unwrap();
        assert_eq!(arrangements(&springs, &broken), 1);
    }

    #[test]
    fn arrage_5() {
        let (_, (springs, broken)) = parse_springs("????.######..#####. 1,6,5").unwrap();
        assert_eq!(arrangements(&springs, &broken), 4);
    }

    #[test]
    fn arrage_6() {
        let (_, (springs, broken)) = parse_springs("?###???????? 3,2,1").unwrap();
        assert_eq!(arrangements(&springs, &broken), 10);
    }

    #[test]
    fn expand() {
        let (_, (springs, broken)) = parse_springs(".# 1").unwrap();
        let (springs, broken) = super::expand((springs, broken));
        assert_eq!(springs.len(), 14);
        assert_eq!(broken.len(), 5);

        let (_, (springs, broken)) = parse_springs("???.### 1,1,3").unwrap();
        let (springs, broken) = super::expand((springs, broken));
        assert_eq!(springs.len(), 39);
        assert_eq!(broken.len(), 15);
    }
}
