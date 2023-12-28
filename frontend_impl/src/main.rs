use clap::{Parser, ValueEnum};
use std::{fs, process::{exit, Command}};
use language_impl::{self, interpret_from_string};

/// Processes files based on the given command
#[derive(Parser, Debug)]
#[clap(name = "blockpipe", version = "1.0", author = "Varun Ramani <varun.ramani@gmail.com>")]
struct BlockPipe {
    command: Commands,
    filename: String,
    parameters: Vec<String>
}

#[derive(Parser, Debug, Clone, ValueEnum)]
enum Commands {
    Lex,
    Parse,
    Interpret,
    InterpretExecute,
    Compile
}

fn main() {
    let opts: BlockPipe = BlockPipe::parse();
    let file_data = match fs::read_to_string(&opts.filename) {
        Ok(data) => data,
        Err(_) => {
            println!("Failed to read {}", opts.filename);
            exit(-1);
        }
    };

    match opts.command {
        Commands::Lex => {
            let result = language_impl::lex_from_string(&file_data);
            println!("{:?}", result);
        }, 
        Commands::Parse => {
            let result = language_impl::parse_from_string(&file_data);
            println!("{:?}", result);
        },
        Commands::Interpret => {
            let result = language_impl::interpret_from_string(&file_data, None, false);
            println!("{:?}", result);
        },
        Commands::InterpretExecute => {
            let result = language_impl::interpret_from_string(&file_data, Some(opts.parameters), true);
            println!("{:?}", result);
        },
        _ => {
            println!("unimplemented");
        }
    }
}
