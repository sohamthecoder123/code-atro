mod text_parser; //parses the text (for eg, converts strings to the vectors that represent the code)
mod user_input; //handles user input
mod random_code_generator; //handles random code generation
mod code_guess; //handles the code guessing logic (basically comparing the two vectors - the actual code and the guess)
mod round; //handles every round of the game
mod clear_terminal; //has a fn which clears the terminal
mod wrap_text; //has a fn which makes the text wrap
mod play; //handles the play state
mod jochar; //handles the JoChars

//GameState enum. Helps screen navigation
# [derive(Debug, PartialEq)]
enum GameState {
    MainMenu, //the main menu
    Play, //play state
    Tutorial { shown: bool }, //tutorial. "shown" handles whether the tutorial text has been shown before or not
    Quit, //quit
}

//functions for the game state
impl GameState {
    //handles changing the state (changes state if the older state isn't the same as the new one.)
    fn change_to (&mut self, new_state: GameState) {
        if *self != new_state { //checks if the older one is the sae as the new one
            *self = new_state; //change the state
            clear_terminal::clear_terminal(); //clear the terminal
        }
    } 

    //handles stuff that is run at the start of the loop
    fn on_enter(&mut self){
        match self {
            //Main menu
            GameState::MainMenu => {
                title(); //title card 
                main_menu_display(); //the display for the main menu
            }

            //Tutorial
            GameState::Tutorial { shown } => {
                title(); //title card
                if !*shown { //if the tutorial text hasn't been shown before
                    tutorial_display(); //show the tutorial text
                    *shown = true; //set shown to false
                }
            }

            //Quit
            GameState::Quit => {
                println!("Quit"); //duh
            }

            _ => {
                //Eat 5*. Do nothing.
            }
        }
    }

    //handles stuff that is updated. usually deals with things involving user input and the actual play state 
    fn update (&mut self) -> usize{
        match self {
            //Menu
            GameState::MainMenu => {
                main_menu_handling(self); //handles the user choice at the main menu
                separating_line(); //The Separating Line of Death (tm)
            },

            //Play
            GameState::Play => {
                play::play(); //play
            },

            //Tutorial.
            GameState::Tutorial { .. } => {
                //handles user input to return back to the main menu
                println!("Type anything to Return to Main Menu");
                let _choice: String = user_input::get_user_input_trimmed("");

                self.change_to(GameState::MainMenu); //go back to main menu state
            }

            //Quit
            GameState::Quit => {
                println!("Quit");
                return 0; //return 0 here to return 0 in main (see main() for further info)
            }
        }
        
        return 1; //this return value is technically useless, but i have to return it because of how i am handling quitting.
    }
 
}

fn main() {
    //change the name of the tab in the terminal. I have little idea how it works. It just does. (Glory be to Todd Howard.)
    print!("\x1b]2;CodeAtro\x07");


    let mut current_state: GameState = GameState::MainMenu; //defines a new variable that actually stores the current game state and sets it to the initial value of MainMenu (ie, the game opens with the menu)
    
    //the main gameplay loop
    loop {
        current_state.on_enter(); //run the code which is to be run at the start


        let game_result: usize = current_state.update(); //store the return value. 
        //basically, what i am doing here is that the update() fn returns 1 if the player hasn't quit, and 0 if they have
        //so i store that in this variable
        //a game result of 1 means the player hasn't quit, and 0 means they have.
        //so, if the game result is 0...
        if game_result == 0 {
            return; //...we return nothing. This stops the execution of this fn and terminates the program.
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

//the title card
fn title(){
    //got this neat ascii font from this: https://patorjk.com/software/taag/
    //the font is Doom/DooM
    //the &str, much like the food prepared by a cook who's about to be shown the door by Gordon Ramsay, is raw.
    let heading: &str = r" _____           _       ___  _             
/  __ \         | |     / _ \| |            
| /  \/ ___   __| | ___/ /_\ \ |_ _ __ ___  
| |    / _ \ / _` |/ _ \  _  | __| '__/ _ \ 
| \__/\ (_) | (_| |  __/ | | | |_| | | (_) |
 \____/\___/ \__,_|\___\_| |_/\__|_|  \___/ 
                                            
                                            "; 
    
    //print the heading
    println!("{}", heading);
    separating_line(); //The Separating Line of Death (tm)

}
//"Huh, after all that training, I really feel [TITLE CARD]" - John Invincible, 2023, colorized

//handles all the stuff that is displayed at the main menu.
fn main_menu_display(){
    //Main Menu
    println!("Main Menu");
    separating_line();

    //option 1, play the game
    println!("1 - Play");
    separating_line();

    //option 2, read the tutorial
    println!("2 - Tutorial (Recommended if you haven't Played before)");
    separating_line();

    //option 3, quit
    println!("3 - Quit");
    separating_line();

    //pretty self explanatory, ig
    println!("");
    println!("Please Enter Below the Index Number of the Option you want to Select.");
    //separating_line();
}

//gets the choice of the player and changes the state of the game on that basis
fn main_menu_handling (state: &mut GameState) {
    //get the user's choice
    let input: String = user_input::get_user_input_trimmed("");
    //convert the user's choice to an int cuz why not
    let choice: usize = text_parser::text_to_u_int(&input);

    match choice {
        1 => state.change_to(GameState::Play), //play the game if the choice is 1
        2 => state.change_to(GameState::Tutorial {shown: false}), //show the tutorial if the choice is 2
        3 => state.change_to(GameState::Quit), //quit if the choice is 3
        _ => clear_terminal::clear_terminal()//clear the terminal if it is anything else. Because of the fact that this is a part of a loop{} (see main()), we just get the main menu displayed again.
    }
}

//handles the tutorial text
fn tutorial_display(){
    //the tutorial text.
    let tutorial_text: &str = r"Hello, and Welcome to CodeAtro! My Name is Timbo, and I am Here to Teach you How to Play this Game!!!

Your Aim in this Game is to Guess the Code. The Code is just a Sequence of Characters. 
These Characters are taken from a Larger Collection of Characters called the Alphabet. The Alphabet has all the Characters separated by Spaces. You will be Shown the Alphabet before every attempt you make to guess the Code. You will also be Shown the Length of the Code. 

To Enter a Guess, you just have to type out the Guess, with each Character separated by Spaces (any Whitespace should do the Trick, to be Honest). Once you Enter a Guess, you will be Shown your Result. The Result Shows you, for Each Character of the Guess, whether it is In Place (the Character has been correctly guessed with its Position), Out Of Place (the Character is in the Code but in a different Position) or Not In Code (the Character is not present in the Code). For Each Character in the Guess, you will be Given a Score. Summing the Scores for each Character yields the total Score for that Guess. 

Once you have Guessed, you can Guess and Guess again. But Beware, you have a Finite number of Guesses. If you manage to Guess the Code before running out of Guesses, you Beat the Code, and Move on to the Next Code. Else, you Lose.

Oh, and Also, every Time you Beat a Code, you get to Choose one out of Three JoChars. JoChars can either Buff or Debuff your Run. Why would you Choose a JoChar which Debuffs your Run? I don't know. But the Option is there!";

    println!("{}", wrap_text::wrap_text(tutorial_text)); //wrap the tutorial text properly

    separating_line(); //The Separating Line of Death (tm)
}

//The Separating Line of Death (tm)
pub fn separating_line(){
    println!("---------------------------------------------"); //...there's prolly something like replication which makes this easier, but i like this as it shows me what i would actually see in-game. 
}
