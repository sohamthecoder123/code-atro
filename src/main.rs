use crate::code_guess::ResultCode;

mod text_parser;
mod user_input;
mod random_code_generator;
mod code_guess;
mod run;

fn main() {
    let alphabet: &str = "a b c d e f g h i j k l m n o p q r s t u v w x y z";

    let result: Vec<ResultCode> = run::run(alphabet, 5);

    let mut score: f64 = 0.0;

    for _i in &result {
        match _i {
            ResultCode::InPlace => {
                println!("In Place!");
                score += 1.0;
            }

            ResultCode::OutOfPlace => {
                println!("Out Of Place!");
                score += 0.0;
            }

            ResultCode::NotInCode => {
                println!("Not In Code!");
                score -= 0.5;
            }
            
            ResultCode::SizeError => {
                println!("Size Error!!!");
            }
        }
    }

    println!("Score: {}", score);

}
