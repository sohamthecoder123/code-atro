/*
This file handles the random code generation
*/

use rand::Rng; 
use rand::seq::SliceRandom;
use std::collections::HashMap;

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

pub fn reveal_characters_random(code: &Vec<String>, n: usize) -> Vec<String>{
    let mut _revealed_characters: Vec<String> = Vec::new();

    _revealed_characters = code.clone();
    _revealed_characters.shuffle(&mut rand::thread_rng());
    _revealed_characters.truncate(n);

    return _revealed_characters;
}

pub fn reveal_characters_position(code: &Vec<String>, n: usize) -> HashMap<usize, &String>{
    let mut _perm_1_to_n: Vec<usize> = (1..=n).collect();
    _perm_1_to_n.shuffle(&mut rand::thread_rng());
    _perm_1_to_n.truncate(n);

    let mut _revealed_characters: HashMap<usize, &String> = HashMap::new();

    for i in _perm_1_to_n {
        _revealed_characters.insert(i, &code[i-1]);
    }

    return _revealed_characters;

}

