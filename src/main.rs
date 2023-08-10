#![allow(non_snake_case)]
use std::io::{stdin, stdout};

use RustMonkey::repl;
// PIPE me to a file!
fn main() {
    let vs_number = "1.0";
    println!("RustMonkey {vs_number}");
    repl::start(stdin(), stdout());
}
