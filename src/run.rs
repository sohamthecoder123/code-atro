use crate::code_guess::ResultCode;
use crate::random_code_generator;
use crate::text_parser;
use crate::user_input;
use crate::code_guess;

pub fn run(alphabet_string: &str, length: usize) -> Vec<ResultCode>{
    let alphabet: Vec<String> = text_parser::text_to_code(alphabet_string);

    let generated_code: Vec<String> = random_code_generator::generate_code(&alphabet, length);

    let input: String = user_input::get_user_input_trimmed("");
    let guessed_code: Vec<String> = text_parser::text_to_code(&input);

    let result: Vec<ResultCode> = code_guess::guess_code_check(&generated_code, &guessed_code);

    for i in &generated_code {
        print!("{}, ", i);
    }

    print!("\n");

    return result;

}
