use crate::gamestate_helpers::{PlayerColor, self, turn_column_to_encoded_gamestate};

pub struct Engine {
    color: PlayerColor,
}

impl Engine {
    /// Creates engine for human player
    pub fn new(color: PlayerColor) -> Engine{
        Engine{color}
    }

    /// Asks the user to input the next move of the human player after displaying the current gamestate
    pub fn make_move(&self, gamestate: u32) -> u32 {
        self.print_current_gamestate(gamestate);
        let mut next_move: u32 = 0;

        while next_move == 0 {
            let column = ask_for_next_move();

            if let Some(i) = turn_column_to_encoded_gamestate(gamestate, column, &self.color) {
                next_move = i;
            } else {
                println!("Not a valid column and or move!");
            }
        }
        next_move
    }

    /// Prints the current gamestate for the user
    fn print_current_gamestate(&self, gamestate: u32) {
        match self.color {
            PlayerColor::Blue => println!("It is blue's turn!"),
            PlayerColor::Red => println!("It is red's turn!"),
        };
    
        let current_gamestate_as_string: String = gamestate_helpers::encoded_gamestate_to_str(gamestate);
        println!("The current gamestate is:");
        println!("{}", current_gamestate_as_string);
    }
}


/// Asks the user to input the column of the next token that is supposed to be played
fn ask_for_next_move() -> u32 {

    // Infinite loop getting the column of the next move
    loop {
        let mut input = String::new();

        println!("Please enter the number of the column you want to place your token in:");
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(error) => panic!("Problem reading in input: {:?}", error),
        }

        // Matching the input to the 
        let mut val = match parse_string_tuple(input.trim()) {
            Some(i) => i,
            None => 0,
        };

        if val < 5 && val > 0 {
            return val;
        } else {
            println!("Invalid column input!")
        }
    }
}


/// Parsing string and checking whether an int has been passed
fn parse_string_tuple(string: &str) -> Option<u32> {
    if let Ok(i) = string.parse() {
        Some(i)
    } else {
        None
    }
}



// To do: Add tests
#[cfg(test)]
mod tests {
    use super::*;
    const BASE: u32 = 2;

}