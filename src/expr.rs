use crate::token::Token;
use crate::values::Literal;

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Option<Literal>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}