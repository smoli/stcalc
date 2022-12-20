use clap::builder::Str;
use crate::lexer::Lexer;
use crate::lexer::symbol::{Operation, Symbol};

pub struct Parser {
    input: String,
}

pub enum ParserError {
    Error(String)
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        Parser { input: String::from(input) }
    }

    pub fn eval(input: &str) -> Result<f64, ParserError> {
        let p = Parser::new(input);
        p.evaluate()
    }

    pub fn evaluate(&self) -> Result<f64, ParserError> {
        let infix = self.transform();
        let mut stack: Vec<f64> = vec![];

        for s in infix.unwrap() {
            match s {
                Symbol::Number(n) => stack.push(n),

                Symbol::BinaryOperator(o) => {
                    if stack.len() < 2 {
                        return Result::Err(ParserError::Error("Not enough operands for operation".to_string()));
                        // panic!("Not enough arguments for operation");
                    }

                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();

                    match o {
                        Operation::Pow => stack.push(b.powf(a)),

                        Operation::Multiplication => stack.push(a * b),

                        Operation::Division => stack.push(b / a),

                        Operation::Addition => stack.push(b + a),

                        Operation::Subtraction => stack.push(b - a),
                    }
                }

                _ => panic!("Unexpected symbol {s}")
            }
        }

        if stack.len() != 1 {
            return Result::Err(ParserError::Error("Unexpected Stack Size".to_string()));
        }

        return Ok(stack.pop().unwrap());
    }

    pub fn get_precedence(op: &Operation) -> u8 {
        return match op {
            Operation::Pow => 3,
            Operation::Multiplication | Operation::Division => 2,
            Operation::Addition | Operation::Subtraction => 1,
        };
    }

    fn transform(&self) -> Option<Vec<Symbol>> {
        let mut out: Vec<Symbol> = vec![];
        let mut op: Vec<Symbol> = vec![];

        let lexer = Lexer::new(self.input.as_str());

        for s in lexer {
            match s {
                Symbol::Number(_) => out.push(s),

                Symbol::BinaryOperator(o1) => {
                    loop {
                        if op.len() == 0 { break; }
                        let top = op.last().unwrap();

                        match top {
                            Symbol::BinaryOperator(o2) => {
                                let p1 = Parser::get_precedence(&o1);
                                let p2 = Parser::get_precedence(&o2);

                                if p1 <= p2 {
                                    out.push(op.pop().unwrap());
                                } else {
                                    break;
                                }
                            }

                            _ => break
                        }
                    }

                    op.push(Symbol::BinaryOperator(o1));
                }

                Symbol::OpenParenthesis => op.push(s),

                Symbol::ClosedParenthesis => {
                    if op.len() == 0 { return None; }

                    loop {
                        if op.len() == 0 { break; }

                        let top = op.last().unwrap();

                        match top {
                            Symbol::OpenParenthesis => break,

                            _ => out.push(op.pop().unwrap())
                        }

                        op.pop();
                    }
                }

                _ => {}
            }
        }

        op.reverse();

        for s in op {
            out.push(s);
        }

        Some(out)
    }
}

#[cfg(test)]
mod test {
    // use super::*;
    use crate::parser::Parser;

    #[test]
    fn can_be_created() {
        let _parser = Parser::new("5 * (3 + 2)");

        assert!(true);
    }

    #[test]
    fn can_evaluate_expression() {
        let parser = Parser::new("5 * (3 + 2)^2");
        let result = parser.evaluate();

        assert_eq!(result, 25. * 25.);
    }

    #[test]
    fn uses_the_correct_order_for_non_commutative_operations() {
        let parser = Parser::new("5 * (3 - 2)/2");
        let result = parser.evaluate();

        assert_eq!(result, 2.5);
    }

    #[test]
    fn works_with_decimals() {
        let parser = Parser::new("2.5 + 3.1415");
        let result = parser.evaluate();

        assert_eq!(result, 2.5 + 3.1415);
    }

    #[test]
    fn works_with_exponents() {
        let parser = Parser::new("12 + 2^2");
        let result = parser.evaluate();

        assert_eq!(result, 16f64);
    }

    #[test]
    fn has_a_static_convenience_method() {
        assert_eq!(Parser::eval("1 + 2"), 3f64);
        // assert_eq!(Parser::eval("12 + 2^4"), 12.0 + 2f64.powf(4.0));
    }

    #[test]
    fn works_with_negative_numbers() {
        assert_eq!(Parser::eval("-12 + 2"), -10.0);
        assert_eq!(Parser::eval("2-2"), 0.0);
        assert_eq!(Parser::eval("2--2"), 4.0);
        assert_eq!(Parser::eval("2*-2"), -4.0);
        assert_eq!(Parser::eval("2^-2"), 2f64.powf(-2.0));
    }

    #[test]
    fn pow_works() {
        assert_eq!(Parser::eval("23 ^ 2"), 23.0 * 23.0);
    }

}