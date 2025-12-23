use crate::wrap_text;

# [derive(Clone)]
pub struct JoChar {
    name: String,
    desc: String,
    pub cost: isize,
    pub rarity: isize,
    pub is_debuff: bool,
}

pub fn initiate_jochars() -> Vec<JoChar>{
    let mut available_jochars: Vec<JoChar> = Vec::new();

    let regular: JoChar = JoChar {
        name: "Regular JoChar".to_string(),
        desc: "Adds +4 to your Score at the end of a Round.".to_string(),
        cost: 5,
        rarity: 1,
        is_debuff: false,
    };

    let advanced: JoChar = JoChar {
        name: "Advanced JoChar".to_string(),
        desc: "Adds +4 to your Score at the end of an Attempt.".to_string(),
        cost: 10,
        rarity: 2,
        is_debuff: false,
    };

    let funny: JoChar = JoChar { 
        name: "Funny JoChar".to_string(), 
        desc: "Reveals 1 Character present in the Code (without Position) before the Start of the Round.".to_string(), 
        cost: 7, 
        rarity: 2,  
        is_debuff: false,
    };

    let funnier: JoChar = JoChar { 
        name: "Funnier JoChar".to_string(), 
        desc: "Reveals 1 Character in the Code and its Position before the Start of the Round.".to_string(), 
        cost: 14, 
        rarity: 3,
        is_debuff: false,  
    };


    let even_funnier: JoChar = JoChar { 
        name: "Even Funnier JoChar".to_string(), 
        desc: "Reveals 2 Characters in the Code (without Positions) before the Start of the Round.".to_string(), 
        cost: 28, 
        rarity: 4,
        is_debuff: false,  
    };

    let funniest: JoChar = JoChar { 
        name: "Funniest JoChar".to_string(), 
        desc: "Reveals 2 Characters in the Code and their Positions before the Start of the Round.".to_string(), 
        cost: 28, 
        rarity: 4,
        is_debuff: false,  
    };

    let absolute: JoChar = JoChar { 
        name: "Absolute JoChar".to_string(), 
        desc: "When Bought, Converts the current Wealth to its Absolute Value, before Deleting itself from held Jochars.".to_string(), 
        cost: 20, 
        rarity: 2,
        is_debuff: false,  
    };

    let double_the_stakes: JoChar = JoChar { 
        name: "Double The Stakes".to_string(), 
        desc: "Doubles both the Score gained by getting a Character right, and the penalty on getting it wrong.".to_string(), 
        cost: 10,
        rarity: 2,
        is_debuff: false, 
    };    

    let vowel_phile: JoChar = JoChar {
        name: "VowelPhile".to_string(),
        desc: "The Score provided by every Vowel guessed correctly is doubled. Penalties remian unaffected.".to_string(),
        cost: 5,
        rarity: 1,
        is_debuff: false,
    };

    let consonant_phile: JoChar = JoChar {
        name: "ConsonantPhile".to_string(),
        desc: "The Score provided by every Consonant guessed correctly is doubled. Penalties remian unaffected.".to_string(),
        cost: 5,
        rarity: 1,
        is_debuff: false,
    };

    let number_phile: JoChar = JoChar {
        name: "NumberPhile".to_string(),
        desc: "The Score provided by every Number guessed correctly is doubled. Penalties remian unaffected.".to_string(),
        cost: 5,
        rarity: 1,
        is_debuff: false,
    };

    let collector: JoChar = JoChar {
        name: "The Collector".to_string(),
        desc: "+0.25 Score for each JoChar held.".to_string(),
        cost: 5,
        rarity: 1,
        is_debuff: false,
    };

    let debuff_collector: JoChar = JoChar {
        name: "Debuff Collector".to_string(),
        desc: "+1 Score for each Debuff JoChar held.".to_string(),
        cost: 5,
        rarity: 1,
        is_debuff: false,
    };

    let negative_marking: JoChar = JoChar {
        name: "Negative Marking".to_string(),
        desc: "-1 Score for each Character present in a Guess but not in the Code.".to_string(),
        cost: 2,
        rarity: 1,
        is_debuff: true,
    };

    available_jochars.push(regular); //0
    available_jochars.push(advanced); //1
    available_jochars.push(funny); //2
    available_jochars.push(funnier); //3
    available_jochars.push(even_funnier); //4
    available_jochars.push(funniest); //5
    available_jochars.push(absolute); //6
    available_jochars.push(double_the_stakes); //7
    available_jochars.push(vowel_phile); //8
    available_jochars.push(consonant_phile); //9
    available_jochars.push(number_phile); //10
    available_jochars.push(collector); //11
    available_jochars.push(debuff_collector); //12
    available_jochars.push(negative_marking); //13

    return available_jochars;
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
}
