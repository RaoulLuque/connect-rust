use crate::gamestate_helpers::{PlayerColor, self};

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
/// To do: ask again if Turn invalid
fn ask_for_next_move() -> u32 {
    let mut input = String::new();

    // Infinite loop getting the column of the next move
    loop {
        println!("Please enter the number of the column you want to place your token in:");
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(error) => panic!("Problem reading in input: {:?}", error),
        }

        // Matching the input to the 
        if let Some(val) = parse_string_tuple(input.trim()) {
            return val;
        } else {
            println!("Invalid column input please try again!");
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


/// Turns an encoded tuple move into an encoded u32 with the color whos players turn it should be
fn turn_column_to_encoded_gamestate(gamestate: u32, column: u32, color: &PlayerColor) -> Option<u32> {
    let base: u32 = 2;
    let mut row_counter: u32 = base.pow(3*8 + (column - 1) * 2);
    while (row_counter & gamestate == row_counter) || (row_counter * 2 & gamestate == row_counter * 2) {
        row_counter /= base.pow(8);
    }
    if row_counter == 0 {
        None
    } else {
        match color {
            PlayerColor::Blue => Some(row_counter),
            PlayerColor::Red => Some(row_counter * 2),
        }
    }
}


// To do: Add tests
#[cfg(test)]
mod tests {
    use super::*;
    const BASE: u32 = 2;

    // #[test]
    // fn turn_column_to_encoded_gamestate_given_correct_tuples_blue_return_encoded_move() {
    //     assert_eq!(turn_column_to_encoded_gamestate((1,1), &PlayerColor::Blue), 1);
    //     assert_eq!(turn_column_to_encoded_gamestate((2,1), &PlayerColor::Blue), BASE.pow(8));
    //     assert_eq!(turn_column_to_encoded_gamestate((2,2), &PlayerColor::Blue), BASE.pow(8 + 2));
    //     assert_eq!(turn_column_to_encoded_gamestate((3,3), &PlayerColor::Blue), BASE.pow(2 * 8 + 2* 2));
    //     assert_eq!(turn_column_to_encoded_gamestate((4,4), &PlayerColor::Blue), BASE.pow(3 * 8 + 3* 2));
    //     assert_eq!(turn_column_to_encoded_gamestate((4,1), &PlayerColor::Blue), BASE.pow(3 * 8));
    //     assert_eq!(turn_column_to_encoded_gamestate((1,4), &PlayerColor::Blue), BASE.pow(3* 2));
    // }

    // #[test]
    // fn turn_column_to_encoded_gamestate_given_correct_tuples_red_return_encoded_move() {
    //     assert_eq!(turn_column_to_encoded_gamestate((1,1), &PlayerColor::Red), 2);
    //     assert_eq!(turn_column_to_encoded_gamestate((2,1), &PlayerColor::Red), BASE.pow(8 + 1));
    //     assert_eq!(turn_column_to_encoded_gamestate((2,2), &PlayerColor::Red), BASE.pow(8 + 2 + 1));
    //     assert_eq!(turn_column_to_encoded_gamestate((3,3), &PlayerColor::Red), BASE.pow(2 * 8 + 2* 2 + 1));
    // }
}