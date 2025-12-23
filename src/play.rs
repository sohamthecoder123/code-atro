use crate::round;
use crate::separating_line;
use crate::user_input;
use crate::clear_terminal;
use crate::jochar;

pub fn play(){
    let available_jochars: Vec<jochar::JoChar> = jochar::initiate_jochars();
    let mut jochars_in_play: Vec<usize> = vec![0; available_jochars.len()];

    jochars_in_play[0] = 1;
    jochars_in_play[1] = 2;
    jochars_in_play[13] = 3;

    let mut number_of_debuffs: usize = 0;

    for i in 0..jochars_in_play.len(){
        if available_jochars[i].is_debuff {
            number_of_debuffs += jochars_in_play[i];
        }
    }

    let mut round_no: usize = 1;
    let mut current_length: usize = 2;

    let mut wealth:f64 = 0.0;

    let overall_alphabet: &str = "a b c d e f g h i j k l m n o p q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9";

    loop {
        separating_line();
        println!("Your Current Wealth is: {}", wealth);
        separating_line();

        println!("Available Jochars: ");
        if jochars_in_play.iter().all(|&x| x == 0) {
            println!("No JoChars in Play right now!!");
            separating_line();
        }

        else {
            for i in 0..jochars_in_play.len() {
                if jochars_in_play[i] != 0 {
                    jochar::show_jochar(&available_jochars[i]);
                    println!("Quantity: {}", jochars_in_play[i]);
                    separating_line();
                }
            }
        }

        println!("Number of Debuffed JoChars: {}", number_of_debuffs);



        println!("Round {}", round_no);
        separating_line();
        let has_won: bool = round::round(overall_alphabet, 2 * current_length, current_length, current_length + 1, &mut wealth, &jochars_in_play);
        
        if !has_won {
            break;
        }

        println!("Round {} has been Defeated", round_no);      
        separating_line();  
        round_no += 1;

        let two: usize = 2;

        current_length = two.pow((round_no / 2) as u32);
    }
    
    println!("You Lost!!!");
    println!("Enter anything to Try Again.");
    let _dummy: String = user_input::get_user_input_trimmed("");
    clear_terminal::clear_terminal();
}

