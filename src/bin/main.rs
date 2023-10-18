//use shade::{lexer::Token, parser::def_parser, *};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap()]
    Interpret,
    #[clap()]
    Check,
    #[clap()]
    Build,
    #[clap()]
    Run,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Interpret => interpret(),
        Commands::Check => check(),
        Commands::Build => build(),
        Commands::Run => run(),
    }
}

fn interpret() -> std::io::Result<()> {
    unimplemented!();
//    println!("interpret");
//    let mut evaluator = Evaluator::new();
//    let mut line = String::new();
//    print!("> ");
//    loop {
//        std::io::stdout().flush()?;
//        line.clear();
//        std::io::stdin().read_line(&mut line)?;
//        if line.eq("exit\n") || line.eq("quit\n") {
//            break;
//        }
//        let input: Vec<Token> = lexer::Lexer::init(&line, None).collect();
//        match def_parser()(&input) {
//            Ok((_, stmt)) => {
//                dbg!(&stmt);
//                let eval = evaluator.eval_stmt(stmt);
//                println!("{eval}");
//            }
//            Err((_, err)) => println!("{err:#?}"),
//        };
//        print!("\n> ");
//    }
//    Ok(())
}

fn check() -> std::io::Result<()> {
    unimplemented!()
}

fn build() -> std::io::Result<()> {
    unimplemented!()
}
fn run() -> std::io::Result<()> {
    unimplemented!()
}
