use std::io::Write;

mod lexer;
mod parser;
mod ast;
mod types;

#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    #[clap()]
    Interpret,
}


fn main() -> std::io::Result<()> {
    let args = {
        use clap::Parser;
        Args::parse()
    };

    match args.command {
        Commands::Interpret => interpret(),
    }
}

fn interpret() -> std::io::Result<()> {
    println!("interpret");
    let mut line = String::new(); 
    print!("> ");
    while line != "exit" {
        std::io::stdout().flush()?;
        line.clear();
        std::io::stdin().read_line(&mut line)?;
        let mut input = lexer::Lexer::new(line.chars(), None);
        print!("Lexer Input: \n\t {line}\n");
        print!("Lexer output: \n\t");
        while !input.is_empty() {
            print!("{}\n\t", input.lex());
        }
        print!("\n> ");
    }
    //let lexer = lexer::Lexer::new(line.chars(), None);
    //let stmt  = parser::ValueParser::parse(lexer);
    Ok(())
}
