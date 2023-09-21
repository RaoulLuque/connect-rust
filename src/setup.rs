use std::io;
use crate::players::{Player, human};
use crate::gamestate_helpers::PlayerColor;

pub fn print_introduction() {
    // Introduction
    println!("Welcome to connect-rust, a connect-four implementation in rust.");
    println!("The options are to play oneself against oneself or against one of the bots or having the bots play against each other.");
    println!("So far the bots are referring to: None");
}

pub fn read_in_players() -> (Player, Player) {
    println!("Please choose the gamemode you want to play by writing XvY. X and Y standing for the players. Playing yourself is signaled with an H for human and the different bots are abbreviated above");
    println!("E.g.: If you want to play against the bruteforce bot write 'HvB'.");
    println!("You can also write 'BvB' or 'HvH' for the bruteforce bots playing against each other and you playing against another human.");
    println!("Also note that with HvB the human would be starting (playing as blue) and with BvP the other way around.");
    let mut input = String::new();

    // Infinite loop checking the input for a valid input
    loop {
        input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(error) => panic!("Problem reading in input: {:?}", error),
        }

        // Matching the input to the 
        match input.trim() {
            "HvH" => {return (Player::Human(human::Engine::new(PlayerColor::Blue)), Player::Human(human::Engine::new(PlayerColor::Red)))},
            _ => {println!("Not a valid input! Please try again:");},
        }
    }
}

pub fn declare_winner(winner: &Option<PlayerColor>, turn_number: usize) {
    let winner_string: &str = match winner {
        &Some(PlayerColor::Blue) => "Blue",
        &Some(PlayerColor::Red) => "Red",
        &None => "Nobody",
    };
    println!("Congratulations: {} has won the game after {} turns!", winner_string, turn_number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading_in_players() {
       let (player_blue, player_red) = read_in_players();

    }
}
