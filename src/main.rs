use std::io::Write;

mod ast;
mod lexer;
mod types;
//mod parser;

#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    #[clap()]
    I,
}

fn main() -> std::io::Result<()> {
    let args = {
        use clap::Parser;
        Args::parse()
    };

    match args.command {
        Commands::I => interpret(),
    }
}

fn interpret() -> std::io::Result<()> {
    println!("interpret");
    let mut line = String::new();
    print!("> ");
    loop {
        std::io::stdout().flush()?;
        line.clear();
        std::io::stdin().read_line(&mut line)?;
        if line.eq("exit\n") {
            break;
        }
        let mut input = lexer::Lexer::new(line.chars(), None);
        print!("Lexer Input: \n\t {line}\n");
        print!("Lexer output: \n\t");
        while let Ok(tok) = input.lex() {
            print!("{}\n\t", tok);
        }
        print!("\n> ");
    }
    //let lexer = lexer::Lexer::new(line.chars(), None);
    //let stmt  = parser::ValueParser::parse(lexer);
    Ok(())
}

fn check() -> std::io::Result<()> {
    unimplemented!()
}

fn compile() -> std::io::Result<()> {
    unimplemented!()
}
