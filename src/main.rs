use bitvec::{order::Local, vec::BitVec};
use perf_event::Builder;

const EXPECTED_INSTRUCTION_COUNT: u64 = 1012840155;
const ITERATION: usize = 20000;

fn main() {
    let _ = std::env::args().nth(0).unwrap();
    let mut counter = Builder::new().build().unwrap();
    let _: Vec<bool> = serde_json::from_str("[]").unwrap();
    let results: Vec<bool> = vec![false; 1000];
    for _ in 0..ITERATION {
        let mut bitvec = BitVec::<Local, usize>::with_capacity(results.len());
        /* Interesting part starts here */
        counter.enable().unwrap();
        bitvec.extend(results.iter().copied());
        counter.disable().unwrap();
        /* and ends here */
    }

    let instructions = counter.read().unwrap();
    println!("instructions:u = {:?}", instructions);
    if 100 * instructions > 105 * EXPECTED_INSTRUCTION_COUNT {
        println!(
            "Regression of about {}% detected!",
            100 * instructions / EXPECTED_INSTRUCTION_COUNT - 100
        );
        std::process::exit(1);
    }
}
