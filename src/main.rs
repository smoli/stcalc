use std::ops::Add;
use crate::parser::Parser;
use clap::Parser as Clapper;
mod lexer;
mod parser;

#[derive(Clapper, Debug)]
#[clap(author="Stephan Smola", version="0.1", about ="Unnecessary expression evaluator. It's one of many but this is mine.", long_about = None)]
struct Args {
    /// Show only input
    #[clap(short, long, action)]
    input: bool,

    /// Show as equation
    #[clap(short= 'q', long, action)]
    equation: bool,

    expression: Vec<String>,

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

    if args.input {
        println!("{}", expression);
    } else {
        let result = Parser::eval(expression.as_str());

        if args.equation {
            println!("{} = {}", expression, result)
        } else {
            println!("{}", result)
        }
    }
}