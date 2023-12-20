use std::{collections::HashMap, ops::RangeInclusive};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        self,
        complete::{alpha1, line_ending, newline, one_of},
        streaming::multispace1,
    },
    combinator::opt,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

crate::solution!(19, solve_a, solve_b);

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug, Clone)]
struct PartRange {
    x: RangeInclusive<u32>,
    m: RangeInclusive<u32>,
    a: RangeInclusive<u32>,
    s: RangeInclusive<u32>,
}

impl PartRange {
    fn size(&self) -> u128 {
        let mut count = 1;
        for r in [&self.x, &self.m, &self.a, &self.s] {
            if !r.is_empty() {
                count *= (r.end() - r.start() + 1) as u128;
            }
        }
        count
    }

    fn get(&self, c: &char) -> &RangeInclusive<u32> {
        match c {
            'x' => &self.x,
            'm' => &self.m,
            'a' => &self.a,
            's' => &self.s,
            _ => panic!("Unknown variable"),
        }
    }

    fn set(&mut self, c: &char, r: RangeInclusive<u32>) {
        match c {
            'x' => self.x = r,
            'm' => self.m = r,
            'a' => self.a = r,
            's' => self.s = r,
            _ => panic!("Unknown variable"),
        }
    }
}

type Workflow = Vec<Rule>;
type Workflows = HashMap<String, Workflow>;

#[derive(Debug, Clone)]
enum State {
    Accept,
    Reject,
    Continue { workflow: String },
    NotMatched,
}

#[derive(Debug)]
enum Comparison {
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    result: State,
}

#[derive(Debug)]
struct Condition {
    var: char,
    comparator: Comparison,
    number: u32,
}

pub fn solve_a(input: &str) -> u32 {
    let (_, (workflows, parts)) = parse(input).unwrap();

    parts
        .iter()
        .filter(|part| part_accepted(&workflows, part))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

pub fn solve_b(input: &str) -> u128 {
    let (_, (workflows, _)) = parse(input).unwrap();

    count_accepted(
        &workflows,
        &"in".to_string(),
        PartRange {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        },
    )
}

fn part_accepted(workflows: &Workflows, part: &Part) -> bool {
    let mut rules = workflows["in"].iter();

    loop {
        let next_rule = rules.next().unwrap();
        let state = match next_rule.condition {
            Some(ref condition) => {
                let value = match condition.var {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    _ => panic!("Unknown variable"),
                };

                match condition.comparator {
                    Comparison::LessThan => {
                        if value < condition.number {
                            &next_rule.result
                        } else {
                            &State::NotMatched
                        }
                    }
                    Comparison::GreaterThan => {
                        if value > condition.number {
                            &next_rule.result
                        } else {
                            &State::NotMatched
                        }
                    }
                }
            }
            None => &next_rule.result,
        };

        match state {
            State::Accept => return true,
            State::Reject => return false,
            State::Continue { ref workflow } => rules = workflows[workflow].iter(),
            State::NotMatched => { /* Do Nothing and match next */ }
        };
    }
}

fn count_accepted(workflows: &Workflows, current_workflow: &String, mut range: PartRange) -> u128 {
    workflows[current_workflow]
        .iter()
        .map(|rule| match &rule.condition {
            None => match rule.result {
                State::Accept => range.size(),
                State::Reject => 0,
                State::Continue { ref workflow } => {
                    count_accepted(workflows, workflow, range.clone())
                }
                State::NotMatched => unreachable!(),
            },
            Some(condition) => {
                let (matches, rest) = match condition.comparator {
                    Comparison::LessThan => (
                        (*range.get(&condition.var).start()..=condition.number - 1),
                        (condition.number..=*range.get(&condition.var).end()),
                    ),
                    Comparison::GreaterThan => (
                        (condition.number + 1..=*range.get(&condition.var).end()),
                        (*range.get(&condition.var).start()..=condition.number),
                    ),
                };

                let mut sum = 0;
                if matches.start() <= matches.end() {
                    let mut new_range = range.clone();
                    new_range.set(&condition.var, matches);

                    match rule.result {
                        State::Accept => sum += new_range.size(),
                        State::Reject => {}
                        State::Continue { ref workflow } => {
                            sum += count_accepted(workflows, workflow, new_range)
                        }
                        State::NotMatched => unreachable!(),
                    };
                }
                range.set(&condition.var, rest);
                sum
            }
        })
        .sum()
}

fn parse(input: &str) -> IResult<&str, (Workflows, Vec<Part>)> {
    let part = tuple((
        delimited(tag("x="), character::complete::u32, tag(",")),
        delimited(tag("m="), character::complete::u32, tag(",")),
        delimited(tag("a="), character::complete::u32, tag(",")),
        preceded(tag("s="), character::complete::u32),
    ))
    .map(|(x, m, a, s)| Part { x, m, a, s });
    let parts = separated_list1(line_ending, delimited(tag("{"), part, tag("}")));

    separated_pair(
        separated_list1(newline, parse_workflow).map(|workflows: Vec<(&str, Workflow)>| {
            workflows
                .into_iter()
                .fold(Workflows::new(), |mut acc, (name, rules)| {
                    acc.insert(name.to_string(), rules);
                    acc
                })
        }),
        multispace1,
        parts,
    )(input)
}

fn parse_workflow(input: &str) -> IResult<&str, (&str, Workflow)> {
    let condition = tuple((
        one_of("xmas"),
        alt((
            tag("<").map(|_| Comparison::LessThan),
            tag(">").map(|_| Comparison::GreaterThan),
        )),
        character::complete::u32,
    ))
    .map(|(var, comparator, number)| Condition {
        var,
        comparator,
        number,
    });

    let result = pair(
        opt(terminated(condition, tag(":"))),
        alt((
            tag("A").map(|_| State::Accept),
            tag("R").map(|_| State::Reject),
            alpha1.map(|name: &str| State::Continue {
                workflow: name.to_owned(),
            }),
        )),
    )
    .map(|(condition, result)| Rule { condition, result });

    pair(
        alpha1,
        delimited(tag("{"), separated_list1(tag(","), result), tag("}")),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn example_a() {
        assert_eq!(solve_a(EXAMPLE), 19114);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(EXAMPLE), 167409079868000);
    }

    #[test]
    fn parse() {
        let (rest, (workflow, parts)) = super::parse(EXAMPLE).unwrap();
        assert_eq!(rest, "");
        assert_eq!(workflow.len(), 11);

        assert_eq!(parts.len(), 5);

        assert_eq!(workflow["px"].len(), 3);
    }
}
