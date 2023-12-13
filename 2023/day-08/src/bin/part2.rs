use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> usize {
    let (input, (moves, nodes)) = parse_input(input).expect("Failed to parse input");
    assert_eq!(input, "");

    let mut schema = Schema::new();
    for (name, left, right) in nodes {
        schema.insert(
            name.clone(),
            State::new(
                name.clone(),
                (left, right),
                name.clone().pop().unwrap() == 'A',
            ),
        );
    }

   let result =  schema
        .keys()
        .filter(|key| key.ends_with("A"))
        .cloned()
        .map(|key| -> usize {
            let mut schema = schema.clone();
            let initial = schema.get(&key).expect("No initial state").clone();
            schema.insert("initial".to_string(), initial);

            let mut machine = WalkMachine::new(schema);
            moves.iter().cycle().find_map(|x| -> Option<usize> {
                machine.walk(x.clone());
                if machine.state.name.ends_with("Z") {
                    Some(machine.steps)
                } else {
                    None
                }
            })
            .expect("No solution")
        }).collect::<Vec<usize>>();

   lcm(&result)
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}type Schema = BTreeMap<String, State>;

#[derive(Debug, PartialEq, Clone)]
struct State {
    name: String,
    transitions: (String, String),
    is_initial: bool,
}
impl State {
    fn new(name: String, transitions: (String, String), is_initial: bool) -> Self {
        Self {
            name,
            transitions,
            is_initial,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
struct WalkMachine {
    state: State,
    schema: Schema,
    steps: usize,
}
impl WalkMachine {
    fn new(schema: Schema) -> Self {
        let initial = schema.get("initial").expect("No initial state").clone();
        Self {
            state: schema.get(&initial.name).expect("No initial state").clone(),
            schema,
            steps: 0,
        }
    }

    // TODO: Could we avoid cloning the state?
    fn walk(&mut self, direction: Direction) {
        self.steps += 1;
        match direction {
            Direction::Left => {
                self.state = self
                    .schema
                    .get(&self.state.transitions.0)
                    .expect("No state. Cannot move left")
                    .clone();
            }
            Direction::Right => {
                self.state = self
                    .schema
                    .get(&self.state.transitions.1)
                    .expect("No state. Cannot move right")
                    .clone();
            }
        }
    }
}

fn parse_move(input: &str) -> nom::IResult<&str, Direction> {
    let (input, direction) = nom::branch::alt((
        nom::bytes::complete::tag("L"),
        nom::bytes::complete::tag("R"),
    ))(input)?;
    Ok((
        input,
        match direction {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        },
    ))
}
fn parse_name(input: &str) -> nom::IResult<&str, String> {
    let (input, name) = nom::character::complete::alphanumeric1(input)?;
    Ok((input, name.to_string()))
}
fn parse_transition(input: &str) -> nom::IResult<&str, (String, String)> {
    nom::sequence::delimited(
        nom::bytes::complete::tag("("),
        nom::sequence::separated_pair(parse_name, nom::bytes::complete::tag(", "), parse_name),
        nom::bytes::complete::tag(")"),
    )(input)
}
fn parse_node(input: &str) -> nom::IResult<&str, (String, String, String)> {
    let (input, (name, (left, right))) = nom::sequence::separated_pair(
        parse_name,
        nom::bytes::complete::tag(" = "),
        parse_transition,
    )(input)?;
    Ok((input, (name, left, right)))
}
fn parse_nodes(input: &str) -> nom::IResult<&str, Vec<(String, String, String)>> {
    let (input, output) = nom::multi::many1(nom::sequence::terminated(
        parse_node,
        nom::branch::alt((nom::character::complete::line_ending, nom::combinator::eof)),
    ))(input)?;
    Ok((input, output))
}
fn parse_moves(input: &str) -> nom::IResult<&str, Vec<Direction>> {
    let (input, output) = nom::sequence::terminated(
        nom::multi::many1(parse_move),
        nom::multi::many1(nom::character::complete::line_ending),
    )(input)?;
    Ok((input, output))
}
fn parse_input(input: &str) -> nom::IResult<&str, (Vec<Direction>, Vec<(String, String, String)>)> {
    let (input, moves) = parse_moves(input)?;
    let (input, nodes) = parse_nodes(input)?;
    Ok((input, (moves, nodes)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_steps() {
        let mut schema = Schema::new();
        schema.insert(
            "initial".to_string(),
            State::new("AAA".to_string(), ("".to_string(), "".to_string()), true),
        );
        schema.insert(
            "AAA".to_string(),
            State::new(
                "AAA".to_string(),
                ("BBB".to_string(), "CCC".to_string()),
                true,
            ),
        );
        schema.insert(
            "BBB".to_string(),
            State::new(
                "BBB".to_string(),
                ("AAA".to_string(), "CCC".to_string()),
                false,
            ),
        );

        schema.insert(
            "CCC".to_string(),
            State::new(
                "CCC".to_string(),
                ("CCC".to_string(), "CCC".to_string()),
                false,
            ),
        );
        let mut machine = WalkMachine::new(schema);
        assert_eq!(machine.steps, 0);

        machine.walk(Direction::Left);
        assert_eq!(machine.steps, 1);

        machine.walk(Direction::Right);
        assert_eq!(machine.steps, 2);
    }

    #[test]
    fn test_walk() {
        use Direction::*;
        let mut schema = Schema::new();
        schema.insert(
            "initial".to_string(),
            State::new("AAA".to_string(), ("".to_string(), "".to_string()), true),
        );
        schema.insert(
            "AAA".to_string(),
            State::new(
                "AAA".to_string(),
                ("BBB".to_string(), "BBB".to_string()),
                true,
            ),
        );
        schema.insert(
            "BBB".to_string(),
            State::new(
                "BBB".to_string(),
                ("AAA".to_string(), "CCC".to_string()),
                false,
            ),
        );
        schema.insert(
            "CCC".to_string(),
            State::new(
                "CCC".to_string(),
                ("CCC".to_string(), "CCC".to_string()),
                false,
            ),
        );
        let mut machine = WalkMachine::new(schema);

        struct Test {
            from: String,
            to: String,
            direction: Direction,
        }
        impl Test {
            fn new(from: String, to: String, direction: Direction) -> Self {
                Self {
                    from,
                    to,
                    direction,
                }
            }
        }

        let tests = vec![
            Test::new("AAA".to_string(), "BBB".to_string(), Left),
            Test::new("BBB".to_string(), "AAA".to_string(), Left),
            Test::new("AAA".to_string(), "BBB".to_string(), Right),
            Test::new("BBB".to_string(), "AAA".to_string(), Left),
            Test::new("AAA".to_string(), "BBB".to_string(), Left),
            Test::new("BBB".to_string(), "CCC".to_string(), Right),
        ];

        for test in tests {
            assert_eq!(machine.state.name, test.from);
            machine.walk(test.direction);
            assert_eq!(machine.state.name, test.to);
        }
    }

    #[test]
    fn test_parse_input() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let expected_nodes = [
            ("AAA", "BBB", "CCC"),
            ("BBB", "DDD", "EEE"),
            ("CCC", "ZZZ", "GGG"),
            ("DDD", "DDD", "DDD"),
            ("EEE", "EEE", "EEE"),
            ("GGG", "GGG", "GGG"),
            ("ZZZ", "ZZZ", "ZZZ"),
        ];
        let (_, (moves, nodes)) = parse_input(input).unwrap();

        assert_eq!(moves, vec![Direction::Right, Direction::Left]);

        for x in 0..nodes.len() {
            assert_eq!(
                nodes[x],
                (
                    expected_nodes[x].0.to_string(),
                    expected_nodes[x].1.to_string(),
                    expected_nodes[x].2.to_string()
                )
            )
        }
    }

    #[test]
    fn test_create_machine() {
        let mut schema = Schema::new();
        schema.insert(
            "initial".to_string(),
            State::new("AAA".to_string(), ("".to_string(), "".to_string()), true),
        );
        schema.insert(
            "AAA".to_string(),
            State::new(
                "AAA".to_string(),
                ("BBB".to_string(), "CCC".to_string()),
                true,
            ),
        );
        let machine = WalkMachine::new(schema);
        assert_eq!(machine.state.name, "AAA");
    }

    #[test]
    fn test_process() {
        assert_eq!(
            process_input(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            6
        );
    }
}
