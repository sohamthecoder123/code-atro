/*
This file has 3 functions that all take in a &str and return the characters of the string in a different format.
*/

use crate::clear_terminal; 

//converts the given &str to the Code format. The Code is just a vector of Strings.
//(this is prolly one of the ugliest pieces of code i've written in my life)
pub fn text_to_code(text: &str) -> Vec<String> {
    let text_vec: Vec<&str> = text.split_whitespace().collect(); //split the text on whitespaces, and store it in a &str vector

    let mut code_vec: Vec<String> = Vec::new();

    //Effectively convert the &str vec to a String vec. Why would I do this? Because the Code format is a vector of Strings, not of &strs, and much of the code_guess stuff uses this.
    for j in text_vec{
        let code: String = j.to_string();
        code_vec.push(code);
    }

    return code_vec; //return the String vec
}

//converts the given numeric text to an isize. Give an error if this isn't possible.
# [allow(dead_code)] //because this is a vestigial organ from a bygone era when the Code was limited only to integers
 pub fn text_to_int(text: &str) -> isize {
    let num: isize = match text.parse::<isize>() {
        Ok(n) => n,
        Err(e) => {
            println!("Error: {}", e);
            return -1;
        }
    };
    return num;
}

//converts the given numeric text to a usize. Give an error if this isn't possible.
pub fn text_to_u_int(text: &str) -> usize {
    let num: usize = match text.parse::<usize>() {
        Ok(n) => n,
        Err(e) => {
            println!("Error: {}", e);
            clear_terminal::clear_terminal();
            return 0;
        }
    };
    //let num: usize = text.parse().expect("Error");
    return num;
}
