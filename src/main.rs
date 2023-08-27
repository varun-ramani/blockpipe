mod command;
mod language;

use clap::{Parser, Subcommand};
use command::{parse::{parser_standalone, ParserArgs}, interpret::{InterpreterArgs, interpreter_standalone}};

#[derive(Subcommand, Debug)]
enum Command {
    Parse(ParserArgs),
    Interpret(InterpreterArgs)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "bruh")]
struct Arguments {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let cli = Arguments::parse();

    match &cli.command {
        Command::Parse(parser_args) => parser_standalone(parser_args),
        Command::Interpret(args) => interpreter_standalone(args)
    }
}
