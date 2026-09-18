use std::path::PathBuf;

use clap::{Parser, Subcommand};
use envyc::compiler::Compiler;

#[derive(Parser)]
#[command(name = "envious", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Lex { path: PathBuf }
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Lex { path } => {
            let mut compiler = Compiler::default();
            let contents = std::fs::read_to_string(path)?;
            let id = compiler.lex(contents);
            println!("{}", compiler.tokens(&id));
        }
    }

    Ok(())
}
