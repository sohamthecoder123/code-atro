use crate::round;
use crate::user_input;
use crate::clear_terminal;

pub fn play(){
    let mut round_no: usize = 1;
    let mut current_length: usize = 2;

    let overall_alphabet: &str = "a b c d e f g h i j k l m n o p q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9";

    loop {
        println!("Round {}", round_no);
        println!("----------");
        let has_won: bool = round::round(overall_alphabet, 2 * current_length, current_length, current_length + 1);
        
        if !has_won {
            break;
        }

        println!("Round {} has been Defeated", round_no);      
        println!("----------");  
        round_no += 1;

        let two: usize = 2;

        current_length = two.pow((round_no / 2) as u32);
    }
    
    println!("You Lost!!!");
    println!("Enter anything to Try Again.");
    let _dummy: String = user_input::get_user_input_trimmed("");
    clear_terminal::clear_terminal();
}

