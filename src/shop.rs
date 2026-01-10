use crate::jochar;
use crate::separating_line;
use crate::user_input;
use crate::text_parser;
use rand::prelude::*;
use rand::distributions::{Distribution, WeightedIndex};
use rand::thread_rng;
use rand::Rng;

pub fn shop (wealth: &mut f64, available_jochars: &Vec<jochar::JoChar>, jochars_in_play: &mut Vec<usize>){
    separating_line();
    println!("---SHOP---");
    
    //Step 1: Generate the JoChars in the shope
    //firstly, we will create 4 vectors spitting the JoChars into rarity
    let rarity_1: Vec<usize> = jochar::return_jochars_rarity(1, &available_jochars);
    let rarity_2: Vec<usize> = jochar::return_jochars_rarity(2, &available_jochars);
    let rarity_3: Vec<usize> = jochar::return_jochars_rarity(3, &available_jochars);
    let rarity_4: Vec<usize> = jochar::return_jochars_rarity(4, &available_jochars);
    
    //secondly, we will decide on the rarity level of the three jochars and store it in an array
    //This is done by the generate_array function
    let jochar_rarity_array: [i32; 3] = generate_array();

    //finally, we shall randomly choose elements on the basis of the rarity chosen.
    let mut rng: ThreadRng = rand::thread_rng();
    let mut market: Vec<usize> = Vec::new(); //stores the indices of the jochars available in the market
    
    for i in jochar_rarity_array{
        match i {
            1 => {
                let random_number: usize = rng.gen_range(0..rarity_1.len());
                market.push(rarity_1[random_number]);
            }

            2 => {
                let random_number: usize = rng.gen_range(0..rarity_2.len());
                market.push(rarity_2[random_number]);
            }

            3 => {
                let random_number: usize = rng.gen_range(0..rarity_3.len());
                market.push(rarity_3[random_number]);
            }

            4 => {
                let random_number: usize = rng.gen_range(0..rarity_4.len());
                market.push(rarity_4[random_number]);
            }

            _ => {

            }
        }
    }

    //Step 2: Now that the market has been decided, time to show the user what they got
    println!("The JoChars in the Market are:");
    for i in 0..3 {
        separating_line();
        println!("JoChar {}: ", i);
        jochar::show_jochar(&available_jochars[market[i]]);
    }

    //Step 3: Now, get the user input regarding the JoChar they want
    println!("Enter your Choice: ");
    loop {
        let input_str: String = user_input::get_user_input_trimmed("");
        let choice: usize = text_parser::text_to_u_int(&input_str) - 1;

        let available_choice: Vec<usize> = vec![0, 1, 2];

        if available_choice.contains(&choice) {
            //Step 4: Add the chosen JoChar to the jochars_in_play
            let index: usize = market[choice];
            jochars_in_play[index] += 1;

            //Step 5: Decrease the player's wealth
            *wealth -= available_jochars[index].cost;
        }

        println!("INVALID CHOICE!!!");
    }
}

fn generate_array() -> [i32; 3] {
    let choices: [i32; 4] = [1, 2, 3, 4];
    let weights: [i32; 4] = [4, 3, 2, 1]; // exact probability ratios

    // Create the weighted index distribution
    let dist: WeightedIndex<i32> = WeightedIndex::new(&weights).unwrap();
    let mut rng: ThreadRng = thread_rng();

    [
        choices[dist.sample(&mut rng)],
        choices[dist.sample(&mut rng)],
        choices[dist.sample(&mut rng)],
    ]
}
