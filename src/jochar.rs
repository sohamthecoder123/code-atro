/*
This file stores the JoChars. Minaly description and stuff.
The code behind how each JoChar actually functions is in round and play 
 */

use crate::wrap_text; //because we are displaying text (the descriptions, etc.)

use std::fs::File;
use std::io::{BufRead, BufReader};


//the JoChar struct. Each JoChar has basic stuff like name, description, cost, rarity and a variable to store if it is a debuff or not
# [derive(Clone)]
pub struct JoChar {
    name: String, //the name of the JoChar
    desc: String, //description - what the JoChar does
    pub cost: f64, //the cost of the JoChar in the Shop
    pub rarity: isize, //the rarity of the JoChar. 
    pub is_debuff: bool, //whether the JoChar is a debuff or not. Used by the Debuff Collector JoChar to determine the bonus it gives out.
}


pub fn show_jochar(jo_char: &JoChar){
    println!("Name: {}", wrap_text::wrap_text(&jo_char.name));
    println!("Description: {}", wrap_text::wrap_text(&jo_char.desc));

    if jo_char.is_debuff {
        println!("Debuff: Yes");
    }

    else {
        println!("Debuff: No");
    }

    println!("Cost: {:.2}", &jo_char.cost);

    println!("Rarity: {}", &jo_char.rarity);
}

pub fn load_jochars(path: &std::path::Path) -> Vec<JoChar> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return Vec::new();
        }
    };

    let reader: BufReader<File> = BufReader::new(file);

    let mut jochars: Vec<JoChar> = Vec::new();
    let mut buffer: Vec<String> = Vec::new();
    let mut in_section = false;

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l.trim().to_string(),
            Err(e) => {
                eprintln!("Read error: {}", e);
                continue;
            }
        };

        match line.as_str() {
            "<<<" => {
                buffer.clear();
                in_section = true;
            }
            ">>>" => {
                if buffer.len() != 5 {
                    eprintln!("Invalid section (wrong length) {:?}", buffer);
                    in_section = false;
                    continue;
                }

                let cost = match buffer[2].parse::<f64>() {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("Invalid cost");
                        continue;
                    }
                };

                let rarity = match buffer[3].parse::<isize>() {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("Invalid rarity");
                        continue;
                    }
                };

                let is_debuff = match buffer[4].parse::<bool>() {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("Invalid is_debuff");
                        continue;
                    }
                };

                jochars.push(JoChar {
                    name: buffer[0].clone(),
                    desc: buffer[1].clone(),
                    cost,
                    rarity,
                    is_debuff,
                });

                in_section = false;
            }
            _ if in_section && !line.is_empty() => buffer.push(line),
            _ => {}
        }
    }

    jochars
}

pub fn return_jochars_rarity(rarity: isize, jochars_vec: &Vec<JoChar>) -> Vec<usize> {
    let mut return_vector: Vec<usize> = Vec::new();
    let mut index: usize = 0;

    for i in jochars_vec {
        
        if i.rarity == rarity {
            return_vector.push(index);
        }

        index += 1;
    }

    return return_vector;
}
