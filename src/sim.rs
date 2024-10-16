use crate::TM;


struct Tape {

}

// Sees if the turing machine halts in some steps
fn sim(tm: TM, steps: usize) -> bool {
    let mut steps = 0;
    let mut current_state = "q1";

    loop {
        if steps > 500 {
            return false;
        }

        steps += 1;
    }

    true
}
