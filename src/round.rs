use crate::code_guess::ResultCode;
use crate::random_code_generator;
use crate::text_parser;
use crate::user_input;
use crate::code_guess;

pub fn round(alphabet_string: &str, length: usize) -> Vec<ResultCode>{
    let alphabet: Vec<String> = text_parser::text_to_code(alphabet_string);
    let mut score: f64 = 0.0;

    println!("The Alphabet is: {}", alphabet_string);
    println!("The Length of the Code is: {}", length);
    println!("----------");
    println!("Enter your Guess below: ");

    let generated_code: Vec<String> = random_code_generator::generate_code(&alphabet, length);

    let input: String = user_input::get_user_input_trimmed("");
    let guessed_code: Vec<String> = text_parser::text_to_code(&input);

    println!("----------");
    println!("The Code is: ");

    for i in &generated_code {
        print!("{} ", i);
    }

    let result: Vec<ResultCode> = code_guess::guess_code_check(&generated_code, &guessed_code);

    println!("");
    println!("----------");

    println!("The Result is: ");
    for _i in &result {
        match _i {
            ResultCode::InPlace => {
                println!("In Place!");
                score += 1.0;
            }

            ResultCode::OutOfPlace => {
                println!("Out Of Place!");
                score += 0.0;
            }

            ResultCode::NotInCode => {
                println!("Not In Code!");
                score -= 0.5;
            }
            
            ResultCode::SizeError => {
                println!("Size Error!!!");
            }
        }
    }

    println!("----------");
    println!("The Score is: {}", score);
    println!("----------");

    return result;

}
