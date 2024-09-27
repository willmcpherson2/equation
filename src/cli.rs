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
}
