use std::fmt;

pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Pow,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Operation::Addition => write!(f, "+"),
            Operation::Subtraction => write!(f, "-"),
            Operation::Multiplication => write!(f, "*"),
            Operation::Division => write!(f, "/"),
            Operation::Pow => write!(f, "**")
        }
    }
}

pub enum Symbol {
    Number(f64),
    BinaryOperator(Operation),
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