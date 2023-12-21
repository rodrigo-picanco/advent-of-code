use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> u32 {
    let mut configuration = Configuration::from(input);

    for _ in 0..1000 {
        configuration.run();
    }

    configuration.pulse_counter.high * configuration.pulse_counter.low
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, PartialEq)]
enum State {
    On,
    Off,
}

#[derive(Debug, PartialEq)]
enum ModuleType {
    Conjuction,
    FlipFlop(State),
    Broadcaster,
}

struct Module {
    module_type: ModuleType,
    outputs: Vec<String>,
    memory: HashMap<String, Pulse>,
}
impl Module {
    fn new(type_: String, outputs: Vec<String>) -> Self {
        Self {
            module_type: match type_.as_str() {
                "%" => ModuleType::FlipFlop(State::Off),
                "&" => ModuleType::Conjuction,
                "broadcaster" => ModuleType::Broadcaster,
                _ => panic!("Invalid module type"),
            },
            outputs,
            memory: HashMap::new(),
        }
    }
}

struct PulseCounter {
    high: u32,
    low: u32,
}

struct Configuration {
    modules: HashMap<String, Module>,
    broadcast_targets: Vec<String>,
    pulse_counter: PulseCounter,
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
                    Module::new(type_, outputs.collect::<Vec<String>>()),
                );
            }
        }
        let mut modifications: Vec<(String, String, Pulse)> = Vec::new();
        for (name, module) in &modules {
            for output in &module.outputs {
                if let Some(output_module) = modules.get(output) {
                    if output_module.module_type == ModuleType::Conjuction {
                        modifications.push((output.to_string(), name.clone(), Pulse::Low));
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
            pulse_counter: PulseCounter { high: 0, low: 0 },
        }
    }
}

impl Configuration {
    fn increment(&mut self, pulse: Pulse) {
        match pulse {
            Pulse::Low => self.pulse_counter.low += 1,
            Pulse::High => self.pulse_counter.high += 1,
        }
    }
    fn run(&mut self) {
        self.increment(Pulse::Low);

        let mut queue = self
            .broadcast_targets
            .iter()
            .map(|x| ("broadcaster".to_string(), x.to_owned(), Pulse::Low))
            .collect::<VecDeque<(String, String, Pulse)>>();

        while let Some((origin, target, pulse)) = queue.pop_front() {
            self.increment(pulse);

            match self.modules.get_mut(&target) {
                Some(module) => match module.module_type {
                    ModuleType::FlipFlop(_) => {
                        if pulse == Pulse::Low {
                            module.module_type = match module.module_type {
                                ModuleType::FlipFlop(State::On) => ModuleType::FlipFlop(State::Off),
                                ModuleType::FlipFlop(State::Off) => ModuleType::FlipFlop(State::On),
                                _ => panic!("Module type mismatch"),
                            };

                            let outgoing = match module.module_type {
                                ModuleType::FlipFlop(State::On) => Pulse::High,
                                ModuleType::FlipFlop(State::Off) => Pulse::Low,
                                _ => panic!("Module type mismatch"),
                            };

                            for output in module.outputs.iter() {
                                queue.push_back((target.clone(), output.to_owned(), outgoing));
                            }
                        }
                    }
                    _ => {
                        module.memory.insert(origin, pulse);
                        let outgoing = match module.memory.values().all(|x| *x == Pulse::High) {
                            true => Pulse::Low,
                            false => Pulse::High,
                        };
                        for output in module.outputs.iter() {
                            queue.push_back((target.clone(), output.to_owned(), outgoing));
                        }
                    }
                },
                None => continue,
            }
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
    fn test_process(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(process_input(input), expected);
    }
}
