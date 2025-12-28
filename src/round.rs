use std::collections::HashMap;

use crate::code_guess::ResultCode;
use crate::random_code_generator;
use crate::text_parser;
use crate::user_input;
use crate::code_guess;
use crate::separating_line;  

pub fn round(overall_alphabet_string: &str, alphabet_length: usize, code_length: usize, max_attempts: usize, wealth: &mut f64, jochars_in_play: &Vec<usize>) -> bool {
    let vowels: Vec<String> = vec![
        "a".to_string(),
        "e".to_string(),
        "i".to_string(),
        "o".to_string(),
        "u".to_string(),
    ];

    let consonants: Vec<String> = vec![
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
        "f".to_string(),
        "g".to_string(),
        "h".to_string(),
        "j".to_string(),
        "k".to_string(),
        "l".to_string(),
        "m".to_string(),
        "n".to_string(),
        "p".to_string(),
        "q".to_string(),
        "r".to_string(),
        "s".to_string(),
        "t".to_string(),
        "v".to_string(),
        "w".to_string(),
        "x".to_string(),
        "y".to_string(),
        "z".to_string(),
    ];

    let numbers: Vec<String> = vec![
        "0".to_string(),
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
        "6".to_string(),
        "7".to_string(),
        "8".to_string(),
        "9".to_string(),
    ];
    
    let overall_alphabet: Vec<String> = text_parser::text_to_code(overall_alphabet_string);
    let alphabet: Vec<String> = random_code_generator::generate_alphabet(&overall_alphabet, alphabet_length);
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

    if jochars_in_play[5] > 0 {
        let revealed_characters: HashMap<usize, &String> = random_code_generator::reveal_characters_position(&generated_code, 2);
        println!("Even Funnier JoChar Found!");
        
        for (key, value) in &revealed_characters{
            println!("{} is in the Code at Position = {}", value, key);
        }
    }    

    else if jochars_in_play[4] > 0 {
        let revealed_characters: HashMap<usize, &String> = random_code_generator::reveal_characters_position(&generated_code, 1);
        println!("Even Funnier JoChar Found!");
        
        for (key, value) in &revealed_characters{
            println!("{} is in the Code at Position = {}", value, key);
        }
    }

    else if jochars_in_play[3] > 0 {
        let revealed_characters: Vec<String> = random_code_generator::reveal_characters_random(&generated_code, 2); 
        println!("Funnier JoChar Found!");
        println!("{} and {} are parts of the Code!", revealed_characters[0], revealed_characters[1]);
    }

    else if jochars_in_play[2] > 0 {
        let revealed_characters: Vec<String> = random_code_generator::reveal_characters_random(&generated_code, 1); 
        println!("Funny JoChar Found!");
        println!("{} is a part of the Code!", revealed_characters[0]);
    }


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
        let mut profit: f64 = 0.0;
        let mut loss: f64 = 0.0;

        println!("The Result is: ");
        let mut index: usize = 0;
        for _i in &result {
            match _i {
                ResultCode::InPlace => {
                    println!("In Place!");
                    let mut gain: f64 = 1.0;

                    if vowels.contains(&generated_code[index]){
                        for _ in 1..=jochars_in_play[8] {
                            gain *= 2.0;
                        }   
                    } 

                    else if consonants.contains(&generated_code[index]){
                        for _ in 1..=jochars_in_play[9] {
                            gain *= 2.0;
                        }  
                    }

                    else if numbers.contains(&generated_code[index]){
                        for _ in 1..=jochars_in_play[10] {
                            gain *= 2.0;
                        }
                    }

                    profit += gain;
                    println!("Profit +{}", gain);
                }

                ResultCode::OutOfPlace => {
                    println!("Out Of Place!");
                    is_round_defeated = false;
                    profit += 0.5;
                    println!("Profit +0.5");
                }

                ResultCode::NotInCode => {
                    println!("Not In Code!");
                    is_round_defeated = false;
                    
                    for _ in 1..=jochars_in_play[13] {
                        loss += 1.0;
                        println!("Negative Marking found.");
                        println!("Loss +1.0");
                    }                    
                }
                
                ResultCode::SizeError => {
                    println!("Size Error!!!");
                    is_round_defeated = false;
                }
            }

            index += 1;
        }

        for _ in 1..=jochars_in_play[7] {
            println!("Double the Stakes found!");
            println!("Profit x2");
            profit *= 2.0;
            println!("Loss x2");
            loss *= 2.0;
        }

        println!("Profit: {}", profit);
        println!("Score increased by Profit.");
        score += profit;

        println!("Loss: {}", loss);
        println!("Score decreased by Loss");
        score -= loss;

        for _ in 1..=jochars_in_play[1] {
            score += 4.0;
            println!("Advanced JoChar found.");
            println!("Score +4");
        }

        if jochars_in_play[11] > 0 {
            for i in jochars_in_play {
                for _ in 1..=jochars_in_play[*i] {
                    println!("The Collecter found.");
                    println!("Score +0.25");
                    score += 0.25;
                }
            }
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

