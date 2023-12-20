use std::{collections::HashMap, u16};

const ACCEPTED: &str = "A";
const REJECTED: &str = "R";

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> u64 {
    let (_, workflows) = parse(input).expect("Failed to parse");
    workflows.possibilities("in", InputRange::default())
}

#[derive(Debug, PartialEq, Clone)]
struct InputRange {
    inputs: HashMap<char, std::ops::Range<u16>>,
}
impl InputRange {
    fn default() -> Self {
        Self {
            inputs: HashMap::from([
                ('x', 1..4001),
                ('m', 1..4001),
                ('a', 1..4001),
                ('s', 1..4001),
            ]),
        }
    }
    fn create_lt_branches(&self, key: &char, value: &u32) -> (Self, Self) {
        let mut current_range = self.clone();
        let mut next_range = self.clone();
        let current_key_range = current_range.inputs.get_mut(&key).unwrap();
        *current_key_range = current_key_range.start..current_key_range.end.min(*value as u16);
        let next_key_range = next_range.inputs.get_mut(&key).unwrap();
        *next_key_range = (*value as u16)..next_key_range.end;
        (current_range, next_range)
    }
    fn create_gt_branches(&self, key: &char, value: &u32) -> (Self, Self) {
        let mut current_range = self.clone();
        let mut next_range = self.clone();
        let current_key_range = current_range.inputs.get_mut(&key).unwrap();
        *current_key_range = (*value as u16 + 1)..current_key_range.end;
        let next_key_range = next_range.inputs.get_mut(&key).unwrap();
        *next_key_range = next_key_range.start..(*value as u16 + 1);
        (current_range, next_range)
    }
    fn possibility_count(&self) -> u64 {
        self.inputs
            .values()
            .map(|v| v.len() as u64)
            .product::<u64>()
    }
}

#[derive(Debug, PartialEq)]
struct Workflows {
    steps: HashMap<String, Vec<Rule>>,
}
impl Workflows {
    fn new(steps: Vec<(&str, Vec<Rule>)>) -> Self {
        Self {
            steps: steps
                .into_iter()
                .fold(HashMap::new(), |mut acc, (key, rule)| {
                    acc.insert(key.to_string(), rule);
                    acc
                }),
        }
    }

    fn possibilities(&self, key: &str, mut input_range: InputRange) -> u64 {
        let mut sum = 0_u64;
        let rules = self.steps.get(key).unwrap();

        for rule in rules {
            match rule {
                Rule::Lt(key, value, new_destination) => {
                    let (current_range, next_range) = input_range.create_lt_branches(key, value);
                    input_range = next_range;
                    sum += match new_destination.as_str() {
                        ACCEPTED => current_range.possibility_count(),
                        REJECTED => 0,
                        _ => self.possibilities(new_destination, current_range),
                    }
                }
                Rule::Gt(key, value, new_destination) => {
                    let (current_range, next_range) = input_range.create_gt_branches(key, value);
                    input_range = next_range;
                    sum += match new_destination.as_str() {
                        ACCEPTED => current_range.possibility_count(),
                        REJECTED => 0,
                        _ => self.possibilities(new_destination, current_range),
                    }
                }
                Rule::Default(new_destination) => {
                    let current_range = input_range.clone();
                    sum += match new_destination.as_str() {
                        ACCEPTED => current_range.possibility_count(),
                        REJECTED => 0,
                        _ => self.possibilities(new_destination, current_range),
                    }
                }
            }
        }
        sum
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Rule {
    Lt(char, u32, String),
    Gt(char, u32, String),
    Default(String),
}

fn parse_condition(input: &str) -> nom::IResult<&str, Rule> {
    let (input, field) = nom::character::complete::one_of("xmas")(input)?;
    let (input, conditional) = nom::character::complete::one_of("<>")(input)?;
    let (input, compared) = nom::character::complete::digit1(input)?;
    let (input, destination) = nom::sequence::preceded(
        nom::character::complete::char(':'),
        nom::character::complete::alpha1,
    )(input)?;

    Ok((
        input,
        match conditional {
            '<' => Rule::Lt(
                field,
                compared.parse::<u32>().expect("cannot parse value"),
                destination.to_string(),
            ),
            '>' => Rule::Gt(
                field,
                compared.parse::<u32>().expect("cannot parse value"),
                destination.to_string(),
            ),
            _ => Rule::Default(destination.to_string()),
        },
    ))
}
fn parse_rule_list(input: &str) -> nom::IResult<&str, Vec<Rule>> {
    let (input, rules) =
        nom::multi::separated_list1(nom::character::complete::char(','), parse_condition)(input)?;
    let (input, default) = nom::sequence::preceded(
        nom::character::complete::char(','),
        nom::character::complete::alpha1,
    )(input)?;
    let rules = rules
        .into_iter()
        .chain(std::iter::once(Rule::Default(default.to_string())))
        .collect::<Vec<Rule>>();
    Ok((input, rules))
}
fn parse_workflow(input: &str) -> nom::IResult<&str, (&str, Vec<Rule>)> {
    let (input, name) = nom::bytes::complete::take_until("{")(input)?;
    let (input, rules) = nom::sequence::delimited(
        nom::character::complete::char('{'),
        parse_rule_list,
        nom::character::complete::char('}'),
    )(input)?;
    Ok((input, (name, rules)))
}
fn parse(input: &str) -> nom::IResult<&str, Workflows> {
    let (input, workflows) = nom::multi::many1(nom::sequence::terminated(
        parse_workflow,
        nom::branch::alt((nom::character::complete::line_ending, nom::combinator::eof)),
    ))(input)?;
    Ok((input, Workflows::new(workflows)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let input = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}";
        let (_, workflows) = parse(input).expect("Failed to parse");
        let expected_workflows = Workflows::new(vec![
            (
                "px",
                vec![
                    Rule::Lt('a', 2006, "qkq".to_string()),
                    Rule::Gt('m', 2090, "A".to_string()),
                    Rule::Default("rfg".to_string()),
                ],
            ),
            (
                "pv",
                vec![
                    Rule::Gt('a', 1716, "R".to_string()),
                    Rule::Default("A".to_string()),
                ],
            ),
        ]);
        assert_eq!(workflows, expected_workflows);
    }
    #[test]
    fn test_process() {
        assert_eq!(
            process_input(
                "\
px{a<2006:qkq,m>2090:A,rfg}
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
{x=2127,m=1623,a=2188,s=1013}"
            ),
            167409079868000
        );
    }

    #[test]
    fn test_create_lt_branches() {
        let input_range = InputRange::default();
        let (current_range, next_range) = input_range.create_lt_branches(&'x', &2000);
        assert_eq!(current_range.inputs.get(&'x'), Some(&(1..2000)));
        assert_eq!(next_range.inputs.get(&'x'), Some(&(2000..4001)));
    }

    #[test]
    fn test_create_gt_branches() {
        let input_range = InputRange::default();
        let (current_range, next_range) = input_range.create_gt_branches(&'x', &1999);
        assert_eq!(current_range.inputs.get(&'x'), Some(&(2000..4001)));
        assert_eq!(next_range.inputs.get(&'x'), Some(&(1..2000)));
    }
}

