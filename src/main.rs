mod collector;
mod inout;
mod utils;

use std::env::args;

use crate::inout::{FStatInput, FStatOutput};

fn main() {
    let input = FStatInput::from(args_as_vector());
    let output = FStatOutput::from(&input);
    println!("Input: {input:#?}\n\nOutput: {output}");
}

fn args_as_vector() -> Vec<String> {
    args().collect()
}
