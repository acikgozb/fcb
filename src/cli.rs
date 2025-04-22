use std::io::{self, IsTerminal, Read};

use clap::Parser;

const PROGRAM: &str = "fcb";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Add LANG for syntax highlighting.
    #[arg(short, long)]
    lang: Option<String>,

    /// Set CODE as the block content.
    code: Option<String>,
}

pub fn run() -> Result<(), io::Error> {
    let cli = Cli::parse();
    Ok(())
}

