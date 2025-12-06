pub fn text_to_code(text: &str) -> Vec<String> {
    let text_vec: Vec<&str> = text.split_whitespace().collect();

    let mut guess_vec: Vec<String> = Vec::new();

    for j in text_vec{
        let code: String = j.to_string();
        guess_vec.push(code);
    }

    return guess_vec;
}
