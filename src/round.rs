/// This file has a singular function which effectively represents a round, ie. it handles all the stuff that happens during any specific round.


use std::collections::HashMap; //The Funny JoChars, which reveal the position too, use HashMaps for their functioning 
use crate::code_guess::ResultCode; 
//the public enum from code_guess. The return of the code_guess's function are is as this enum.
use crate::random_code_generator; //to generate a random code every round
use crate::text_parser; //to parse the code, etc.
use crate::user_input; //to get user input
use crate::code_guess; //to verify the guesses
use crate::separating_line; //The Separating Line of Death (tm)


//the eponymous round function
pub fn round(overall_alphabet_string: &str, alphabet_length: usize, code_length: usize, max_attempts: usize, wealth: &mut f64, debuff_count: usize, jochars_in_play: &Vec<usize>) -> bool {
    //list of vowels
    let vowels: Vec<String> = vec![
        "a".to_string(),
        "e".to_string(),
        "i".to_string(),
        "o".to_string(),
        "u".to_string(),
    ];

    //list of alphabets
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

    //list of numbers
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
    
    let overall_alphabet: Vec<String> = text_parser::text_to_code(overall_alphabet_string); //creates the overall  alphabet
    let alphabet: Vec<String> = random_code_generator::generate_alphabet(&overall_alphabet, alphabet_length); //generates a random alphabet of a given length
    let mut score: f64 = 0.0; //stores the score

    //show stuff to the player that they need
    println!("The Alphabet is:");
    for i in &alphabet {
        print!("{} ", i);
    }
    println!("");
    println!("The Length of the Code is: {}", code_length);
    println!("The Maximum Number of Attempts is: {}", max_attempts);

    //generate the random code
    let generated_code: Vec<String> = random_code_generator::generate_code(&alphabet, code_length);
    separating_line();

    //Funny JoChars
    //Funniest JoChar
    if jochars_in_play[5] > 0 {
        let revealed_characters: HashMap<usize, &String> = random_code_generator::reveal_characters_position(&generated_code, 2);
        println!("Funniest JoChar Found!");
        
        for (key, value) in &revealed_characters{
            println!("{} is in the Code at Position = {}", value, key);
        }
    } 
    
    //Even Funnier JoChar 
    else if jochars_in_play[4] > 0 {
        let revealed_characters: HashMap<usize, &String> = random_code_generator::reveal_characters_position(&generated_code, 1);
        println!("Even Funnier JoChar Found!");
        
        for (key, value) in &revealed_characters{
            println!("{} is in the Code at Position = {}", value, key);
        }
    }

    //Funnier JoChar
    else if jochars_in_play[3] > 0 {
        let revealed_characters: Vec<String> = random_code_generator::reveal_characters_random(&generated_code, 2); 
        println!("Funnier JoChar Found!");
        println!("{} and {} are parts of the Code!", revealed_characters[0], revealed_characters[1]);
    }

    //Funny JoChar
    else if jochars_in_play[2] > 0 {
        let revealed_characters: Vec<String> = random_code_generator::reveal_characters_random(&generated_code, 1); 
        println!("Funny JoChar Found!");
        println!("{} is a part of the Code!", revealed_characters[0]);
    }


    //Attempts
    for attempts in 1..=max_attempts {
        //Display info regarding the attempts left
        println!("Current Attempt: {}", attempts);
        println!("Attempts Left: {}", max_attempts - attempts + 1);
        separating_line();

        //guessing
        println!("Enter your Guess below: ");
        let input: String = user_input::get_user_input_trimmed(""); //get user's guess
        let guessed_code: Vec<String> = text_parser::text_to_code(&input); //convert the guessed string to the code format
        
        separating_line();

        //get the result of the code
        let result: Vec<ResultCode> = code_guess::guess_code_check(&generated_code, &guessed_code);

        println!("");
        separating_line();

        //the code that follows handles the round based on the result
        
        let mut is_round_defeated: bool = true; //is the round defeated?
        let mut profit: f64 = 0.0; //the profit gained by the round
        let mut loss: f64 = 0.0; //the loss sustained due to the round

        //show the result
        println!("The Result is: ");
        let mut index: usize = 0; //stores the index of the character which we are "scrutinizing" 
        for _i in &result {
            match _i {
                //the character is in place
                ResultCode::InPlace => {
                    println!("In Place!");
                    let mut gain: f64 = 1.0; //increment gain by 1

                    //VowelPhile
                    if vowels.contains(&generated_code[index]){
                        for _ in 1..=jochars_in_play[8] {
                            println!("VowelPhile found!");
                            println!("Vowel Detected! Gain Doubled.");
                            gain *= 2.0;
                        }   
                    } 

                    //ConsonantPhile
                    else if consonants.contains(&generated_code[index]){
                        for _ in 1..=jochars_in_play[9] {
                            println!("ConsonantPhile found!");
                            println!("Consonant Detected! Gain Doubled.");
                            gain *= 2.0;
                        }  
                    }

                    //NumberPhile
                    else if numbers.contains(&generated_code[index]){
                        for _ in 1..=jochars_in_play[10] {
                            println!("NumberPhile found!");
                            println!("Number Detected! Gain Doubled.");
                            gain *= 2.0;
                        }
                    }

                    profit += gain; //increase profit by gain
                    println!("Profit +{}", gain); //show this to the player
                }

                //if the character is in the code but in a different place
                ResultCode::OutOfPlace => {
                    println!("Out Of Place!");
                    if is_round_defeated {is_round_defeated = false;} //set is_round_defeated to false. if a character is out of place, you lost.
                    profit += 0.5; //increase profit by +0.5
                    println!("Profit +0.5");

                    //Strict Negative Marking JoChar
                    for _ in 1..=jochars_in_play[14] {
                        loss += 0.5;
                        println!("Strict Negative Marking found!");
                        println!("Loss +0.5.");
                    } 
                }

                //if the character is not in the code at all
                ResultCode::NotInCode => {
                    println!("Not In Code!");
                    if is_round_defeated {is_round_defeated = false;} //set is_round_defeated to false. Same reason as above
                    
                    //Negative Marking JoChar
                    for _ in 1..=jochars_in_play[13] {
                        loss += 1.0;
                        println!("Negative Marking found!");
                        println!("Loss +1.0.");
                    } 

                    //Strict Negative Marking JoChar
                    for _ in 1..=jochars_in_play[14] {
                        loss += 1.0;
                        println!("Strict Negative Marking found!");
                        println!("Loss +1.0.");
                    }                   
                }
                
                //if there is a size error
                ResultCode::SizeError => {
                    println!("Size Error!!!");
                    is_round_defeated = false; //ofc you lost.
                }
            }

            index += 1;
        }

        //Double the Stakes JoChar
        for _ in 1..=jochars_in_play[7] {
            println!("Double the Stakes found!");
            println!("Profit x2"); //double the profit
            profit *= 2.0;
            println!("Loss x2"); //double the loss
            loss *= 2.0;
        }

        //Display profit and loss, and update the score
        println!("Profit: {}", profit);
        println!("Score increased by Profit.");
        score += profit;

        println!("Loss: {}", loss);
        println!("Score decreased by Loss");
        score -= loss;

        //Advanced JoChar
        for _ in 1..=jochars_in_play[1] {
            score += 4.0;
            println!("Advanced JoChar found.");
            println!("Score +4");
        }

        //The Collector JoChar
        if jochars_in_play[11] > 0 {
            for i in jochars_in_play {
                for _ in 1..=jochars_in_play[*i] {
                    println!("The Collecter found.");
                    println!("Score +0.25");
                    score += 0.25;
                }
            }
        }

        //The Debuff Collector JoChar
        if jochars_in_play[12] > 0 {
            println!("The Debuff Collector Found.");
            println!("Number of Debuff JoChars Found: {}", debuff_count);
            println!("Score +(1x{})", debuff_count);
            for _i in 1..= debuff_count {
                score += 1.0;
            }
        }


        separating_line();
        println!("The Score is: {}", score);
        separating_line();

        //Regular JoChar
        if is_round_defeated {
            for _ in 1..=jochars_in_play[0] {
                score += 4.0;
                println!("Regular JoChar found.");
                println!("Score +4");
            }

            *wealth += score;

            //show the code
            print!("The Code was: ");
            for i in &generated_code {
                print!("{} ", i);
            }
            println!("");

            return true;
        }
    }

    //show the code
    print!("The Code was: ");
    for i in &generated_code {
        print!("{} ", i);
    }
    println!("");

    return false;
}

