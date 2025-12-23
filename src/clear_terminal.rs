/*
Clears the terminal
*/

use clearscreen::clear; //this is a crate that clears the terminal 

pub fn clear_terminal(){
    clear().expect("Failed to Clear Screen."); //clear the terminal. In case of an error, just show this.
}