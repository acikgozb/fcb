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
