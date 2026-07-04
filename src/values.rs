#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Bool(bool),
    String(String),
    Number(f64),
    Nil,
}
