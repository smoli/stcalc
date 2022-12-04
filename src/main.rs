use std::fmt;

/*
        expression =
 */

enum Symbol {
    Number(u32),
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

struct Lexer {
    input: String,
    scanner: usize
}

impl Lexer {

    fn new(input: &str) -> Lexer {
        Lexer { input: String::from(input), scanner: 0}
    }

    fn take_next(&mut self) -> Option<char> {
        self.scanner += 1;

        if self.exhausted() {
            return None;
        }

        self.current()
    }

    fn exhausted(&self) -> bool {
        self.scanner >= self.input.len()
    }

    fn current(&self) -> Option<char> {
        self.input.chars().nth(self.scanner)
    }

    fn take_numbers(&mut self, first: char) -> Symbol {
        let mut result = String::from(first);

        loop {
            if self.exhausted() {
                break;
            }
            let next = self.take_next();
            match next {
                None => break,
                Some(c) =>
                    match c {
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
                        => result.push(c),

                        _ => break

                    }


            }
        }
        Symbol::Number(result.parse().unwrap())
    }

    fn next_symbol(&mut self) -> Symbol {
        if self.exhausted() {
            return Symbol::End;
        }

        let mut next = self.current();
        loop {
            match next {
                None => break,

                Some(c) =>
                    match c {
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
                        => return self.take_numbers(c),

                        '*' | '+' | '-' | '/' | '^'
                        => {
                            self.take_next();
                            return Symbol::BinaryOperator(c)
                        },

                        '(' => {
                            self.take_next();
                            return Symbol::OpenParenthesis
                        },

                        ')' => {
                            self.take_next();
                            return Symbol::ClosedParenthesis
                        },

                        _ => if self.exhausted() { return Symbol::End; } else { next = self.take_next() }
                    }
            }
        }

        Symbol::Unknown
    }
}


impl Iterator for Lexer {

    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.next_symbol();

        return match r {
            Symbol::End => None,
            _ => Some(r)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_number() {
        let mut lexer = Lexer::new("12");
        let result = lexer.next_symbol();

        assert!(matches!(result, Symbol::Number(12)));
    }

    #[test]
    fn ignores_white_space() {
        let mut lexer = Lexer::new("   12   14");
        let mut result = lexer.next_symbol();
        assert!(matches!(result, Symbol::Number(12)));

        result =lexer.next_symbol();
        assert!(matches!(result, Symbol::Number(14)));
    }

    #[test]
    fn reads_all_symbols() {
        let mut lexer = Lexer::new("5 * (3 + 2)");

        let mut result = lexer.next_symbol();
        assert!(matches!(result, Symbol::Number(5)));

        result = lexer.next_symbol();
        assert!(matches!(result, Symbol::BinaryOperator('*')));

        result = lexer.next_symbol();
        assert!(matches!(result, Symbol::OpenParenthesis));

        result = lexer.next_symbol();
        assert!(matches!(result, Symbol::Number(3)));

        result = lexer.next_symbol();
        assert!(matches!(result, Symbol::BinaryOperator('+')));

        result = lexer.next_symbol();
        assert!(matches!(result, Symbol::Number(2)));

        result = lexer.next_symbol();
        assert!(matches!(result, Symbol::ClosedParenthesis));

        result = lexer.next_symbol();
        assert!(matches!(result, Symbol::End));
    }
}

#[test]
fn can_be_used_as_iterator() {
    let lexer = Lexer::new("5 * (3 + 2)");

    let mut count = 0;
    for _ in lexer {
        count += 1;
    }

    assert_eq!(count, 7);
}

fn main() {
    println!("Hello, world!");
}