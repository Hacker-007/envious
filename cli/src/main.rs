use std::path::PathBuf;

use clap::{Parser, Subcommand};
use envyc::{compiler::Compiler, evaluator::Evaluator};

#[derive(Parser)]
#[command(name = "envious", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Run { path: PathBuf },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Run { path } => {
            let mut compiler = Compiler::default();
            let contents = std::fs::read_to_string(path)?;
            let id = compiler.lex(contents);
            compiler.parse(&id);
            let evaluator = Evaluator::new(
                compiler.source(&id),
                compiler.tokens(&id),
                compiler.ast(&id),
            );

            match evaluator.evaluate() {
                Ok(value) => println!("{}", value),
                Err(_) => println!("got an unknown error"),
            }
        }
    }

    Ok(())
}
