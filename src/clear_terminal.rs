use clearscreen::clear;

pub fn clear_terminal(){
    clear().expect("Failed to Clear Screen.");
}