use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use num::Integer;
use regex::Regex;

pub type Input = HashMap<String, Module>;
pub type Output = usize;

#[derive(Debug, Clone)]
pub struct Module {
    typ: char,
    name: String,
    outputs: Vec<String>,
    inputs: Vec<String>,
}
pub fn parse(input: &str) -> Input {
    let lines = input.lines();
    let mut modules = HashMap::new();
    for line in lines {
        let (lhs, rhs) = line.split_once(" -> ").unwrap();
        let rhs_parts = rhs.split(",").map(|x| x.trim().to_string()).collect_vec();
        let mut lchars = lhs.chars();
        let lhs_type = lchars.next().unwrap();
        let nchars = lchars.collect::<String>();
        let name = nchars.trim().to_string();
        let module = Module { typ: lhs_type, name: name.clone(), outputs: rhs_parts, inputs: vec![] };
        modules.insert(name, module);
    }
    let mod_keys = modules.keys().cloned().collect_vec();
    for mname in mod_keys {
        let module = modules.get(&mname).unwrap();
        if module.typ == '&' {
            let mut inputs = vec![];
            for (mn2, mod2) in &modules {
                if mod2.outputs.contains(&mname.to_string()) {
                    inputs.push(mn2.to_string());
                }
            }
            let mod_new =
                Module { typ: module.typ, name: mname.to_string(), outputs: module.outputs.clone(), inputs: inputs };
            modules.remove(&mname);
            modules.insert(mname, mod_new);
        }
    }

    modules
}

fn step(
    modmap: &Input,
    states: &mut HashMap<String, bool>,
    part2: bool,
    step_no: usize,
    prev_true: &mut HashMap<String, Vec<usize>>,
    cycle_lens: &mut HashMap<String, usize>,
) -> (usize, usize) {
    //& is conjunction, % is flip flop, b is broadcast and o is output
    //% = flip flop - high - do nothing. low - flips between on and off, and sends high if turned on, low if turned off
    //& = conjunction - remembers low from all inputs. on receive, update. if all inputs = high, send low. else send high
    let mut to_visit = VecDeque::from(vec![("roadcaster".to_string(), false)]);
    let mut next = VecDeque::new();

    let mut los = 0;
    let mut his = 0;

    while to_visit.len() > 0 {
        let (gate, input) = to_visit.pop_front().unwrap();

        if part2
            && let Some(true) = states.get(&gate)
            && let Some(previous_steps) = prev_true.get_mut(&gate)
        {
            if !previous_steps.contains(&step_no) {
                previous_steps.push(step_no);
            }
            if previous_steps.len() > 3 {
                let diffs = previous_steps.windows(2).map(|win| win[1] - win[0]).rev().take(3).collect_vec();
                if diffs.iter().all_equal() {
                    let cycle_len = diffs[0];
                    cycle_lens.insert(gate.clone(), cycle_len);
                    println!("Cycle length for gate {gate} is {cycle_len}");
                }
            }
            if cycle_lens.len() == prev_true.len() {
                return (cycle_lens.values().fold(1, |acc, e| acc.lcm(e)), 0);
            }
        }

        if input == false {
            los += 1;
        } else {
            his += 1;
        }

        if !modmap.contains_key(&gate) {
            if to_visit.len() == 0 {
                to_visit = next;
                next = VecDeque::new();
            }
            continue;
        }
        let module = modmap.get(&gate).unwrap();

        let mtyp = module.typ;
        let outputs = &module.outputs;

        match (mtyp, input) {
            ('b', false) => {
                for output in outputs {
                    next.push_back((output.clone(), false));
                }
            }
            ('%', true) => (),
            ('%', false) => {
                let gs = gate.to_string();
                states.entry(gs).and_modify(|v| *v = !*v);

                let new_state = states.get(&module.name).unwrap();
                for output in outputs {
                    next.push_back((output.clone(), *new_state));
                }
            }
            ('&', _) => {
                let mut all_hi = true;
                for input in &module.inputs {
                    if *states.get(input).unwrap() == false {
                        all_hi = false;
                    }
                }
                states.entry(gate.to_string()).and_modify(|v| *v = !all_hi);
                let my_out = !all_hi;
                for output in outputs {
                    next.push_back((output.clone(), my_out));
                }
            }
            ('_', _) => (),
            _ => panic!(),
        }
        if to_visit.len() == 0 {
            to_visit = next;
            next = VecDeque::new();
        }
    }
    (los, his)
}

pub fn part1(raw_input: &str) -> Output {
    let modmap = parse(raw_input);
    let mut states = HashMap::new();
    for (k, v) in modmap.iter() {
        if v.typ == '%' || v.typ == '&' {
            states.insert(k.clone(), false);
        }
    }
    let mut los = 0usize;
    let mut his = 0usize;
    for i in 0..1000 {
        let (lo, hi) = step(&modmap, &mut states, false, i, &mut HashMap::new(), &mut HashMap::new());
        los += lo as usize;
        his += hi as usize;
    }
    println!("{los} Lo {his} Hi");
    los * his
}

pub fn part2(raw_input: &str) -> Output {
    let modmap = parse(raw_input);
    let mut states = HashMap::new();
    for (k, v) in modmap.iter() {
        if v.typ == '%' || v.typ == '&' {
            states.insert(k.clone(), false);
        }
    }

    /*
       &rs -> rx
       &bt -> rs
       &dl -> rs
       &fr -> rs
       &rv -> rs
       In my input, rx has a single incoming edge from a conjunction, we check to ensure this and solve the problem for this special case
    */
    let rx_inputs = &modmap.iter().filter(|(k, v)| (*v).outputs.len() == 1 && (*v).outputs[0] == "rx").collect_vec();
    assert!(rx_inputs.len() == 1);
    let rx_input = rx_inputs[0].1;
    assert!(rx_input.typ == '&');
    let prev_inputs = &rx_input.inputs;

    let mut prev_true: HashMap<String, Vec<usize>> = HashMap::new();
    prev_inputs.iter().for_each(|gate| {
        prev_true.insert(gate.clone(), vec![]);
    });
    let mut cycle_lens = HashMap::new();
    for i in 0..1000000 {
        let (lo, hi) = step(&modmap, &mut states, true, i + 1, &mut prev_true, &mut cycle_lens);
        if hi == 0 {
            return lo;
        }
    }
    unreachable!()
}
