#![warn(missing_docs)]
//! Generates fenced code blocks.

/// `render` is used to create a String that represents a code block.
///
/// `lang`: It is used to decorate the code block to enable syntax highlighting.
///
/// `code`: It is used as the code block's content.
///
/// # Details
///
/// By default, `render` uses empty `&str` for arguments that are `None`.
///
/// When `code` is `Some`, then `render` adds a trailing newline to it.
/// Therefore, users do not need to add a trailing newline to their `code`.
///
/// # Examples
///
/// With `lang` and `code`:
///
/// ```
/// let mut code = Some("println!(\"Hello world!\");".to_string());
/// let lang = Some("rust".to_string());
///
/// let code_block = fcb::render(&lang, &mut code);
/// println!("{}", code_block);
/// // Output:
/// // ```rust
/// // println!("Hello world!");
/// // ```
/// ```
///
/// With `lang` only:
///
/// ```
/// let lang = Some("rust".to_string());
///
/// let code_block = fcb::render(&lang, &mut None);
/// println!("{}", code_block);
/// // Output:
/// // ```rust
/// // ```
/// ```
///
/// With `code` only:
///
/// ```
/// let mut code = Some("println!(\"Hello world!\");".to_string());
///
/// let code_block = fcb::render(&None, &mut code);
/// println!("{}", code_block);
/// // Output:
/// // ```
/// // println!("Hello world!");
/// // ```
/// ```
///
/// With nothing provided:
///
/// ```
/// let code_block = fcb::render(&None, &mut None);
/// println!("{}", code_block);
/// // Output:
/// // ```
/// // ```
/// ```
pub fn render(lang: &Option<String>, code: &mut Option<String>) -> String {
    let empty = "";

    let lang = lang.as_ref().map(|val| val.as_str()).unwrap_or(empty);
    let code = code
        .as_mut()
        .map(|val| {
            val.push('\n');
            val.as_str()
        })
        .unwrap_or(empty);

    format!("```{}\n{}```", lang, code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_render_without_any_args() {
        let lang: Option<String> = None;
        let mut code: Option<String> = None;

        let result = render(&lang, &mut code);

        assert_eq!(result, "```\n```".to_string());
    }

    #[test]
    fn should_render_with_lang() {
        let test_lang = "rust";
        let lang: Option<String> = Some(test_lang.to_string());
        let mut code: Option<String> = None;

        let result = render(&lang, &mut code);
        let expected = format!("```{}\n```", test_lang);

        assert_eq!(result, expected);
    }

    #[test]
    fn should_render_code_with_newline() {
        let test_code = "println!(\"Hello world!\")";
        let mut code: Option<String> = Some(test_code.to_string());

        let result = render(&None, &mut code);
        let expected = format!("```\n{}\n```", test_code);

        assert_eq!(result, expected);
    }

    #[test]
    fn should_render_with_lang_and_code() {
        let test_lang = "rust";
        let test_code = "println!(\"Hello world!\")";

        let lang: Option<String> = Some(test_lang.to_string());
        let mut code: Option<String> = Some(test_code.to_string());

        let result = render(&lang, &mut code);
        let expected = format!("```{}\n{}\n```", test_lang, test_code);

        assert_eq!(result, expected);
    }
}
