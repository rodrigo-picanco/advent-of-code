use std::collections::{BTreeMap, HashMap};

use nom::{sequence::separated_pair, IResult};

const ACCEPTED: &str = "A";
const REJECTED: &str = "R";
const ENTRANCE: &str = "in";

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> u32 {
    let (_, (workflows, parts)) = parse(input).expect("Failed to parse");
    parts
        .into_iter()
        .filter(|part| workflows.process(part))
        .map(|part| part.value())
        .sum()
}

#[derive(Debug, PartialEq)]
struct Part {
    values: HashMap<char, u32>,
}
impl Part {
    fn new((x, m, a, s): (u32, u32, u32, u32)) -> Self {
        let mut hash = HashMap::new();
        hash.insert('x', x);
        hash.insert('m', m);
        hash.insert('a', a);
        hash.insert('s', s);
        Self { values: hash }
    }
    fn value(&self) -> u32 {
        self.values.values().sum()
    }
}

#[derive(Debug, PartialEq)]
struct Workflows<'a> {
    steps: HashMap<&'a str, Vec<Rule<'a>>>
}
impl<'a> Workflows<'a> {
    fn new(steps: Vec<(&'a str, Vec<Rule<'a>>)>) -> Self {
        Self { 
            steps: steps.into_iter().fold(HashMap::new(), |mut acc, (key, rule)| {
                acc.insert(key, rule);
                acc
            })
        }
    }

    fn is_decision(destination: &str) -> bool {
        destination == ACCEPTED || destination == REJECTED
    }

    fn process(&self, part: &Part) -> bool {
        let mut destination = ENTRANCE;

        while !Self::is_decision(&destination) {
            let rules = self.steps.get(destination).unwrap();
            for rule in rules.iter() {
                match rule {
                    Rule::Lt(key, value, new_destination) => {
                        if part.values.get(&key).unwrap() < &value {
                            destination = new_destination;
                            break;
                        }
                    }
                    Rule::Gt(key, value, new_destination) => {
                        if part.values.get(&key).unwrap() > &value {
                            destination = new_destination;
                            break;
                        }
                    }
                    Rule::Default(new_destination) => {
                        destination = new_destination;
                    }
                }
            }
        }

        destination == ACCEPTED
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Rule<'a> {
    Lt(char, u32, &'a str),
    Gt(char, u32, &'a str),
    Default(&'a str),
}

fn parse_condition(input: &str) -> IResult<&str, Rule> {
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
                destination,
            ),
            '>' => Rule::Gt(
                field,
                compared.parse::<u32>().expect("cannot parse value"),
                destination,
            ),
            _ => Rule::Default(destination),
        },
    ))
}
fn parse_rule_list(input: &str) -> IResult<&str, Vec<Rule>> {
    let (input, rules) =
        nom::multi::separated_list1(nom::character::complete::char(','), parse_condition)(input)?;
    let (input, default) = nom::sequence::preceded(
        nom::character::complete::char(','),
        nom::character::complete::alpha1,
    )(input)?;
    let rules = rules
        .into_iter()
        .chain(std::iter::once(Rule::Default(default)))
        .collect::<Vec<Rule>>();

    Ok((input, rules))
}
fn parse_workflow(input: &str) -> IResult<&str, (&str, Vec<Rule>)> {
    let (input, name) = nom::bytes::complete::take_until("{")(input)?;
    let (input, rules) = nom::sequence::delimited(
        nom::character::complete::char('{'),
        parse_rule_list,
        nom::character::complete::char('}'),
    )(input)?;
    Ok((input, (name, rules)))
}
fn parse_workflow_list(input: &str) -> IResult<&str, Workflows> {
    let (input, workflows) = nom::multi::many1(nom::sequence::terminated(
        parse_workflow,
        nom::branch::alt((nom::character::complete::line_ending, nom::combinator::eof)),
    ))(input)?;
    Ok((input, Workflows::new(workflows)))
}
fn parse_part_field(input: &str) -> IResult<&str, u32> {
    let (input, _) = nom::character::complete::one_of("xmas")(input)?;
    let (input, _) = nom::character::complete::char('=')(input)?;
    let (input, value) = nom::character::complete::digit1(input)?;
    Ok((input, value.parse::<u32>().expect("cannot parse value")))
}
fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, output) = nom::sequence::delimited(
        nom::character::complete::char('{'),
        nom::multi::separated_list1(nom::character::complete::char(','), parse_part_field),
        nom::character::complete::char('}'),
    )(input)?;
    Ok((
        input,
        Part::new((output[0], output[1], output[2], output[3])),
    ))
}
fn parse_part_list(input: &str) -> IResult<&str, Vec<Part>> {
    let (input, output) = nom::multi::many1(nom::sequence::terminated(
        parse_part,
        nom::branch::alt((nom::character::complete::line_ending, nom::combinator::eof)),
    ))(input)?;

    Ok((input, output))
}
fn parse(input: &str) -> IResult<&str, (Workflows, Vec<Part>)> {
    let (input, (workflows, parts)) = separated_pair(
        parse_workflow_list,
        nom::character::complete::line_ending,
        parse_part_list,
    )(input)?;
    Ok((input, (workflows, parts)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}";
        let (_, (workflows, parts)) = parse(input).expect("Failed to parse");
        let expected_workflows = Workflows::new(vec![
            (
                "px",
                vec![
                    Rule::Lt('a', 2006, "qkq"),
                    Rule::Gt('m', 2090, &"A"),
                    Rule::Default("rfg"),
                ],
            ),
            ("pv", vec![Rule::Gt('a', 1716, "R"), Rule::Default("A")]) ,
        ]);

        let expected_parts = vec![
            Part::new((787, 2655, 1222, 2876)),
            Part::new((1679, 44, 2067, 496)),
        ];
        assert_eq!(workflows, expected_workflows);
        assert_eq!(parts, expected_parts);
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
            19114
        );
    }
}
