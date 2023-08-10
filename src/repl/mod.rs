use super::lexer::*;
use std::{
    error::Error,
    io::{self, BufRead, BufReader, BufWriter, Stdin, Stdout, Write},
};

pub fn start(input: Stdin, output: Stdout) {
    loop {
        let mut buffer = String::new();
        {
            let _i = input
                .lock()
                .read_line(&mut buffer)
                .expect("Failed to Read from stdin");
        }
        let mut l = Lexer::new(buffer.clone());
        let mut guard = output.lock();
        loop {
            let tok = l.next_token().expect("Error getting next token");
            if tok == Token::EOF {
                break;
            }
            writeln!(guard, "{:?}", tok).expect("Error writing to stdout");
        }
    }
    // Ok(())
}
