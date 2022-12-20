use std::io::stdin;
use std::ops::Add;
use crate::parser::{Parser, ParserError};
use clap::Parser as Clapper;

mod lexer;
mod parser;

#[derive(Clapper, Debug)]
#[clap(author = "Stephan Smola", version = "0.1", about = "Unnecessary expression evaluator. It's one of many but this is mine.", long_about = None)]
struct Args {
    /// REPL mode
    #[clap(short = 'r', long, action)]
    repl: bool,

    /// Show only input
    #[clap(short, long, action)]
    input: bool,

    /// Show as equation
    #[clap(short = 'q', long, action)]
    equation: bool,

    expression: Vec<String>,

}


fn repl(firstExpression: String) {
    loop {
        let mut i = String::new();
        match stdin().read_line(&mut i) {
            Err(_) => return,

            Ok(_) => {
                if i.trim().len() == 0 {
                    continue
                }
                let r = Parser::eval(i.as_str());
                match r {
                    Err(e) => {
                        match e {
                            ParserError::Error(e) => println!("ERROR: {}", e)
                        }
                    },

                    Ok(v) => {
                        println!("{}", v);
                    }
                }
            }
        }


    }


}


fn main() {
    let args = Args::parse();

    let mut expression = String::from("");
    let mut sep = "";

    for e in args.expression {
        expression = expression.add(sep);
        expression = expression.add(e.as_str());
        sep = " ";
    }

    if args.repl {
        repl(expression);
        return;
    }


    if args.input {
        println!("{}", expression);
    } else {
        let result = Parser::eval(expression.as_str());

        match result {
            Err(e) => {
                match e {
                    ParserError::Error(e) => {
                        println!("ERROR: {e}");
                    }
                }
            },

            Ok(v) => {
                    if args.equation {
                        println!("{} = {}", expression, v)
                    } else {
                        println!("{}", v)
                    }

            }
        }
    }
}