mod text_parser;
mod user_input;
mod random_code_generator;
mod code_guess;
mod round;
mod clear_terminal;
mod wrap_text;
mod play;

use crate::user_input::get_user_input_trimmed;

# [derive(Debug, PartialEq)]
enum GameState {
    MainMenu,
    Play,
    Tutorial { shown: bool },
    Quit,
}

impl GameState {
    fn change_to (&mut self, new_state: GameState) {
        if *self != new_state {
            *self = new_state;
            clear_terminal::clear_terminal();
        }
    } 

    fn on_enter(&mut self){
        match self {
            GameState::MainMenu => {
                title();
                main_menu_display();
            }

            GameState::Tutorial { shown } => {
                title();
                if !*shown {
                    tutorial_display();
                    *shown = true;
                }
            }

            GameState::Quit => {
                println!("Quit");
            }

            _ => {

            }
        }
    }

    fn update (&mut self) -> usize{
        match self {
            GameState::MainMenu => {
                //println!("Menu!");

                main_menu_handling(self);
                separating_line();
            },
            GameState::Play => {
                //println!("Playing");

                play::play();
            },
            GameState::Tutorial { .. } => {
                println!("Type anything to Return to Main Menu");
                let _choice: String = get_user_input_trimmed("");

                self.change_to(GameState::MainMenu);
            }
            GameState::Quit => {
                println!("Quit");
                return 0;
            }
        }

        return 1;
    }
 
}

fn main() {
    print!("\x1b]2;CodeAtro\x07");


    let mut current_state: GameState = GameState::MainMenu;
    loop {
        current_state.on_enter();
        let game_result: usize = current_state.update();

        if game_result == 0 {
            return;
        }
    }

    /*
    loop {

        if &current_state != &previous_state {        
            match &current_state {
                GameState::MainMenu => {
                    title();

                    main_menu(&mut current_state);
                    separating_line();
                },
                GameState::Play => {
                    play();
                },
                GameState::Tutorial => {
                    tutorial();
                },
                GameState::Quit => {
                    return;
                },
                _ => println!("Error"),
            }
        }

        previous_state = current_state;        
    }

    */
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

fn main_menu_display(){
    //title();

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
    //separating_line();
}

fn main_menu_handling (state: &mut GameState) {
    let input: String = user_input::get_user_input_trimmed("");
    let choice: usize = text_parser::text_to_u_int(&input);

    match choice {
        1 => state.change_to(GameState::Play),
        2 => state.change_to(GameState::Tutorial {shown: false}),
        3 => state.change_to(GameState::Quit),
        _ => println!("Error"),
    }
}

fn tutorial_display(){
    let tutorial_text: &str = r"Hello, and Welcome to CodeAtro! My Name is Timbo, and I am Here to Teach you How to Play this Game!!!

Your Aim in this Game is to Guess the Code. The Code is just a Sequence of Characters. 
These Characters are taken from a Larger Collection of Characters called the Alphabet. The Alphabet has all the Characters separated by Spaces. You will be Shown the Alphabet before every attempt you make to guess the Code. You will also be Shown the Length of the Code. 

To Enter a Guess, you just have to type out the Guess, with each Character separated by Spaces (any Whitespace should do the Trick, to be Honest). Once you Enter a Guess, you will be Shown your Result. The Result Shows you, for Each Character of the Guess, whether it is In Place (the Character has been correctly guessed with its Position), Out Of Place (the Character is in the Code but in a different Position) or Not In Code (the Character is not present in the Code). For Each Character in the Guess, you will be Given a Score. Summing the Scores for each Character yields the total Score for that Guess. 

Once you have Guessed, you can Guess and Guess again. But Beware, you have a Finite number of Guesses. If you manage to Guess the Code before running out of Guesses, you Beat the Code, and Move on to the Next Code. Else, you Lose.

Oh, and Also, every Time you Beat a Code, you get to Choose one out of Three JoChars. JoChars can either Buff or Debuff your Run. Why would you Choose a JoChar which Debuffs your Run? I don't know. But the Option is there!";

    println!("{}", wrap_text::wrap_text(tutorial_text));

    separating_line();
}

pub fn separating_line(){
    println!("----------");
}
