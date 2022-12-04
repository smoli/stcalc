use std::fmt;

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

struct Calc {
    input: String,
    scanner: usize
}

impl Calc {

    fn new(input: &str) -> Calc {
        Calc { input: String::from(input), scanner: 0}
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_number() {
        let mut calc = Calc::new("12");
        let result = calc.next_symbol();

        assert!(matches!(result, Symbol::Number(12)));
    }

    #[test]
    fn ignores_white_space() {
        let mut calc = Calc::new("   12   14");
        let mut result = calc.next_symbol();
        assert!(matches!(result, Symbol::Number(12)));

        result =calc.next_symbol();
        assert!(matches!(result, Symbol::Number(14)));
    }

    #[test]
    fn reads_all_symbols() {
        let mut calc = Calc::new("5 * (3 + 2)");

        let mut result = calc.next_symbol();
        assert!(matches!(result, Symbol::Number(5)));

        result = calc.next_symbol();
        assert!(matches!(result, Symbol::BinaryOperator('*')));

        result = calc.next_symbol();
        assert!(matches!(result, Symbol::OpenParenthesis));

        result = calc.next_symbol();
        assert!(matches!(result, Symbol::Number(3)));

        result = calc.next_symbol();
        assert!(matches!(result, Symbol::BinaryOperator('+')));

        result = calc.next_symbol();
        assert!(matches!(result, Symbol::Number(2)));

        result = calc.next_symbol();
        assert!(matches!(result, Symbol::ClosedParenthesis));

        result = calc.next_symbol();
        assert!(matches!(result, Symbol::End));
    }

}

fn main() {
    println!("Hello, world!");
}