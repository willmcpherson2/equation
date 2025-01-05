use clap::Parser;

/// The Equation Calculus
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Expression or file containing expression
    pub target: String,

    /// Interpret target as expression rather than file
    #[arg(short, long)]
    pub expression: bool,

    /// Print the input text
    #[arg(long)]
    pub trace_input: bool,

    /// Pretty print the parsed program
    #[arg(long)]
    pub trace_parse: bool,
}
