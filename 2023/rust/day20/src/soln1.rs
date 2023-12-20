use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
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
    subcyclesets: &Vec<(String, HashSet<String>)>,
    part2: bool,
    step_no: usize,
    prev_true: &mut HashMap<String, Vec<usize>>,
) -> (i32, i32) {
    //& is conjunction, % is flip flop, b is broadcast and o is output
    //flip flop - high - do nothing. low - flips between on and off, and sends high if turned on, low if turned off
    //conjunction - rememgers low from all inputs. on receive, update. i all memory = high, send low. else send high
    let mut to_visit = VecDeque::from(vec![("roadcaster".to_string(), false)]);
    let mut next = VecDeque::new();

    let mut los = 0;
    let mut his = 0;

    // let mut prev_substates = vec![vec![]; subcyclesets.len()];
    // for i in 0..subcyclesets.len() {
    // prev_substates.push()
    // }/

    while to_visit.len() > 0 {
        let (gate, input) = to_visit.pop_front().unwrap();

        // Inspection of my input shows these 4 gates lead to a conjunction that leads to rx.
        if gate == "bt" || gate == "dl" || gate == "fr" || gate == "rv" {
            if *states.get(&gate).unwrap() {
                prev_true.entry(gate.clone()).and_modify(|v| v.push(step_no)).or_insert(vec![step_no]);
                let prevs = prev_true.get(&gate).unwrap();
                println!("{gate} prevs = {prevs:?}");
                if prevs.len() > 3 {
                    let d1 = prevs[3] - prevs[2];
                    let d2 = prevs[2] - prevs[1];
                    let d3 = prevs[1] - prevs[0];
                    let cycle_len = d1;
                    if d1 == d2 && d2 == d3 {
                        println!("Cycle length for {gate} is probably {cycle_len}");
                    }
                }
                // println!("prevs = {prevs}")
                // println!("{gate} true at {step_no}");
            }
        }
        // if part2 {
        //     for (i, (gate, cycle_set_gates)) in subcyclesets.iter().enumerate() {
        //         let states_clone = states.clone();
        //         let substate = states_clone.into_iter().filter(|(k, v)| cycle_set_gates.contains(k)).collect_vec();
        //         if let Some((psno, z)) = prev_substates[i].iter().find_position(|&(_, z)| z == substate) {
        //             let cycle_len = step_no - psno;
        //             println!("cycle for {gate} between {step_no} and {psno} = {cycle_len}");
        //         }
        //         prev_substates[i].push((step_no, substate.clone()));
        //     }
        // }
        //    &bt -> rs
        //    &dl -> rs
        //    &fr -> rs
        //    &rv -> rs
        // if gate == "bt" && input == true {
        // println!("bt on")
        // }

        // println!("{gate} receives {input}");
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
    println!("Modules: {modmap:?}");
    let mut states = HashMap::new();
    for (k, v) in modmap.iter() {
        if v.typ == '%' || v.typ == '&' {
            states.insert(k.clone(), false);
        }
    }
    let mut los = 0usize;
    let mut his = 0usize;
    for i in 0..1000 {
        let (lo, hi) = step(&modmap, &mut states, &vec![], false, i, &mut HashMap::new());
        los += lo as usize;
        his += hi as usize;
        // println!("Lo = {lo} hi = {hi}");
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
    println!("states = {states:?}");

    // println!("Lead to BT = {lead_to_bt:?}");
    // let mut statesclone = states.clone();
    // let mut bt_states = statesclone.iter().filter(|(k, v)| lead_to_bt.contains(*k)).collect_vec();
    // bt_states.sort();
    // let mut prev_bt_states = vec![bt_states];

    // let mut bt_cycle_start = 0;
    // let mut bt_cycle_end = 0;

    let mut subcyclesets = vec![];
    for state in ["bt", "dl", "fr", "rv"] {
        let mut lead_to = HashSet::new();
        lead_to.insert(state.to_string());
        for i in 0..1000 {
            for (gate, module) in modmap.iter() {
                if module.outputs.iter().filter(|z| lead_to.contains(*z)).next().is_some() {
                    lead_to.insert(gate.clone());
                }
            }
        }
        subcyclesets.push((state.to_string(), lead_to));
    }
    // let mut prev_state_bt = vec![states];

    let mut prev_states = vec![states.clone()];
    let mut prev_true: HashMap<String, Vec<usize>> = HashMap::new();
    for i in 0..1000000 {
        let (lo, hi) = step(&modmap, &mut states, &subcyclesets, true, i + 1, &mut prev_true);
        // let statesclon = states.clone();
        // let bt_states = statesclon.iter().filter(|(k, v)| lead_to_bt.contains(*k)).collect_vec();
        // if prev_bt_states.contains(&bt_states) {
        // println!("Found BT cycle at i = {i}");
        // }
        // if prev_states.contains(&states) {
        // println!("Found cycle at i = {i}");
        // }
        // prev_states.push(states.clone());
        // if *states.get("bt").unwrap() == true {
        // println!("BT is true at i = {i}");
        // }
    }
    0
    /*
       &rs -> rx
       &bt -> rs
       &dl -> rs
       &fr -> rs
       &rv -> rs
    */
}
