use std::io::Write;
use shade::*;


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
    #[clap()]
    C,
    #[clap()]
    B,

}

fn main() -> std::io::Result<()> {
    let args = {
        use clap::Parser;
        Args::parse()
    };

    match args.command {
        Commands::I => interpret(),
        Commands::C => check(),
        Commands::B => build(),
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
    Ok(())
}

fn check() -> std::io::Result<()> {
    unimplemented!()
}

fn build() -> std::io::Result<()> {
    unimplemented!()
}
