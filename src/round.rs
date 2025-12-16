use crate::code_guess::ResultCode;
use crate::random_code_generator;
use crate::random_code_generator::generate_alphabet;
use crate::text_parser;
use crate::user_input;
use crate::code_guess;


pub fn round(overall_alphabet_string: &str, alphabet_length: usize, code_length: usize, max_attempts: usize) -> bool {
    let overall_alphabet: Vec<String> = text_parser::text_to_code(overall_alphabet_string);
    let alphabet: Vec<String> = generate_alphabet(&overall_alphabet, alphabet_length);
    let mut score: f64 = 0.0;

    println!("The Alphabet is:");
    for i in &alphabet {
        print!("{} ", i);
    }
    println!("");
    println!("The Length of the Code is: {}", code_length);
    println!("The Maximum Number of Attempts is: {}", max_attempts);

    let generated_code: Vec<String> = random_code_generator::generate_code(&alphabet, code_length);
    println!("----------");
    for attempts in 1..=max_attempts {
        println!("Current Attempt: {}", attempts);
        println!("Attempts Left: {}", max_attempts - attempts + 1);
        println!("----------");
        println!("Enter your Guess below: ");
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

        let mut foo: bool = true;

        println!("The Result is: ");
        for _i in &result {
            match _i {
                ResultCode::InPlace => {
                    println!("In Place!");
                    score += 1.0;
                }

                ResultCode::OutOfPlace => {
                    println!("Out Of Place!");
                    foo = false;
                    score += 0.0;
                }

                ResultCode::NotInCode => {
                    println!("Not In Code!");
                    foo = false;
                    score -= 0.5;
                }
                
                ResultCode::SizeError => {
                    println!("Size Error!!!");
                    foo = false;
                }
            }
        }

        println!("----------");
        println!("The Score is: {}", score);
        println!("----------");

        if foo {
            return true;
        }
    }

    return false;

}

/*
pub fn round(overall_alphabet_string: &str, alphabet_length: usize, code_length: usize, max_attempts: usize){
    
}
*/

