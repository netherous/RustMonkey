//macro
#![allow(dead_code)]
use regex::Regex;
use std::{error::Error, thread::current};

#[derive(Debug, PartialEq)]
pub enum Token {
    // Identifiers + literals
    IDENT(String),
    INT(String),
    // Operators
    ASSIGN,
    PLUS,
    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    // Keywords
    FUNCTION,
    LET,
    //None match
    ILLEGAL,
    EOF,
}

pub(crate) struct Lexer {
    curr: usize,
    next: usize,
    input: Vec<u8>,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            curr: 0,
            next: 0,
            input: input.into_bytes(),
            ch: 0,
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Result<Token, Box<dyn Error>> {
        self.skip_whitespace();
        let token = match self.ch {
            b'=' => Token::ASSIGN,
            b'+' => Token::PLUS,
            b',' => Token::COMMA,
            b';' => Token::SEMICOLON,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let id = self.read_identifier();
                return Ok(match id.as_str() {
                    "fn" => Token::FUNCTION,
                    "let" => Token::LET,
                    //change to variable in the future
                    _ => Token::IDENT(id),
                });
            }
            b'0'..=b'9' => return Ok(Token::INT(self.read_number())),
            0 => Token::EOF,
            _ => Token::ILLEGAL,
        };
        self.read_char();
        Ok(token)
    }

    fn read_identifier(&mut self) -> String {
        let p = self.curr;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input.as_slice()[p..self.curr]).to_string()
    }

    fn read_char(&mut self) {
        if self.next >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = match self.input.get(self.next) {
                Some(expr) => *expr,
                None => 0,
            };
        }
        self.curr = self.next;
        self.next += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\n' || self.ch == b'\r' || self.ch == b'\t' {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> String {
        let p = self.curr;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input.as_slice()[p..self.curr]).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Token;
    use super::*;
    use std::error::Error;
    #[test]
    fn TestNextToken() -> Result<(), Box<dyn Error>> {
        let input = r#"let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);"#;

        let exp = vec![
            Token::LET,
            Token::IDENT(String::from("five")),
            Token::ASSIGN,
            Token::INT("5".to_string()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("ten")),
            Token::ASSIGN,
            Token::INT(String::from("10")),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("add")),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT(String::from("x")),
            Token::COMMA,
            Token::IDENT(String::from("y")),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT(String::from("x")),
            Token::PLUS,
            Token::IDENT(String::from("y")),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("result")),
            Token::ASSIGN,
            Token::IDENT(String::from("add")),
            Token::LPAREN,
            Token::IDENT(String::from("five")),
            Token::COMMA,
            Token::IDENT(String::from("ten")),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::EOF,
        ];
        let mut lexer = Lexer::new(input.into());
        //moved into the loop
        for token in exp {
            let next_token = lexer.next_token()?;
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }
        // exp.iter().try_for_each(|token| -> Result<(), Box<dyn Error>>{
        //     let next_token = lexer.NextToken()?;
        //     println!("expected: {:?}, received {:?}", token, next_token);
        //     assert_eq!(token, &next_token);
        //     Ok(())
        // });

        Ok(())
    }
}
