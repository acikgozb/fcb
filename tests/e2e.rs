#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::{self, Write},
        process::{Command, Stdio},
        thread,
    };

    #[test]
    fn write_empty_lang_empty_code_block_to_stdout() -> io::Result<()> {
        let fcb_path = fs::canonicalize("./target/debug/fcb")?;

        let fcb_out = Command::new(fcb_path).output()?;
        let expected_stdout = "```\n```";

        assert!(fcb_out.status.success());
        assert_eq!(fcb_out.stdout, expected_stdout.as_bytes());

        Ok(())
    }

    #[test]
    fn write_empty_lang_nonempty_code_block_to_stdout() -> io::Result<()> {
        let fcb_path = fs::canonicalize("./target/debug/fcb")?;

        let code = "println!(\"Hello world!\");";

        let fcb_out = Command::new(fcb_path).arg(code).output()?;
        let expected_stdout = format!("```\n{}\n```", code);

        assert!(fcb_out.status.success());
        assert_eq!(fcb_out.stdout, expected_stdout.as_bytes());

        Ok(())
    }

    #[test]
    fn write_nonempty_lang_nonempty_code_block_to_stdout() -> io::Result<()> {
        let fcb_path = fs::canonicalize("./target/debug/fcb")?;

        let lang = "rust";
        let code = "println!(\"Hello world!\");";

        let fcb_out = Command::new(fcb_path)
            .args(["-l", lang])
            .arg(code)
            .output()?;
        let expected_stdout = format!("```{}\n{}\n```", lang, code);

        assert!(fcb_out.status.success());
        assert_eq!(fcb_out.stdout, expected_stdout.as_bytes());

        Ok(())
    }

    #[test]
    fn write_stdin_errors_to_stderr() -> io::Result<()> {
        let fcb_path = fs::canonicalize("./target/debug/fcb")?;

        let mut fcb_handle = Command::new(fcb_path)
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let mut stdin = fcb_handle.stdin.take().expect("failed to bind to stdin");
        thread::spawn(move || {
            // NOTE: Invalid UTF-8 is written to stdin to trigger
            // a read error inside the process.
            let code = b"\xF09F";

            stdin.write_all(code).expect("failed to write to stdin");
        });

        let fcb_out = fcb_handle.wait_with_output()?;

        assert!(!fcb_out.status.success());
        assert!(!fcb_out.stderr.is_empty());
        assert!(fcb_out.stdout.is_empty());

        Ok(())
    }

    #[test]
    fn write_stdout_errors_to_stderr() -> io::Result<()> {
        let fcb_path = fs::canonicalize("./target/debug/fcb")?;

        // SAFETY: The unsafe block below is intentional and safe to use.
        // The goal is to pass an invalid UTF-8 String to make the process
        // throw an error on stdout write, which expects a valid UTF-8 String.
        //
        // It is safe because the binding is not accessed on anywhere else,
        // and the provided value is static. Therefore, it is guaranteed to
        // create an invalid UTF-8 String (aka. no undefined behaviors).
        let code = unsafe { String::from_utf8_unchecked(b"\xF09F".to_vec()) };

        let fcb_out = Command::new(fcb_path)
            .arg(code)
            .stdout(Stdio::inherit())
            .output()?;

        assert!(!fcb_out.status.success());
        assert!(!fcb_out.stderr.is_empty());
        assert!(fcb_out.stdout.is_empty());

        Ok(())
    }
}
