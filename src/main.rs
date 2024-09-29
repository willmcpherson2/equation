use equation::{compile, eval, parse_program, show_program, show_state, Cli};

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

    println!("text:\n{}\n", text);

    let prog = match parse_program(&text) {
        Ok(prog) => prog,
        Err(e) => {
            println!("parse error: {}", e);
            return;
        }
    };
    println!("parsed:\n{}\n", show_program(&prog));

    let state = compile(&prog);
    println!("eval:");
    let state = eval(state);
    println!("{}", show_state(&state));
}
