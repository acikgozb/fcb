use std::process::ExitCode;

mod cli;

fn main() -> ExitCode {
    match cli::run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(fcb_err) => {
            eprintln!("{fcb_err}");
            ExitCode::FAILURE
        }
    }
}
