use std::env;
mod tokenizer;
mod value;
mod parse;
mod eval_env;
mod special_forms;
mod builtins;
mod error;
mod reader_interact;
mod reader_file;
mod command_line;

fn main() {
    let config = command_line::Config::build(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    if let Err(e) = command_line::run(config) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}
