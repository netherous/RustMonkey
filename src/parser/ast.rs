use super::Token;
trait Node {}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Expression {
    Identifier { token: Token, value: String },
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Statement {
    Let {
        token: Token,
        identifier: Box<Expression>,
        value: Box<Expression>,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }
}
