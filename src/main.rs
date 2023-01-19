mod collector;
mod inout;
mod utils;

use std::env::args;

use crate::inout::FStatInput;

fn main() {
    let input = FStatInput::from(args_as_vector());
    println!("Input: {input:?}");
}

fn args_as_vector() -> Vec<String> {
    args().collect()
}
