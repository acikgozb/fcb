use std::io::{self, IsTerminal, Read, Write};

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
    CannotWriteStdout(io::Error),
}

impl std::fmt::Display for FcbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FcbError::CannotReadStdin(err) => {
                write!(f, "{PROGRAM}: failed to read stdin: {}", err)
            }
            FcbError::CannotWriteStdout(err) => {
                write!(f, "{PROGRAM}: failed to write stdout: {}", err)
            }
        }
    }
}

impl std::error::Error for FcbError {}

pub fn run() -> Result<(), FcbError> {
    let cli = Cli::parse();

    let mut code = match cli.code {
        Some(code) => Ok(Some(code)),
        None => read_from_stdin(),
    }?;

    let code_block = fcb::render(&cli.lang, &mut code);
    write_to_stdout(code_block)
}

fn read_from_stdin() -> Result<Option<String>, FcbError> {
    let mut stdin = io::stdin();

    if stdin.is_terminal() {
        return Ok(None);
    }

    let mut buf = String::new();
    let _ = stdin
        .read_to_string(&mut buf)
        .map_err(FcbError::CannotReadStdin)?;

    if !buf.is_empty() {
        Ok(Some(buf))
    } else {
        Ok(None)
    }
}

fn write_to_stdout(code_block: String) -> Result<(), FcbError> {
    let mut stdout = io::stdout();
    stdout
        .write_all(code_block.as_bytes())
        .map_err(FcbError::CannotWriteStdout)
}
