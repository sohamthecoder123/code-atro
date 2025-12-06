/*
 * The aim of this module is to provide a simple way to get user input, instead of writing io::stdin()... everytime
 * 
 * Current functions:
 * 1. get_user_input() 
 *      Returns the string the user inputted, till the first \n. Gives error if it failed to read the user input
 * 
 * 2. get user_input_trimmed(style)
 *      Returns the user input as above, but with whitespaces trimmed depending on the style. If the style is "l", only left whitespaces are removed, and similarly for "r". style="" trims whitespaces from both sides.
 */

use std::io;

#[allow(unused_doc_comments)]
pub fn get_user_input() -> String{
    ///  get_user_input() 
    ///    Returns the string the user inputted, till the first \n. Gives error if it failed to read the user input
    
    let mut user_input: String = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    return user_input;

}

#[allow(unused_doc_comments)]
pub fn get_user_input_trimmed(style: &str) -> String{

    ///get user_input_trimmed(style)
    ///     Returns the user input as above, but with whitespaces trimmed depending on the style. If the style is "l", only left whitespaces are removed, and similarly for "r". style="" trims whitespaces from both sides.

    match style {
        "l" => return get_user_input().trim_start().to_string(),
        "r" => return get_user_input().trim_end().to_string(),
        "" => return get_user_input().trim().to_string(),
        _ => return "/u/Cannot trim in this style".to_string(),
    };

}