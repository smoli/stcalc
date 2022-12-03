
enum Symbol {
    Number(u32),
    BinaryOperator(char),
    OpenParenthesis,
    ClosedParenthesis,
    End,
    Unknown,
}

struct Calc {
    input: String,
    scanner: usize,
    symbol: Symbol,
}

impl Calc {
    fn new(input: String) -> Calc {
        Calc { input, scanner: 0, symbol: Symbol::Unknown }
    }

    fn take_next(&mut self) -> Option<char> {
        // println!("Scanner {}({})", self.scanner, self.input.len());
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

        println!("Result {}", result);
        Symbol::Number(result.parse().unwrap())
    }

    fn next_symbol(&mut self) -> Symbol {
        if self.exhausted() {
            return Symbol::Unknown;
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
                        => return Symbol::BinaryOperator(c),

                        '(' => return Symbol::OpenParenthesis,

                        ')' => return Symbol::ClosedParenthesis,

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
        let mut calc = Calc::new(String::from("12"));
        let result = calc.next_symbol();

        assert!(matches!(result, Symbol::Number(12)));
    }

    #[test]
    fn ignores_leading_white_space() {
        let mut calc = Calc::new(String::from("   12   "));
        let result = calc.next_symbol();

        assert!(matches!(result, Symbol::Number(12)));
    }
}

fn main() {
    println!("Hello, world!");
}