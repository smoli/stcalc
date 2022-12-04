use lexer::Lexer;
mod lexer;
mod parser;


fn main() {

    let l = Lexer::new("23");

    for s in l {
        println!("{s}");
    }

}