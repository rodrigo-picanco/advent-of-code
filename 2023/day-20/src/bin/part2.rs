use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> u32 {
    let mut configuration = Configuration::from(input);
    configuration.run()
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, PartialEq, Clone)]
enum State {
    On,
    Off,
}

#[derive(Debug, PartialEq, Clone)]
enum ModuleType {
    Conjuction,
    FlipFlop(State),
    Broadcaster,
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    module_type: ModuleType,
    outputs: Vec<String>,
    memory: HashMap<String, Pulse>,
}
impl Module {
    fn new(name: String, type_: String, outputs: Vec<String>) -> Self {
        Self {
            name,
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

#[derive(Debug, Clone)]
struct Configuration {
    modules: HashMap<String, Module>,
    broadcast_targets: Vec<String>,
    press_count: u32,
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
            press_count: 0,
        }
    }
}
impl Configuration {
    fn run(&mut self) -> u32 {
        let mut seen: HashMap<String, bool> = HashMap::new();
        let mut cycles: HashMap<String, u32> = HashMap::new();

        'outer: loop {
            self.press_count += 1;
            let mut queue = self
                .broadcast_targets
                .iter()
                .map(|x| ("broadcaster".to_string(), x.to_owned(), Pulse::Low))
                .collect::<VecDeque<(String, String, Pulse)>>();

            while let Some((origin, target, pulse)) = queue.pop_front() {
                match (
                    self.clone().modules.get_mut(&target),
                    self.modules
                        .values_mut()
                        .find(|x| x.outputs.contains(&"rx".to_string())),
                ) {
                    (Some(module), Some(feed)) => {
                        if module.name == feed.name && pulse == Pulse::High {
                            seen.insert(module.name.clone(), true);
                            cycles.insert(module.name.clone(), self.press_count);

                            if seen.len() == self.broadcast_targets.len() {
                                let cycles = cycles.values().copied().collect::<Vec<u32>>();
                                break 'outer lcm(cycles);
                            }
                        }

                        match module.module_type {
                            ModuleType::FlipFlop(_) => {
                                if pulse == Pulse::Low {
                                    module.module_type = match module.module_type {
                                        ModuleType::FlipFlop(State::On) => {
                                            ModuleType::FlipFlop(State::Off)
                                        }
                                        ModuleType::FlipFlop(State::Off) => {
                                            ModuleType::FlipFlop(State::On)
                                        }
                                        _ => panic!("Module type mismatch"),
                                    };

                                    let outgoing = match module.module_type {
                                        ModuleType::FlipFlop(State::On) => Pulse::High,
                                        ModuleType::FlipFlop(State::Off) => Pulse::Low,
                                        _ => panic!("Module type mismatch"),
                                    };
                                    for output in module.outputs.iter() {
                                        queue.push_back((
                                            target.clone(),
                                            output.to_owned(),
                                            outgoing,
                                        ));
                                    }
                                }
                            }
                            _ => {
                                module.memory.insert(origin, pulse);
                                let outgoing =
                                    match module.memory.values().all(|x| *x == Pulse::High) {
                                        true => Pulse::Low,
                                        false => Pulse::High,
                                    };
                                for output in module.outputs.iter() {
                                    queue.push_back((target.clone(), output.to_owned(), outgoing));
                                }
                            }
                        };
                    }
                    (None, _) => continue,
                    (_, None) => panic!("No feed found"),
                }
            }
        }
    }
}

fn lcm(nums: Vec<u32>) -> u32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(nums[1..].to_vec());
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
