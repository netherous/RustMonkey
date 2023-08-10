use super::lexer::*;
use ast::*;
use std::{clone, error::Error, panic};
//pub mod
pub(crate) mod ast;

#[cfg(test)]
mod tests;

struct Parser<'a> {
    lexer: &'a mut Lexer,
    curToken: Token,
    peekToken: Token,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer) -> Parser<'a> {
        let ct = lexer.next_token().expect("Error reading current token");
        let pt = lexer.next_token().expect("Error reading peek token");
        Parser {
            lexer,
            curToken: ct,
            peekToken: pt,
        }
    }

    fn next_token(&mut self) {
        self.curToken = self.peekToken.clone();
        self.peekToken = self.lexer.next_token().expect("Error reading next_token");
    }

    fn peek_expect(&mut self, tok: Token) -> bool {
        tok == self.peekToken
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program::new();
        while self.curToken != Token::EOF {
            match self.parse_statement() {
                Some(stmt) => program.statements.push(stmt),
                None => {}
            }
            self.next_token();
        }
        return program;
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.curToken {
            Token::LET => Some(self.parse_let_statement()),
            Token::RETURN => None,
            Token::IF => None,
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Statement {
        let token = self.curToken.clone();
        self.next_token();
        let Identifier = match &self.curToken {
            Token::IDENT(value) => Box::new(Expression::Identifier {
                token: self.curToken.clone(),
                value: value.to_string(),
            }),
            _ => panic!("Error parsing Ident"),
        };

        // self.next_token();
        // if self.curToken != Token::EQ {
        //     panic!("No {:?} find", Token::EQ);
        // }

        todo!();
    }

    fn parse_identifier(&mut self) -> Result<Expression, Box<dyn Error>> {
        Ok(Expression::Identifier {
            token: Token::ILLEGAL,
            value: "ILLEGAL".to_string(),
        })
    }
    // fn parse_expression(&mut self) -> Result<Expression, Box<dyn Error>> {}
}
