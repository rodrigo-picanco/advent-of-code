use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

const LOW: &'static str = "low";
const HIGH: &'static str = "high";
const ON: &'static str = "on";
const OFF: &'static str = "off";

fn process_input(input: &str) -> usize {
    let mut configuration = Configuration::from(input);
    let mut high = 0;
    let mut low = 0;

    for _ in 0..1000 {
        low += 1;
        let mut queue = configuration
            .broadcast_targets
            .iter()
            .map(|x| ("broadcaster".to_string(), x.to_owned(), LOW.to_string()))
            .collect::<VecDeque<(String, String, String)>>();
        while let Some((origin, target, pulse)) = queue.pop_front() {
            if &pulse == LOW {
                low += 1;
            } else {
                high += 1;
            }
            if !configuration.modules.contains_key(&target) {
                continue;
            }
            let module = configuration.modules.get_mut(&target).unwrap();
            if module.type_ == "%" {
                if pulse == LOW {
                    module.memory.insert(
                        "state".to_string(),
                        match module.memory.get("state").unwrap().as_str() {
                            ON => OFF.to_string(),
                            OFF => ON.to_string(),
                            _ => panic!("Invalid state"),
                        },
                    );
                    let outgoing = match module.memory.get("state").unwrap().as_str() {
                        ON => HIGH,
                        OFF => LOW,
                        _ => panic!("Invalid state"),
                    };
                    for output in module.outputs.iter() {
                        queue.push_back((target.clone(), output.to_owned(), outgoing.to_string()));
                    }
                }
            } else {
                module.memory.insert(origin, pulse);
                let outgoing = match module.memory.values().all(|x| x == HIGH) {
                    true => LOW,
                    false => HIGH,
                };
                for output in module.outputs.iter() {
                    queue.push_back((target.clone(), output.to_owned(), outgoing.to_string()));
                }
            }
        }
    }
    high * low
}

struct Module {
    name: String,
    type_: String,
    outputs: Vec<String>,
    memory: HashMap<String, String>,
}
impl Module {
    fn new(name: String, type_: String, outputs: Vec<String>) -> Self {
        let mut memory = HashMap::new();
        if type_ == "%" {
            memory.insert("state".to_string(), OFF.to_string());
        }
        Self {
            name,
            type_,
            outputs,
            memory,
        }
    }
}

struct Configuration {
    modules: HashMap<String, Module>,
    broadcast_targets: Vec<String>,
}
impl From<&str> for Configuration {
    fn from(input: &str) -> Self {
        let mut modules = HashMap::new();
        let mut broadcast_targets = vec![];
        for line in input.lines() {
            let mut parts = line.split(" -> ");
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();
            let outputs = right.split(", ").map(|x| x.to_string());
            if left == "broadcaster" {
                broadcast_targets.extend(outputs);
            } else {
                let type_ = left.chars().take(1).collect::<String>();
                let name = left.chars().skip(1).collect::<String>();
                modules.insert(
                    name.clone(),
                    Module::new(name, type_, outputs.collect::<Vec<String>>()),
                );
            }
        }

        let mut modifications: Vec<(String, String, String)> = Vec::new();
        for (name, module) in &modules {
            for output in &module.outputs {
                if let Some(output_module) = modules.get(output) {
                    if output_module.type_ == "&" {
                        modifications.push((output.to_string(), name.clone(), "lo".to_string()));
                    }
                }
            }
        }

        for (output, name, value) in modifications {
            if let Some(output_module) = modules.get_mut(&output) {
                output_module.memory.insert(name, value);
            }
        }

        Self {
            modules,
            broadcast_targets,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(
        "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
        32000000
    )]
    #[case(
        "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
        11687500
    )]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process_input(input), expected);
    }
}
