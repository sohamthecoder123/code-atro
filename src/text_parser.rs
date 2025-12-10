pub fn text_to_code(text: &str) -> Vec<String> {
    let text_vec: Vec<&str> = text.split_whitespace().collect();

    let mut guess_vec: Vec<String> = Vec::new();

    for j in text_vec{
        let code: String = j.to_string();
        guess_vec.push(code);
    }

    return guess_vec;
}

 pub fn text_to_int(text: &str) -> isize {
    let num: isize = match text.parse::<isize>() {
        Ok(n) => n,
        Err(e) => {
            println!("Error: {}", e);
            return -1;
        }
    };
    return num;
}

pub fn text_to_u_int(text: &str) -> usize {
    let num: usize = match text.parse::<usize>() {
        Ok(n) => n,
        Err(e) => {
            println!("Error: {}", e);
            return 0;
        }
    };
    //let num: usize = text.parse().expect("Error");
    return num;
}
