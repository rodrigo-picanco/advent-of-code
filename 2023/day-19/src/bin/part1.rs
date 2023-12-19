use nom::{sequence::separated_pair, IResult};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> i32 {
    let (_, (workflows, parts)) = parse(input).expect("Failed to parse");

    let start = workflows.iter().find(|workflow| workflow.name == "in").unwrap();
    let mut accepted = vec![];
    
    for part in parts {
        let mut queue = vec![start.clone()];
        let part = part.clone();

        while let Some(workflow) = queue.pop() {
            let result = workflow.process(part);
            match result {
                Result::Destination(destination) => {
                    match destination.as_str() {
                        "A" => {
                            accepted.push(part);
                            break;
                        },
                        "R" => {
                            continue;
                        },
                        _ => {

                    let next = workflows.iter().find(|workflow| workflow.name == destination).unwrap();
                    queue.push(next);
                        }
                    }
                    
                }
                Result::Accepted => {
                    accepted.push(part);
                    break;
                }
                Result::Rejected => {
                    continue;
                }
            }
        }
    };

    accepted.iter().fold(0_i32, |acc, part| acc + part.x + part.m + part.a + part.s)

}

fn compare(value: i32, condition: Condition) -> bool {
    match condition.conditional.as_str() {
        "<" => value < condition.compared,
        ">" => value > condition.compared,
        _ => unreachable!(),
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}
impl Part {
    fn get(&self, field: char) -> i32 {
        match field {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!(),
        }
    }
}


#[derive(Debug, PartialEq)]
struct Workflow {
    rules: Vec<Rule>,
    name: String,
}
impl Workflow {
    fn new(rules: Vec<Rule>, name: &str) -> Self {
        Self {
            rules,
            name: name.to_string(),
        }
    }
    fn process(&self, part: Part) -> Result{
        let mut result = Result::Rejected;
        let rules = self.rules.clone();
        for rule in rules {
            match rule {
                Rule::Condition(condition) => {
                        if compare(part.get(condition.field), condition.clone()) {
                            result = Result::Destination(condition.destination);
                            break;
                        }
                }
                Rule::Result(r) => {
                    result = r;
                    break;
                }
            }


        }
        result
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Result {
    Accepted,
    Rejected,
    Destination(String),
}

#[derive(Debug, PartialEq, Clone)]
struct Condition {
    field: char,
    conditional: String,
    compared: i32,
    destination: String,
}
impl Condition {
    fn new(field: char, conditional: &str, compared: i32, destination: &str) -> Self {
        Self {
            field,
            conditional: conditional.to_string(),
            destination: destination.to_string(),
            compared,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Rule {
    Condition(Condition),
    Result(Result),
}

fn parse_condition(input: &str) -> IResult<&str, Rule> {
    let (input, field) = nom::character::complete::one_of("xmas")(input)?;
    let (input, conditional) = nom::multi::many1(nom::character::complete::one_of("<>"))(input)?;
    let (input, compared) = nom::character::complete::digit1(input)?;
    let (input, destination) = nom::sequence::preceded(
        nom::character::complete::char(':'),
        nom::character::complete::alpha1,
    )(input)?;
    Ok((
        input,
        Rule::Condition(Condition::new(
            field,
            &conditional.iter().collect::<String>(),
            i32::from_str_radix(compared, 10).expect("cannot parse compared"),
            destination,
        )),
    ))
}
fn parse_result(input: &str) -> IResult<&str, Rule> {
    let (input, result) = nom::branch::alt((
        nom::bytes::complete::tag("A"),
        nom::bytes::complete::tag("R"),
        nom::character::complete::alpha1,
    ))(input)?;
    Ok((
        input,
        Rule::Result(match result {
            "A" => Result::Accepted,
            "R" => Result::Rejected,
            _ => Result::Destination(result.to_string()),
        }),
    ))
}
fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, rule) = nom::branch::alt((parse_condition, parse_result))(input)?;
    Ok((input, rule))
}
fn parse_rule_list(input: &str) -> IResult<&str, Vec<Rule>> {
    let (input, rules) =
        nom::multi::separated_list1(nom::character::complete::char(','), parse_rule)(input)?;
    Ok((input, rules))
}
fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = nom::bytes::complete::take_until("{")(input)?;
    let (input, rules) = nom::sequence::delimited(
        nom::character::complete::char('{'),
        parse_rule_list,
        nom::character::complete::char('}'),
    )(input)?;

    Ok((input, Workflow::new(rules, name)))
}
fn parse_workflow_list(input: &str) -> IResult<&str, Vec<Workflow>> {
    let (input, workflows) = nom::multi::many1(nom::sequence::terminated(
        parse_workflow,
        nom::branch::alt((nom::character::complete::line_ending, nom::combinator::eof)),
    ))(input)?;
    Ok((input, workflows))
}
fn parse_part_field(input: &str) -> IResult<&str, (char, i32)> {
    let (input, field) = nom::character::complete::one_of("xmas")(input)?;
    let (input, _) = nom::character::complete::char('=')(input)?;
    let (input, value) = nom::character::complete::digit1(input)?;
    Ok((
        input,
        (
            field,
            i32::from_str_radix(value, 10).expect("cannot parse value"),
        ),
    ))
}
fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, output) = nom::sequence::delimited(
            nom::character::complete::char('{'),
            nom::multi::separated_list1(nom::character::complete::char(','), parse_part_field),
            nom::character::complete::char('}')
        )(input)?;
    Ok((
            input,
            Part {
                x: output[0].1,
                m: output[1].1,
                a: output[2].1,
                s: output[3].1
            }
            ))

}

fn parse_part_list(input: &str) -> IResult<&str, Vec<Part>> {
    let (input, output) = nom::multi::many1(nom::sequence::terminated(
            parse_part,
        nom::branch::alt((nom::character::complete::line_ending, nom::combinator::eof)),
    ))(input)?;

    Ok((input, output))
}
fn parse(input: &str) -> IResult<&str, (Vec<Workflow>, Vec<Part>)> {
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
    use rstest::rstest;

    #[rstest]
    #[case("x=787", ('x', 787))]
    #[case("m=2655", ('m', 2655))]
    #[case("a=1222", ('a', 1222))]
    #[case("s=2876", ('s', 2876))]
    fn test_parse_part_field(#[case] input: &str, #[case] expected: (char, i32)) {
        let (_, output) = parse_part_field(input).expect("Failed to parse part field");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_part_list() {
        let input = "\
{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}";
        let (_, output) = parse_part_list(input).expect("Failed to parse part list");
        assert_eq!(
            output,
            vec![
                Part { x: 787, m: 2655, a: 1222, s: 2876 },
                Part { x: 1679, m: 44, a: 2067, s: 496},
            ] as Vec<Part>
        );
    }

    #[rstest]
    #[case("a<2006:qkq", Rule::Condition(Condition::new('a', "<", 2006, "qkq")))]
    #[case("m>2090:A", Rule::Condition(Condition::new('m', ">", 2090, "A")))]
    #[case("A", Rule::Result(Result::Accepted))]
    #[case("R", Rule::Result(Result::Rejected))]
    fn test_parse_rule(#[case] input: &str, #[case] expected: Rule) {
        let (_, rule) = parse_rule(input).expect("Failed to parse rule");
        assert_eq!(rule, expected);
    }

    #[rstest]
    #[case((200, Condition::new('a', "<", 2000, "qkq")), true)]
    #[case((200, Condition::new('a', ">", 10, "qkq")), true)]
    #[case((200, Condition::new('a', ">", 200, "qkq")), false)]
    fn test_compare(#[case] input: (i32, Condition), #[case] expected: bool) {
        let (value, condition) = input;
        assert_eq!(compare(value, condition), expected);
    }

    #[rstest]
    #[case("a<2006:qkq,m>2090:A,rfg", vec![
        Rule::Condition(Condition::new('a', "<", 2006, "qkq")),
        Rule::Condition(Condition::new('m', ">", 2090, "A")),
        Rule::Result(Result::Destination("rfg".to_string())),
    ])]
    #[case("a>1716:R,A", vec![
        Rule::Condition(Condition::new('a', ">", 1716, "R")),
        Rule::Result(Result::Accepted),
    ])]
    fn test_parse_rule_list(#[case] input: &str, #[case] expected: Vec<Rule>) {
        let (_, rules) = parse_rule_list(input).expect("Failed to parse rule list");
        assert_eq!(rules, expected);
    }

    #[rstest]
    #[case("px{a<2006:qkq,m>2090:A,rfg}", 
           Workflow::new(
            vec![
                Rule::Condition(Condition::new('a', "<", 2006, "qkq")),
                Rule::Condition(Condition::new('m', ">", 2090, "A")),
                Rule::Result(Result::Destination("rfg".to_string())),
            ], "px"))]
    #[case("pv{a>1716:R,A}", Workflow::new(
            vec![
                Rule::Condition(Condition::new('a', ">", 1716, "R")),
                Rule::Result(Result::Accepted),
            ], "pv"))]
    fn test_parse_workflow(#[case] input: &str, #[case] expected: Workflow) {
        let (_, workflow) = parse_workflow(input).expect("Failed to parse workflow");
        assert_eq!(workflow, expected);
    }

    #[test]
    fn test_parse_workflow_list() {
        let input = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}";
        let (_, workflows) = parse_workflow_list(input).expect("Failed to parse workflow list");
        assert_eq!(
            workflows,
            vec![
                Workflow::new(
                    vec![
                        Rule::Condition(Condition::new('a', "<", 2006, "qkq")),
                        Rule::Condition(Condition::new('m', ">", 2090, "A")),
                        Rule::Result(Result::Destination("rfg".to_string())),
                    ],
                    "px"
                ),
                Workflow::new(
                    vec![
                        Rule::Condition(Condition::new('a', ">", 1716, "R")),
                        Rule::Result(Result::Accepted),
                    ],
                    "pv"
                ),
            ]
        );
    }

    #[test]
    fn test_parse() {
        let input = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}";
        let (_, (workflows, parts)) = parse(input).expect("Failed to parse");
        let expected_workflows = vec![
            Workflow::new(
                vec![
                    Rule::Condition(Condition::new('a', "<", 2006, "qkq")),
                    Rule::Condition(Condition::new('m', ">", 2090, "A")),
                    Rule::Result(Result::Destination("rfg".to_string())),
                ],
                "px",
            ),
            Workflow::new(
                vec![
                    Rule::Condition(Condition::new('a', ">", 1716, "R")),
                    Rule::Result(Result::Accepted),
                ],
                "pv",
            ),
        ];
        let expected_parts = vec![
            Part { x: 787, m: 2655, a: 1222, s: 2876 },
            Part { x: 1679, m: 44, a: 2067, s: 496 },
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
