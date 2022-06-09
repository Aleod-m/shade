mod lexer;
mod parser;
mod ast;

fn main() {
    let input = "2 + 3 * 5";
    let mut lex = lexer::Lexer::new(input.chars(), None);
    
    for tok in lex {
        println!("{}", tok);
    }
}
