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

#[derive(Debug)]
pub enum FcbError {
    CannotReadStdin(io::Error),
}

impl std::fmt::Display for FcbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FcbError::CannotReadStdin(err) => {
                write!(f, "{PROGRAM}: failed to read stdin: {}", err)
            }
        }
    }
}

impl std::error::Error for FcbError {}

pub fn run() -> Result<(), FcbError> {
    let cli = Cli::parse();
    let code = match cli.code {
        Some(code) => Ok(code),
        None => read_from_stdin().map_err(FcbError::CannotReadStdin),
    }?;

}

fn read_from_stdin() -> io::Result<String> {
    let mut stdin = io::stdin();

    if stdin.is_terminal() {
        return Ok("".to_string());
    }

    let mut buf = String::new();
    let _ = stdin.read_to_string(&mut buf)?;
    Ok(buf)
}

