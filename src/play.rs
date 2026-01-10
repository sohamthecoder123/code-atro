use crate::round;
use crate::separating_line;
use crate::shop;
use crate::user_input;
use crate::clear_terminal;
use crate::jochar;
use std::env;
use std::path::PathBuf;

fn asset_path(name: &str) -> PathBuf {
    //println!("Exe path: {:?}", std::env::current_exe()?);
    let exe = env::current_exe()
        .expect("Failed to get executable path");
    exe.parent()
        .expect("Executable must be in a directory")
        .join("assets")
        .join(name)
}


pub fn play(){
    let jochar_path: PathBuf = asset_path("jochars.txt");
    let available_jochars: Vec<jochar::JoChar> = jochar::load_jochars(&jochar_path);

    let mut jochars_in_play: Vec<usize> = vec![0; available_jochars.len()];

    let rarity_1: Vec<usize> = jochar::return_jochars_rarity(1, &available_jochars);
    
    for i in &rarity_1 {
        println!("{}", i);
    }

    jochars_in_play[1] = 2;
    jochars_in_play[5] = 1;
    jochars_in_play[7] = 1;
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
        let has_won: bool = round::round(overall_alphabet, 2 * current_length, current_length, current_length + 1, &mut wealth, number_of_debuffs, &jochars_in_play);
        
        if !has_won {
            break;
        }

        println!("Round {} has been Defeated", round_no);      
        separating_line();  

        let two: usize = 2;
        current_length = two.pow((round_no / 2) as u32);

        println!("Enter anything to Go To Shop.");
        let _dummy: String = user_input::get_user_input_trimmed("");
        clear_terminal::clear_terminal();
        shop::shop(&mut wealth, &available_jochars, &mut jochars_in_play);

        round_no += 1;
    }
    
    println!("You Lost!!!");
    println!("Enter anything to Try Again.");
    let _dummy: String = user_input::get_user_input_trimmed("");
    clear_terminal::clear_terminal();
}

