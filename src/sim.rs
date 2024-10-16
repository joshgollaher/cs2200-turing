use std::collections::HashMap;
use crate::TM;


struct Tape {
    map: HashMap<i32, String>,
    default_symbol: String,
}

impl Tape {
    fn new(default_symbol: String) -> Self {
        Self {
            map: HashMap::new(),
            default_symbol
        }
    }

    fn symbol_at(&self, pos: i32) -> String {
        if self.map.contains_key(&pos) {
            self.map.get(&pos).unwrap().clone()
        } else {
            self.default_symbol.clone()
        }
    }

    fn set_symbol_at(&mut self, pos: i32, new_sym: String) {
        self.map.insert(pos, new_sym);
    }
}

fn movement_for_direction(direction: String) -> i32 {
    match direction.as_str() {
        "<" => -1,
        ">" => 1,
        "-" => 0,
        _ => panic!("Unknown direction")
    }
}

// Sees if the turing machine halts in some steps, if it does return the steps
pub fn sim(tm: TM, min_steps: usize, max_steps: usize) -> Option<usize> {
    let mut steps = 0;
    let mut current_state = "q0".to_string();
    let mut current_pos = 0;
    let mut tape = Tape::new("_".to_string());

    loop {
        if steps > max_steps {
            return None;
        }

        let mut transitioned = false;
        for ((curr_state, sym), transition) in &tm.transitions {
            if *curr_state == current_state && *sym == tape.symbol_at(current_pos) {
                // Transition
                current_state = transition.next_state.clone();
                tape.set_symbol_at(current_pos, transition.write.clone());
                current_pos += movement_for_direction(transition.movement.clone());
                transitioned = true;
                break;
            }
        }

        if current_state == String::from("qH") || !transitioned {
            if steps < min_steps {
                return None;
            }

            return Some(steps)
        }

        steps += 1;
    }
}
