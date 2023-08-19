mod language;
mod command;

use clap::{Parser, Subcommand};
use command::parse::{ParserArgs, parser_standalone};

#[derive(Subcommand, Debug)]
enum Command {
    Parse(ParserArgs)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "bruh")]
struct Arguments {
    #[command(subcommand)]
    command: Command
}

fn main() {
    let cli = Arguments::parse();

    match &cli.command {
        Command::Parse(parser_args) => parser_standalone(parser_args)
    }
}
