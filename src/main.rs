use std::env;
use std::ops::Add;
use crate::parser::Parser;
use clap::Parser as Clapper;
mod lexer;
mod parser;

#[derive(Clapper, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Expression to evaluate
    #[arg(short, long)]
    expression: String,

    /// Show only input
    #[arg(short, long, action)]
    input: bool,

    /// Show as equation
    #[arg(short= 'q', long, action)]
    equation: bool,


}


fn main() {

    let args = Args::parse();

    if args.input {
        println!("{}", args.expression);
    } else {
        let result = Parser::eval(args.expression.as_str());

        if args.equation {
            println!("{} = {}", args.expression, result)
        } else {
            println!("{}", result)
        }
    }
}