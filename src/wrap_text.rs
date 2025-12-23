/*
This file wraps the text around on the basis of the terminal size
*/

use terminal_size::{terminal_size, Width, Height};
use textwrap;

//wraps the given text on the basis of the terminal size 
pub fn wrap_text(text: &str) -> String{
    let size: Option<(Width, Height)> = terminal_size();
    let width: usize = if let Some((Width(w), Height(_))) = size {
        w as usize //set the terminal width as the width on the basis of which the wrapping will happen
    }
    else {
        80 //default width
    };

    let wrapped_text: String = textwrap::fill(text, width); //wrap the text on the basis of the terminal width
    return wrapped_text; //return the wrapped text
}