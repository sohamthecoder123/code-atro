use crate::code_guess::ResultCode;
use crate::random_code_generator;
use crate::random_code_generator::generate_alphabet;
use crate::text_parser;
use crate::user_input;
use crate::code_guess;
use crate::separating_line;
use crate::jochar;

pub fn round(overall_alphabet_string: &str, alphabet_length: usize, code_length: usize, max_attempts: usize, wealth: &mut f64, jochars_in_play: &Vec<usize>) -> bool {
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
    separating_line();
    for attempts in 1..=max_attempts {
        println!("Current Attempt: {}", attempts);
        println!("Attempts Left: {}", max_attempts - attempts + 1);
        separating_line();
        println!("Enter your Guess below: ");
        let input: String = user_input::get_user_input_trimmed("");
        let guessed_code: Vec<String> = text_parser::text_to_code(&input);

        
        separating_line();
        println!("The Code is: ");

        for i in &generated_code {
            print!("{} ", i);
        }

        let result: Vec<ResultCode> = code_guess::guess_code_check(&generated_code, &guessed_code);

        println!("");
        separating_line();

        let mut is_round_defeated: bool = true;

        println!("The Result is: ");
        for _i in &result {
            match _i {
                ResultCode::InPlace => {
                    println!("In Place!");
                    score += 1.0;
                    println!("Score +1");
                }

                ResultCode::OutOfPlace => {
                    println!("Out Of Place!");
                    is_round_defeated = false;
                    score += 0.0;
                    println!("Score +0");
                }

                ResultCode::NotInCode => {
                    println!("Not In Code!");
                    is_round_defeated = false;
                    score -= 0.5;
                    println!("Score -0.5");
                }
                
                ResultCode::SizeError => {
                    println!("Size Error!!!");
                    is_round_defeated = false;
                }
            }
        }

        for _ in 1..=jochars_in_play[1] {
            score += 4.0;
            println!("Advanced JoChar found.");
            println!("Score +4");
        }


        separating_line();
        println!("The Score is: {}", score);
        separating_line();

        if is_round_defeated {
            for _ in 1..=jochars_in_play[0] {
                score += 4.0;
                println!("Regular JoChar found.");
                println!("Score +4");
            }

            *wealth += score;

            return true;
        }
    }

    return false;

}

