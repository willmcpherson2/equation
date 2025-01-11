use equation::{compile, eval, parse_program, show_program, show_stack, Cli};

use clap::Parser;
use std::fs;

fn main() {
    let cli = Cli::parse();

    let text = if cli.expression {
        cli.target
    } else {
        match fs::read_to_string(cli.target) {
            Ok(text) => text,
            Err(e) => return eprintln!("Error reading file: {}", e),
        }
    };
    if cli.trace_input {
        println!("input:\n{}\n", text);
    }

    let prog = match parse_program(&text) {
        Ok(prog) => prog,
        Err(e) => {
            println!("parse error: {}", e);
            return;
        }
    };
    if cli.trace_parse {
        println!("parsed:\n{}\n", show_program(&prog));
    }

    let mut state = match compile(&prog) {
        Ok(state) => state,
        Err(e) => {
            println!("compile error: {}", e);
            return;
        }
    };
    eval(&mut state);
    println!("{}", show_stack(&state.names, &state.stack));
}
