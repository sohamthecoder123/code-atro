/*
Herein lies a function that takes in 2 inputs: the code and a guess, compares them, and gives out the result
*/

pub enum ResultCode {
    InPlace,
    OutOfPlace,
    NotInCode,
    SizeError,
}

#[allow(unused_doc_comments)]
pub fn guess_code_check(code: &Vec<String>, guess: &Vec<String>) -> Vec<ResultCode>{
    ///
    /// Takes in two inputs: the code and a guess. Compares the two
    /// Returns a vector of Strings
    /// Each element of the vector tells you whether the corresponding guess was correct and in place, correct but out of place, or not correct at all
    /// Returns vec!["size_error".to_string()] if the guess wasn't of the same size as the code. This prevents the game from punishing the player if they accidentally pressed enter
    /// 
    
    // error in case size of the guess doesn't match that of the code
    if code.len() != guess.len(){
        return vec![ResultCode::SizeError,];
    }

    //if the sizes do match
    else{
        //create a dummy variable to return
        let mut result: Vec<ResultCode> = Vec::new();

        //go through the code vector line by line
        for i in 0..code.len(){
            
            //if the corresponding guess matches
            if guess[i] == code[i]{
                result.push(ResultCode::InPlace);
            }

            //if the corresponding guess is in the code, but not here
            else if code.contains(&guess[i]){
                result.push(ResultCode::OutOfPlace);
            }

            //if the corresponding guess is not in the code
            else{
                result.push(ResultCode::NotInCode)
            }
        }

        //return the dummy variable
        return result;
    }
}
