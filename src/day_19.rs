use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        self,
        complete::{alpha1, line_ending, newline},
        streaming::multispace1,
    },
    combinator::opt,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};
use rayon::prelude::*;

crate::solution!(19, solve_a, solve_b);

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

type Workflow = Vec<Box<dyn Fn(&Part) -> State>>;
type Workflows = HashMap<String, Workflow>;

#[derive(Debug, Clone)]
enum State {
    Accept,
    Reject,
    Continue { workflow: String },
    NotMatched,
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
    0
}

fn part_accepted(workflows: &Workflows, part: &Part) -> bool {
    let mut rules = workflows["in"].iter();

    loop {
        let state = rules.next().expect("No next rule found")(part);
        match state {
            State::Accept => return true,
            State::Reject => return false,
            State::Continue { ref workflow } => rules = workflows[workflow].iter(),
            State::NotMatched => { /* Do Nothing and match next */ }
        };
    }
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
        alt((
            tag("x").map(|_| Box::new(|part: &Part| part.x) as Box<dyn Fn(&Part) -> u32>),
            tag("m").map(|_| Box::new(|part: &Part| part.m) as Box<dyn Fn(&Part) -> u32>),
            tag("a").map(|_| Box::new(|part: &Part| part.a) as Box<dyn Fn(&Part) -> u32>),
            tag("s").map(|_| Box::new(|part: &Part| part.s) as Box<dyn Fn(&Part) -> u32>),
        )),
        alt((
            tag("<").map(|_| Box::new(|a, b| a < b) as Box<dyn Fn(u32, u32) -> bool>),
            tag(">").map(|_| Box::new(|a, b| a > b) as Box<dyn Fn(u32, u32) -> bool>),
            tag("=").map(|_| Box::new(|a, b| a == b) as Box<dyn Fn(u32, u32) -> bool>),
        )),
        character::complete::u32,
    ))
    .map(|(var, comparator, number)| {
        Box::new(move |part: &Part| comparator(var(part), number)) as Box<dyn Fn(&Part) -> bool>
    });

    let rule = pair(
        opt(terminated(condition, tag(":"))),
        alt((
            tag("A").map(|_| State::Accept),
            tag("R").map(|_| State::Reject),
            alpha1.map(|name: &str| State::Continue {
                workflow: name.to_owned(),
            }),
        )),
    )
    .map(|(condition, result)| {
        if let Some(condition) = condition {
            Box::new(move |part: &Part| {
                if condition(part) {
                    result.clone()
                } else {
                    State::NotMatched
                }
            }) as Box<dyn Fn(&Part) -> State>
        } else {
            Box::new(move |_: &Part| result.clone()) as Box<dyn Fn(&Part) -> State>
        }
    });

    pair(
        alpha1,
        delimited(tag("{"), separated_list1(tag(","), rule), tag("}")),
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
