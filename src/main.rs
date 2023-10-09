mod logging;
mod gamestate_helpers;
mod players;
mod setup;
mod multithreading;


use std::thread::current;
use gamestate_helpers::PlayerColor;
use logging::Logger;
use players::Player;

use std::time::Instant;

/// Plays the connect four game and asks which players/engines should play against which.
/// If human players are playing, gamestates are shown in console directly otherwise they are visible in logs
fn main() {
    // Printing explanation of game
    setup::print_introduction();

    // Choosing who to play as/against (choosing players)
    let (mut player_blue, mut player_red, elapsed_blue, elapsed_red) =  setup::read_in_players();

    // Setup of variables for game
    let mut current_gamestate: u128 = 0;
    let mut turn_number: usize = 0;
    let mut log = Logger::new();
    let mut winner: Option<PlayerColor> = None;
    let mut elapsed: u128 = 1000;

    log.log_initialization(elapsed_blue, elapsed_red).expect("Logging should be possible");

    let thread_identifier = match (&player_blue, &player_red) {
        (Player::Human(e), Player::Montecarlo(f)) => Some(true),
        (Player::Montecarlo(e), Player::Human(f)) => Some(false),
        _ => None,
    };

    // Running the game
    while winner == None && !gamestate_helpers::is_full(current_gamestate) {
        // Increment turn number
        turn_number += 1;

        // Timing how long it took to calculate turn
        let timer = Instant::now();

        let player_blue = &mut player_blue;
        let player_red = &mut player_red;

        let next_move = match (thread_identifier, gamestate_helpers::whos_turn_is_it_turn_number(turn_number)) {
            (Some(true), PlayerColor::Blue) => {
                multithreading::calculate_montecarlo_while_human_chooses_turn(player_red, player_blue, current_gamestate)
            },

            (Some(false), PlayerColor::Red) => {
                multithreading::calculate_montecarlo_while_human_chooses_turn(player_blue, player_red, current_gamestate)
            },
            _ => {
                // Chooses the next move based on the current player who's turn it is and the engine chosen
                match gamestate_helpers::whos_turn_is_it_turn_number(turn_number) {
                    PlayerColor::Blue => player_blue.make_move(current_gamestate, elapsed),
                    PlayerColor::Red => player_red.make_move(current_gamestate, elapsed),
                }
            },
        };

        // Taking time
        elapsed = timer.elapsed().as_millis();

        // Checking whether move was valid
        if !crate::gamestate_helpers::is_allowed_move(current_gamestate, next_move, turn_number) {
            // Move is invalid, logged and game is stopped
            log.log_invalid_turn(turn_number, current_gamestate, next_move).expect("Logging should be possible");
            winner = match gamestate_helpers::whos_turn_is_it_turn_number(turn_number) {
                PlayerColor::Blue => Some(PlayerColor::Red),
                PlayerColor::Red => Some(PlayerColor::Blue),
            };
            break;

        } else {
            // Move is valid and is logged
            current_gamestate = current_gamestate | next_move;
            log.log_turn(turn_number, current_gamestate, elapsed).expect("Logging should be possible");

        }
        // Set winner for checking if game over?
        winner = gamestate_helpers::is_won(current_gamestate);
    }
    
    // Log who has won
    log.log_winner(&winner, turn_number).expect("Logging should be possible");

    // Declare winner
    setup::declare_winner(&winner, turn_number, current_gamestate);

}
