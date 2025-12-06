use rand::Rng;

pub fn generate_code(alphabet: &Vec<String>, length: usize) -> Vec<String>{
    let mut generated_code: Vec<String> = Vec::new();

    for _ in 0..length {
        let index: usize = rand::thread_rng().gen_range(0..alphabet.len());
        generated_code.push(alphabet[index].clone());
    }

    return generated_code;
}

