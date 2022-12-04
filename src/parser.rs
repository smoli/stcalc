use crate::lexer::Lexer;
use crate::lexer::symbol::Symbol;

pub struct Parser {
    input: String,
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        Parser { input: String::from(input) }
    }

    pub fn evaluate(&self) -> f64 {
        let infix = self.transform();
        let mut stack: Vec<f64> = vec![];

        for s in infix.unwrap() {
            match s {
                Symbol::Number(n) => stack.push(n),

                Symbol::BinaryOperator(o) => {
                    if stack.len() < 2 {
                        panic!("Not enough arguments for operation");
                    }

                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();

                    match o {
                        '^' => stack.push(b.powf(a)),

                        '*' => stack.push(a * b),

                        '/' => stack.push(b / a),

                        '+' => stack.push(b + a),

                        '-' => stack.push(b - a),

                        _ => {
                            panic!("Unknown operator {o}")
                        }
                    }
                }

                _ => panic!("Unexpected symbol {s}")
            }
        }

        if stack.len() != 1 {
            panic!("Unexpected stack size {}", stack.len());
        }

        return stack.pop().unwrap();
    }

    pub fn get_precedence(op: &char) -> u8 {
        return match op {
            '^' => 3,
            '*' | '/' => 2,
            '+' | '-' => 1,
            _ => 0
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
                                }
                            }

                            _ => break
                        }
                    }

                    op.push(s);
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
}