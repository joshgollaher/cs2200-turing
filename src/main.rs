mod sim;

use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::string::ToString;
use rayon::prelude::*;
use crate::sim::sim;


fn symbols() -> [String; 3] {
    ["_".to_string(), "0".to_string(), "1".to_string()]
}

fn states() -> [String; 3] {
    ["q0".to_string(), "q1".to_string(), "qH".to_string()]
}

fn moves() -> [String; 3] {
    ["<".to_string(), ">".to_string(), "-".to_string()]
}

fn next_states() -> [String; 3] {
    ["q0".to_string(), "q1".to_string(), "qH".to_string()]
}


#[derive(Debug, Clone)]
struct Transition {
    write: String,
    movement: String,
    next_state: String,
}

impl Display for Transition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.next_state, self.write, self.movement)
    }
}

#[derive(Debug, Clone)]
struct TM {
    transitions: HashMap<(String, String), Transition>
}

impl Display for TM {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for ((state, read), transition) in &self.transitions {
            writeln!(f, "{},{}\n{}\n", state, read, transition);
        }

        Ok(())
    }
}

fn generate_all_transitions() -> Vec<Transition> {
    symbols()
        .iter()
        .cartesian_product(moves().iter())
        .cartesian_product(next_states().iter())
        .map(|((write, movement), next_state)| Transition {
            write: write.clone(),
            movement: movement.clone(),
            next_state: next_state.clone(),
        })
        .collect()
}

fn generate_all_turing_machines() -> Vec<TM> {
    let all_transitions = generate_all_transitions();

    let states = states();
    let symbols = symbols();
    let ss_pairs = states.iter().take(2).cartesian_product(symbols.iter()).collect::<Vec<_>>();

    all_transitions
        .iter()
        .combinations_with_replacement(ss_pairs.len())
        .map(|transitions| {
            let mut machine = TM {
                transitions: HashMap::new(),
            };

            for ((state, symbol), transition) in ss_pairs.clone().into_iter().zip(transitions) {
                machine.transitions.insert((state.clone(), symbol.clone()), (*transition).clone());
            }

            machine
        })
        .collect()
}

fn main() {

    const MIN_STEPS: usize = 7;
    const MAX_STEPS: usize = 12;

    let all_tm = generate_all_turing_machines();
    let valid_tms = all_tm.into_iter().filter(|tm| {
        let mut valid = false;
        for transition in tm.transitions.values() {
            if transition.next_state == "qH".to_string() {
                valid = true;
            }
        }

        valid
    }).filter(|tm| {
        let mut valid = true;

        for (start_state, _) in tm.transitions.keys() {
            if *start_state == "qH".to_string() {
                valid = false;
            }
        }

        valid
    }).filter(|tm | sim(tm.clone(), MIN_STEPS, MAX_STEPS).is_some())
        .sorted_by_key(|tm| sim(tm.clone(), MIN_STEPS, MAX_STEPS).unwrap()).collect::<Vec<_>>();

    println!("Got {} turing machines.", valid_tms.len());

    if valid_tms.len() > 0 {
        println!("Machine:\n{}", valid_tms[0]);
    }
}
