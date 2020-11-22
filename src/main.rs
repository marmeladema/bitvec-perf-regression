use bitvec::{order::Local, vec::BitVec};
use perf_event::Builder;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("at least 1 command line argument");
    let file = File::open(filename).unwrap();
    let mut vec_of_bitvec = Vec::with_capacity(17793);
    let mut counter = Builder::new().build().unwrap();
    for line in BufReader::new(file).lines() {
        let results: Vec<Option<bool>> = serde_json::from_str(&line.unwrap()).unwrap();
        let mut bitvec = BitVec::<Local, usize>::with_capacity(results.len());
        /* Interesting part starts here */
        counter.enable().unwrap();
        bitvec.extend(results.iter().map(|result| result.unwrap()));
        counter.disable().unwrap();
        /* and ends here */
        vec_of_bitvec.push(bitvec);
    }

    let instructions = counter.read().unwrap();
    println!("instructions:u = {:?}", instructions);
    if 100 * instructions > 105 * 1272309573 {
        println!(
            "Regression of about {}% detected!",
            100 * instructions / 1272309573 - 100
        );
        std::process::exit(1);
    }
}
