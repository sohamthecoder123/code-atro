use terminal_size::{terminal_size, Width, Height};
use textwrap::fill;

pub fn wrap_text(text: &str) -> String{
    let size: Option<(Width, Height)> = terminal_size();
    let width: usize = if let Some((Width(w), Height(_))) = size {
        w as usize
    }
    else {
        80 //default
    };

    let wrapped_text: String = fill(text, width);
    return wrapped_text;
}