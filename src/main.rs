mod logging;
mod gamestate_helpers;
mod players;
mod setup;

use logging::Logger;
use players::{Player,human};
use std::{time::Instant};



/// Plays the connect four game and asks which players/engines should play against which.
/// If human players are playing, gamestates are shown in console directly otherwise they are visible in logs
fn main() {
    // Printing explanation of game
    setup::print_introduction();

    // Choosing who to play as/against (choosing players)
    let (mut player_blue, mut player_red) =  setup::read_in_players();

    // Setup of variables for game
    let mut current_gamestate: u32 = 0;
    let mut turn_number: usize = 0;
    let mut log = Logger::new();


    // Running the game
    while gamestate_helpers::is_won(current_gamestate) == None && !gamestate_helpers::is_over(current_gamestate) {
        // Increment turn number
        turn_number += 1;

        // Timing how long it took to calculate turn
        let timer = Instant::now();

        // Chooses the next move based on the current player who's turn it is and the engine chosen
        let mut next_move: u32 =
            {if turn_number % 2 == 0 {
                player_blue.make_move(current_gamestate)
            } else {
                player_blue.make_move(current_gamestate)
            }
            };

        // Taking time
        let elapsed = timer.elapsed();
        let elapsed = elapsed.as_millis();

        // Checking whether move was valid
        if !crate::gamestate_helpers::is_allowed_move(current_gamestate, next_move, turn_number) {
            // Move is invalid, logged and game is stopped
            log.log_invalid_turn(turn_number, current_gamestate, next_move).expect("Logging should be possible");
            break;

        } else {
            // Move is valid and is logged
            current_gamestate = current_gamestate | next_move;

        }
    }
    
    log.log_winner(gamestate_helpers::is_won(current_gamestate), turn_number);
    setup::declare_winner(gamestate_helpers::is_won(current_gamestate), turn_number);

}
