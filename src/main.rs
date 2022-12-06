use std::env;
use std::ops::Add;
use crate::parser::Parser;

mod lexer;
mod parser;


fn main() {

    let args:Vec<String> = env::args().collect();
    let mut expr: String = String::from("");


    for (i, a) in args.into_iter().enumerate() {
        if i == 0 {
            continue;
        }

        expr = expr.add(" ");
        expr = expr.add(a.as_str());
    }

    println!("{}", Parser::eval(expr.as_str()));
    // println!("{} = {}", expr, Parser::eval(expr.as_str()));
    // println!("{}", expr);
}