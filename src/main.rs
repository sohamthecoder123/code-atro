mod text_parser;
mod user_input;
mod random_code_generator;
mod code_guess;
mod round;

fn main() {
    
    title();

    let choice: usize = main_menu();

    if choice == 1 {
        play();
    }

    else if choice == 2 {
        tutorial();
    }

    loop {
        
    }
/*
    let mut score: f64 = 0.0;

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

*/
}

fn title(){
    let heading: &str = r" _____           _       ___  _             
/  __ \         | |     / _ \| |            
| /  \/ ___   __| | ___/ /_\ \ |_ _ __ ___  
| |    / _ \ / _` |/ _ \  _  | __| '__/ _ \ 
| \__/\ (_) | (_| |  __/ | | | |_| | | (_) |
 \____/\___/ \__,_|\___\_| |_/\__|_|  \___/ 
                                            
                                            ";
    println!("{}", heading);
    separating_line();

}

fn main_menu() -> usize{
    println!("Main Menu");
    separating_line();

    println!("1 - Play");
    separating_line();

    println!("2 - Tutorial (Recommended if you haven't Played before)");
    separating_line();

    println!("3 - Quit");
    separating_line();

    println!("");
    println!("Please Enter Below the Number to the Left of the Option you want to Select.");
    separating_line();

    let input: String = user_input::get_user_input_trimmed("");
    let choice: usize = text_parser::text_to_u_int(&input);

    return choice;
}

fn play(){
    let alphabet: &str = "a b c d e f g h i j k l m n o p q r s t u v w x y z";

    //let result: Vec<ResultCode> = round::round(alphabet, 5);

    round::round(alphabet, 5);
}

fn tutorial(){
    let tutorial_text: &str = r"";
}

pub fn separating_line(){
    println!("----------");
}
