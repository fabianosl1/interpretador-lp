use crate::lexer::{Lexer, Token};

pub enum Expression {
    Variable(String),
    Not(Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Implies(Box<Expression>, Box<Expression>),
    Iff(Box<Expression>, Box<Expression>),
    Grouped(Box<Expression>),
}

