use std::env;
use std::error::Error;
use std::fs::File;
use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};
use std::str::FromStr;
use regex::Regex;

#[derive(Debug, Clone)]
struct ParseErr;

impl Error for ParseErr {}

impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "parse error")
    }
}

#[derive(Debug, Clone)]
enum ModuleKind {
    Broadcaster,
    FlipFlop,
    Conjunction,
    Unknown,
}

impl FromStr for ModuleKind {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => { Ok(ModuleKind::Broadcaster) },
            "%" => { Ok(ModuleKind::FlipFlop) },
            "&" => { Ok(ModuleKind::Conjunction) },
            _ => Err(ParseErr{}),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
struct Module {
    inputs: HashMap<String, Pulse>,
    outputs: Vec<String>,
    kind: ModuleKind,
    state: bool,
}

impl Default for Module {
        fn default() -> Self {
            return Self {
                inputs: HashMap::new(),
                outputs: Vec::new(),
                kind: ModuleKind::Unknown,
                state: false,
            };
        }
}

impl Module {
    fn set_kind(&mut self, kind: ModuleKind) {
        self.kind = kind;
    }

    fn add_input(&mut self, ip: &str) {
        self.inputs.insert(ip.to_string(), Pulse::Low);
    }

    fn add_output(&mut self, op: &str) {
        self.outputs.push(op.to_string());
    }

    fn __send_pulse(&self, pulse: Pulse) -> Vec<(String, Pulse)> {
        return Vec::from_iter(self.outputs.iter().map(|name| (name.to_string(), pulse)));
    }

    fn receive_pulse(&mut self, from: &str, pulse: Pulse) -> Option<Vec<(String, Pulse)>> {
        self.inputs.insert(from.to_string(), pulse);

        match self.kind {
            ModuleKind::Broadcaster => {
                Some(self.__send_pulse(pulse))
            },
            ModuleKind::FlipFlop => {
                match pulse {
                    Pulse::Low => {
                        self.state = !self.state;
                        Some(self.__send_pulse(if self.state { Pulse::High } else { Pulse::Low }))
                    },
                    Pulse::High => {
                        None
                    },
                }
            },
            ModuleKind::Conjunction => {
                if self.inputs.values().all(|&v| v == Pulse::High) {
                    Some(self.__send_pulse(Pulse::Low))
                } else {
                    Some(self.__send_pulse(Pulse::High))
                }
            },
            ModuleKind::Unknown => { None },
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let module_re = Regex::new(r"([%&]?)([a-zA-z]+)").unwrap();

    let mut modules: HashMap<String, Module> = HashMap::new();

    while let Some(line) = lines.next() {
        let line = line?;

        let (left, right) = line.split_once(" -> ").unwrap();

        let caps = module_re.captures(&left).unwrap();
        let kind = caps.get(1).unwrap().as_str();
        let module_kind = ModuleKind::from_str(kind)?;
        let module_name = caps.get(2).unwrap().as_str();

        let module = modules.entry(module_name.to_string()).or_insert(Default::default());
        module.set_kind(module_kind);

        let outputs: Vec<&str> = right.split(", ").collect();
        for output in &outputs {
            module.add_output(output);
        }

        for output in &outputs {
            let op_module = modules.entry(output.to_string()).or_insert(Default::default());
            op_module.add_input(module_name);
        }
    }

    let mut pulses: VecDeque<(String, String, Pulse)> = VecDeque::new();

    let mut high_pulses: u64 = 0;
    let mut low_pulses: u64 = 0;

    let num_buttons = 1000;
    for _ in 0..num_buttons {
        pulses.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

        while let Some((source, target, pulse)) = pulses.pop_front() {
            //println!("{} -{:?}-> {}", source, pulse, target);

            match pulse {
                Pulse::High => high_pulses += 1,
                Pulse::Low => low_pulses += 1,
            }

            let module = modules.get_mut(&target).unwrap();
            if let Some(new_pulses) = module.receive_pulse(&source, pulse) {
                for (new_target, new_pulse) in new_pulses {
                    pulses.push_back((target.clone(), new_target, new_pulse));
                }
            }
        }
    }

    println!("{}", low_pulses * high_pulses);

    Ok(())
}
