use std::collections::{BTreeMap, VecDeque};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, Clone)]
struct Module<'a> {
    module_type: ModuleType,
    inputs: Vec<&'a str>,
    outputs: Vec<&'a str>,
    last_inputs: BTreeMap<&'a str, bool>,
    last_output: bool,
}

#[derive(Debug, Clone)]
struct Signal<'a> {
    source: &'a str,
    destination: &'a str,
    value: bool,
}

fn push_button(modules: &mut BTreeMap<&str, Module>) -> (usize, usize) {
    let mut queue: VecDeque<Signal> = VecDeque::new();
    let mut history: Vec<Signal> = Vec::new();

    let signal = Signal {
        source: "button",
        destination: "broadcaster",
        value: false,
    };
    queue.push_back(signal.clone());
    history.push(signal);

    while queue.len() > 0 {
        let signal = queue.pop_front().unwrap();
        if let Some(module) = modules.get_mut(signal.destination) {
            match module.module_type {
                ModuleType::Broadcaster => {
                    module.outputs.iter().for_each(|output| {
                        let sig = Signal {
                            source: signal.destination,
                            destination: output,
                            value: signal.value,
                        };
                        queue.push_back(sig.clone());
                        history.push(sig);
                    });
                }
                ModuleType::FlipFlop => {
                    if signal.value == false {
                        module.last_output = !module.last_output;
                        module.outputs.iter().for_each(|output| {
                            let sig = Signal {
                                source: signal.destination,
                                destination: output,
                                value: module.last_output,
                            };
                            queue.push_back(sig.clone());
                            history.push(sig);
                        });
                    }
                }
                ModuleType::Conjunction => {
                    module.last_inputs.insert(signal.source, signal.value);
                    let high_inputs = module
                        .last_inputs
                        .iter()
                        .filter(|(_, v)| **v == true)
                        .count();
                    let mut sig_value = true;
                    if high_inputs == module.inputs.len() {
                        sig_value = false;
                    }
                    module.outputs.iter().for_each(|output| {
                        let sig = Signal {
                            source: signal.destination,
                            destination: output,
                            value: sig_value,
                        };
                        queue.push_back(sig.clone());
                        history.push(sig);
                    });
                }
            }
        }
    }

    let low_pulses = history.iter().filter(|s| !s.value).count();
    let high_pulses = history.iter().filter(|s| s.value).count();
    (low_pulses, high_pulses)
}

fn process(input: &str) -> usize {
    let mut modules: BTreeMap<&str, Module> = BTreeMap::new();
    input.lines().for_each(|line| {
        let (id, connections) = line.split("->").map(|s| s.trim()).collect_tuple().unwrap();
        let (module_type, name) = match id.chars().next().unwrap() {
            'b' => (ModuleType::Broadcaster, "broadcaster"),
            '%' => (ModuleType::FlipFlop, &id[1..]),
            '&' => (ModuleType::Conjunction, &id[1..]),
            _ => panic!("Unknown module type"),
        };
        let outputs = connections.split(',').map(|s| s.trim()).collect::<Vec<_>>();
        modules.insert(
            name,
            Module {
                module_type,
                inputs: vec![],
                outputs,
                last_inputs: BTreeMap::new(),
                last_output: false,
            },
        );
    });

    let m = modules.clone();
    for (name, module) in m.iter() {
        module.outputs.iter().for_each(|output| {
            if let Some(m) = modules.get_mut(output) {
                m.inputs.push(name);
            }
        });
    }

    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for _ in 0..1000 {
        let (low, high) = push_button(&mut modules);
        low_pulses += low;
        high_pulses += high;
    }
    low_pulses * high_pulses
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const TEST2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST1), 32000000);
        assert_eq!(process(TEST2), 11687500);
    }
}
