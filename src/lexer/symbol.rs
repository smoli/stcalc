use std::fmt;

pub enum Symbol {
    Number(f64),
    BinaryOperator(char),
    OpenParenthesis,
    ClosedParenthesis,
    End,
    Unknown,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Symbol::Number(n) => write!(f, "Number {}", n),
            Symbol::BinaryOperator(o) => write!(f, "Binary Operator {}", o),
            Symbol::OpenParenthesis => write!(f, "Open Parenthesis"),
            Symbol::ClosedParenthesis => write!(f, "Closed Parenthesis"),
            Symbol::End => write!(f, "End"),
            Symbol::Unknown => write!(f, "Unknown"),
        }
    }
}