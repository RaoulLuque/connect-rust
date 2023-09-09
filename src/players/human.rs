enum Value {
    Uint(u32),
}

use crate::gamestate_helpers::{PlayerColor, self};
use super::Player;

pub struct Engine {
    color: PlayerColor,
}

impl Engine {
    /// Creates engine for human player
    /// to do: Implement
    pub fn new(color: PlayerColor) -> Engine{
        Engine{color}
    }

    /// Asks the user to input the next move of the human player after displaying the current gamestate
    /// to do: Implement
    pub fn make_move(&self, gamestate: u32) -> u32 {
        self.print_current_gamestate(gamestate);

        let next_move: (u32, u32) = ask_for_next_move();

        turn_tuple_to_unsigned_integer(next_move, &self.color)
    }

    /// Prints the current gamestate for the user
    fn print_current_gamestate(&self, gamestate: u32) {
        match self.color {
            PlayerColor::Blue => println!("It is blue's turn!"),
            PlayerColor::Red => println!("It iss red's turn!"),
        };
    
        let current_gamestate_as_string: String = gamestate_helpers::encoded_gamestate_to_str(gamestate);
        println!("The current gamestate is:");
        println!("{}", current_gamestate_as_string);
    }
}


/// Asks the user to input the coordinates of the next token that is supposed to be played
/// To do: ask again if Turn invalid
fn ask_for_next_move() -> (u32, u32) {
    let mut input = String::new();
    let mut column: u32 = 0;
    let mut row: u32 = 0;

    // Infinite loop getting the column of the next move
    loop {
        println!("Please enter the number of the column you want to place your token in:");
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(error) => panic!("Problem reading in input: {:?}", error),
        }

        // Matching the input to the 
        if let Some(val) = parse_string_tuple(input.trim()) {
            column = val;
            break;
        } else {
            println!("Invalid column input please try again!");
        }
    }

    let mut input = String::new();
    // Infinite loop getting the row of the next move
    loop {
        println!("Please enter the number of the row you want to place your token in:");
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(error) => panic!("Problem reading in input: {:?}", error),
        }

        // Matching the input to the 
        if let Some(val) = parse_string_tuple(input.trim()) {
            row = val;
            break;
        } else {
            println!("Invalid row input please try again!");
        }
    }

    (row, column)
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
fn turn_tuple_to_unsigned_integer((row, column): (u32, u32), color: &PlayerColor) -> u32 {
    let base: u32 = 2;
    match color {
        &PlayerColor::Blue => base.pow(8* (row-1) + (column-1) * 2),
        &PlayerColor::Red => base.pow(8* (row-1) + (column-1) * 2 + 1),
    }
}

// To do: Add tests
#[cfg(test)]
mod tests {
    use super::*;
    const BASE: u32 = 2;

    #[test]
    fn turn_tuple_to_unsigned_integer_given_correct_tuples_blue_return_encoded_move() {
        assert_eq!(turn_tuple_to_unsigned_integer((1,1), &PlayerColor::Blue), 1);
        assert_eq!(turn_tuple_to_unsigned_integer((2,1), &PlayerColor::Blue), BASE.pow(8));
        assert_eq!(turn_tuple_to_unsigned_integer((2,2), &PlayerColor::Blue), BASE.pow(8 + 2));
        assert_eq!(turn_tuple_to_unsigned_integer((3,3), &PlayerColor::Blue), BASE.pow(2 * 8 + 2* 2));
        assert_eq!(turn_tuple_to_unsigned_integer((4,4), &PlayerColor::Blue), BASE.pow(3 * 8 + 3* 2));
        assert_eq!(turn_tuple_to_unsigned_integer((4,1), &PlayerColor::Blue), BASE.pow(3 * 8));
        assert_eq!(turn_tuple_to_unsigned_integer((1,4), &PlayerColor::Blue), BASE.pow(3* 2));
    }

    #[test]
    fn turn_tuple_to_unsigned_integer_given_correct_tuples_red_return_encoded_move() {
        assert_eq!(turn_tuple_to_unsigned_integer((1,1), &PlayerColor::Red), 2);
        assert_eq!(turn_tuple_to_unsigned_integer((2,1), &PlayerColor::Red), BASE.pow(8 + 1));
        assert_eq!(turn_tuple_to_unsigned_integer((2,2), &PlayerColor::Red), BASE.pow(8 + 2 + 1));
        assert_eq!(turn_tuple_to_unsigned_integer((3,3), &PlayerColor::Red), BASE.pow(2 * 8 + 2* 2 + 1));
    }
}