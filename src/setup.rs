use std::{io, time::Instant};
use crate::players::{Player, human, bruteforce, monte_carlo};
use crate::gamestate_helpers::{PlayerColor, encoded_gamestate_to_str};

pub fn print_introduction() {
    // Introduction
    println!("Welcome to connect-rust, a connect-four implementation in rust.");
    println!("The options are to play oneself against oneself or against one of the bots or having the bots play against each other.");
    println!("So far the bots are referring to: Bruteforce (B) or Montecarlo (M)");
}

pub fn read_in_players() -> (Player, Player, u128, u128) {
    println!("Please choose the gamemode you want to play by writing XvY. X and Y standing for the players. Playing yourself is signaled with an H for human and the different bots are abbreviated above");
    println!("E.g.: If you want to play against the bruteforce bot write 'HvB'.");
    println!("You can also write 'BvB' or 'HvH' for the bruteforce bots playing against each other and you playing against another human.");
    println!("Also note that with HvM the human would be starting (playing as blue) and with MvH the other way around.");

    // Infinite loop checking the input for a valid input
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(error) => panic!("Problem reading in input: {:?}", error),
        }

        // Players
        let player_one_blue: Player;
        let player_two_red: Player;

        // Elapsed
        let mut elapsed_time_one_blue: u128 = 0;
        let mut elapsed_time_two_red: u128 = 0;


        input = input.trim().to_owned();

        // Checking the inputs for the engine abbreviations
        if input.starts_with("H") {
            player_one_blue = Player::Human(human::Engine::new(PlayerColor::Blue));
        } else if input.starts_with("B") {
            let timer = Instant::now();
            player_one_blue = Player::Bruteforce(bruteforce::Engine::new(PlayerColor::Blue));
            let elapsed = timer.elapsed();
            elapsed_time_one_blue = elapsed.as_millis();
        } else if input.starts_with("M") {
            let timer = Instant::now();
            player_one_blue = Player::Montecarlo(monte_carlo::Engine::new(PlayerColor::Blue));
            let elapsed = timer.elapsed();
            elapsed_time_one_blue = elapsed.as_millis();
        } else {
            println!("Not a valid input for first player! Please try again:");
            continue;
        }

        if input.ends_with("H") {
            player_two_red = Player::Human(human::Engine::new(PlayerColor::Red));
        } else if input.ends_with("B") {
            let timer = Instant::now();
            player_two_red = Player::Bruteforce(bruteforce::Engine::new(PlayerColor::Red));
            let elapsed = timer.elapsed();
            elapsed_time_two_red = elapsed.as_millis();
        } else if input.ends_with("M") {
            let timer = Instant::now();
            player_two_red = Player::Montecarlo(monte_carlo::Engine::new(PlayerColor::Red));
            let elapsed = timer.elapsed();
            elapsed_time_two_red = elapsed.as_millis();
        } else {
            println!("Not a valid input for second player! Please try again:");
            continue;
        }
        
        return (player_one_blue, player_two_red, elapsed_time_one_blue, elapsed_time_two_red)
    } 
}

pub fn declare_winner(winner: &Option<PlayerColor>, turn_number: usize, gamestate: u32) {
    let winner_string: &str = match winner {
        &Some(PlayerColor::Blue) => "Blue",
        &Some(PlayerColor::Red) => "Red",
        &None => "Nobody",
    };
    println!("Congratulations: {} has won the game after {} turns! \n \n", winner_string, turn_number);

    let current_gamestate_as_string: String = encoded_gamestate_to_str(gamestate);
    println!("The final gamestate is:");
    println!("{}", current_gamestate_as_string);
}

// to do: add tests
#[cfg(test)]
mod tests {
    
}
