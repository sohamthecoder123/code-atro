use rand::Rng;
use rand::seq::SliceRandom;

pub fn generate_code(alphabet: &Vec<String>, length: usize) -> Vec<String>{
    let mut generated_code: Vec<String> = Vec::new();

    for _ in 0..length {
        let index: usize = rand::thread_rng().gen_range(0..alphabet.len());
        generated_code.push(alphabet[index].clone());
    }

    return generated_code;
}

pub fn generate_alphabet(overall_alphabet: &Vec<String>, alphabet_length: usize) -> Vec<String>{
    let mut _generated_alphabet: Vec<String> = Vec::new();

    _generated_alphabet = overall_alphabet.clone();
    _generated_alphabet.shuffle(&mut rand::thread_rng());
    _generated_alphabet.truncate(alphabet_length);

    return _generated_alphabet;    
}
