
use clap::{App, Arg};
use rustyline::Editor;
use rustyline::error::ReadlineError;

mod command;
mod session;

use command::Shell;

const CLI_NAME: &'static str = env!("CARGO_PKG_NAME");
const CLI_VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = App::new(CLI_NAME)
        .version(CLI_VERSION)
        .author("Alexander Reece <alreece45@gmail.com>")
        .about("Tokenizes stuff")
        .arg(Arg::with_name("query").takes_value(true).index(1))
        .get_matches();

    if let Some(query) = args.value_of("query") {
        println!("{:?}", query);
    } else {
        main_interactive();
    }
}

fn main_interactive() {
    let prompt_string = format!("{}> ", CLI_NAME);
    let mut editor = Editor::<()>::new();
    let mut shell = Shell::new();
    loop {
        let prompt = prompt_string.as_str();
        match editor.readline(prompt) {
            Ok(command) => shell.process_command(&command),
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                std::process::exit(0);
            },
            Err(ReadlineError::Eof) => {
                println!("^D");
                std::process::exit(0);
            },
            Err(err) => {
                println!("Error: {:?}", err);
                std::process::exit(0);
            }
        }
    }
}
