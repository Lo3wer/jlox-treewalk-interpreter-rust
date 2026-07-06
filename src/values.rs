use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Bool(bool),
    String(String),
    Number(f64),
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Bool(value) => write!(f, "{}", value),
            Literal::String(value) => write!(f, "{}", value),
            Literal::Number(value) => {
                if value.fract() == 0.0 {
                    write!(f, "{:.0}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Literal::Nil => write!(f, "nil"),
        }
    }
}
