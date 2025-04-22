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
