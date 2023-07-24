use std::{
    error::Error,
    io::{stdin, stdout, Stdin},
};

use RustMonkey::repl;
// PIPE me to a file!
fn main() {
    let vs_number = "1.0";
    println!("RustMonkey {vs_number}");
    repl::start(stdin(), stdout());
}
