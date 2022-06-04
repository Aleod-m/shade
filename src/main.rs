mod lexer;
mod expr;
mod module;
mod types;

fn main() {
    let input = "main: 1 + 1";
    let mut lex = lexer::Lexer::new(input.chars(), None);
    for tok in lex {
        println!("{}", tok);
    }
}
