use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> usize {
    todo!()
}


type Schema = BTreeMap<String, State>;

#[derive(Debug, PartialEq, Clone)]
struct State {
    name: String,
    transitions: (String, String)
}
impl State {
    fn new(name: String, transitions: (String, String)) -> Self {
        Self {
            name,
            transitions
        }
    }
    
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Left,
    Right
}

#[derive(Debug, PartialEq, Clone)]
struct WalkMachine {
    state: State,
    schema: Schema,
    steps: u8
}
impl WalkMachine {
    fn new(schema: Schema) -> Self {
        let initial = schema.get("initial").expect("No initial state").clone();
        Self {
            state: schema.get(&initial.name).expect("No initial state").clone(),
            schema,
            steps: 0
        }
    }

    // TODO: Could we avoid cloning the state?
    fn walk(&mut self, direction: Direction) {
        self.steps += 1;
        match direction {
            Direction::Left => {
                self.state = self.schema.get(&self.state.transitions.0).expect("No state. Cannot move left").clone();
            },
            Direction::Right => {
                self.state = self.schema.get(&self.state.transitions.1).expect("No state. Cannot move right").clone();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walk() {
        use Direction::*;
        let mut schema = Schema::new();
        schema.insert("initial".to_string(), State::new("AAA".to_string(), ("".to_string() , "".to_string())));
        schema.insert("AAA".to_string(), State::new("AAA".to_string(), ("BBB".to_string(), "BBB".to_string())));
        schema.insert("BBB".to_string(), State::new("BBB".to_string(), ("AAA".to_string(), "CCC".to_string())));
        schema.insert("CCC".to_string(), State::new("CCC".to_string(), ("CCC".to_string(), "CCC".to_string())));
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
    fn test_create_machine() {
        let mut schema = Schema::new();
        schema.insert("initial".to_string(), State::new("AAA".to_string(), ("".to_string(), "".to_string())));
        schema.insert("AAA".to_string(), State::new("AAA".to_string(), ("BBB".to_string(), "CCC".to_string())));
        let machine = WalkMachine::new(schema);
        assert_eq!(machine.state.name, "AAA");
    }

    #[test]
    fn test_process() {
        assert_eq!(
            process_input(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            6
        );
    }
}
