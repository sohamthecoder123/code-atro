/*
This file handles the random code generation
*/

use rand::Rng; 
use rand::seq::SliceRandom;

//Randomly generates the code on the basis of a given alphabet
pub fn generate_code(alphabet: &Vec<String>, length: usize) -> Vec<String>{
    let mut generated_code: Vec<String> = Vec::new();

    for _ in 0..length {
        let index: usize = rand::thread_rng().gen_range(0..alphabet.len());
        generated_code.push(alphabet[index].clone());
    }

    return generated_code;
}

//Randomly generates an alphabet given an overall alphabet 
pub fn generate_alphabet(overall_alphabet: &Vec<String>, alphabet_length: usize) -> Vec<String>{
    //How this works:
    //Step 1. Generate a random permutation of the alphabet
    //Step 2. Cut off this random permutation at the desired length

    let mut _generated_alphabet: Vec<String> = Vec::new();

    _generated_alphabet = overall_alphabet.clone(); //clone the overall_alphabet
    _generated_alphabet.shuffle(&mut rand::thread_rng()); //permutate the clone
    _generated_alphabet.truncate(alphabet_length); //cut off the clone at the given length

    return _generated_alphabet;    
}
