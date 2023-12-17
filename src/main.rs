mod gamestate_helpers;
mod html_template;
mod logging;
mod multithreading;
mod players;
mod setup;

use gamestate_helpers::{
    encoded_gamestate_to_str, is_allowed_move, possible_next_gamestates,
    turn_column_to_encoded_gamestate, PlayerColor,
};
use html_template::START_PAGE_TEMPLATE;
use logging::Logger;
use players::Player;

use axum::{
    extract::Form,
    response::Html,
    routing::{get, post},
    Router,
};
use minijinja::render;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::players::random::make_move;

const CALCULATE_WHILE_HUMAN_IS_CHOOSING_NEXT_TURN: bool = false;

#[derive(Debug, Deserialize)]
struct GameMoveInput {
    current_gamestate: Option<String>,
    column: Option<u32>,
}

#[derive(Debug, Serialize)]
struct GameMoveOutput {
    board_as_string: String,
    current_gamestate_encoded: String,
    game_not_over: bool,
    who_won: Option<PlayerColor>,
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(start_page))
        .route("/", post(accept_move));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn accept_move(Form(turn): Form<GameMoveInput>) -> Html<String> {
    let (column_player_wants_to_play, current_gamestate) = read_in_response(turn);

    let (new_gamestate, move_was_valid) =
        match calculate_new_gamestate(column_player_wants_to_play, current_gamestate) {
            Some(i) => (i, true),
            None => (
                possible_next_gamestates(current_gamestate).last().unwrap() | current_gamestate,
                false,
            ),
        };

    let response = generate_response(new_gamestate, move_was_valid);
    let response = generate_response_string(response);

    Html(response)
}

fn generate_response_string(response: GameMoveOutput) -> String {
    match response.game_not_over {
        true => render!(START_PAGE_TEMPLATE, turn => response),
        false => render!(START_PAGE_TEMPLATE, turn => response, over => true),
    }
}

fn read_in_response(turn: GameMoveInput) -> (u32, u128) {
    match (turn.column, turn.current_gamestate) {
        (Some(column_as_integer), Some(current_gamestate_as_string)) => (
            column_as_integer,
            current_gamestate_as_string
                .parse::<u128>()
                .expect("Current gamestate should be an u128"),
        ),
        (_, Some(current_gamestate_as_string)) => (
            0,
            current_gamestate_as_string
                .parse::<u128>()
                .expect("Current gamestate should be an u128"),
        ),
        _ => (0, 0),
    }
}

fn calculate_new_gamestate(
    column_player_wants_to_play: u32,
    current_gamestate: u128,
) -> Option<u128> {
    match turn_column_to_encoded_gamestate(
        current_gamestate,
        column_player_wants_to_play,
        &PlayerColor::Blue,
    ) {
        Some(i) => Some(i | current_gamestate),
        // Possible_next_gamestates should not be empty at this point
        None => None,
    }
}

fn generate_response(current_gamestate: u128, move_was_valid: bool) -> GameMoveOutput {
    if gamestate_helpers::is_over(current_gamestate) {
        GameMoveOutput {
            board_as_string: encoded_gamestate_as_string_for_web(current_gamestate, move_was_valid),
            current_gamestate_encoded: format!("{}", 0),
            game_not_over: false,
            who_won: gamestate_helpers::is_won(current_gamestate),
        }
    } else {
        let new_gamestate = make_move(current_gamestate);

        let response: GameMoveOutput = GameMoveOutput {
            board_as_string: encoded_gamestate_as_string_for_web(new_gamestate, move_was_valid),
            current_gamestate_encoded: format!("{}", new_gamestate),
            game_not_over: true,
            who_won: None,
        };

        response
    }
}

fn encoded_gamestate_as_string_for_web(gamestate: u128, move_was_valid: bool) -> String {
    let board = format!(
        "Current board: <br> {}",
        encoded_gamestate_to_str(gamestate, "<br>")
    );

    match move_was_valid {
        true => board,
        false => format!(
            "Your move was invalid. We chose the last possible column: <br> {}",
            board
        ),
    }
}

async fn start_page() -> Html<String> {
    let r = render!(START_PAGE_TEMPLATE);
    Html(r)
}

/// Plays the connect four game and asks which players/engines should play against which.
/// If human players are playing, gamestates are shown in console directly otherwise they are visible in logs
fn other_main() {
    // Printing explanation of game
    setup::print_introduction();

    // Choosing who to play as/against (choosing players)
    let (mut player_blue, mut player_red, elapsed_blue, elapsed_red) = setup::read_in_players();

    // Setup of variables for game
    let mut current_gamestate: u128 = 0;
    let mut turn_number: usize = 0;
    let mut log = Logger::new();
    let mut winner: Option<PlayerColor> = None;
    let mut elapsed: u128 = 1000;

    // Log the initialization of the game
    log.log_initialization(elapsed_blue, elapsed_red)
        .expect("Logging should be possible");

    // Check if multithreading is necessary in case human is playing against montecarlo
    let thread_identifier = match (&player_blue, &player_red) {
        (Player::Human(_), Player::Montecarlo(_)) => Some(true),
        (Player::Montecarlo(_), Player::Human(_)) => Some(false),
        _ => None,
    };

    // Running the game
    while winner == None && !gamestate_helpers::is_full(current_gamestate) {
        // Increment turn number
        turn_number += 1;

        // Timing how long it took to calculate turn
        let timer = Instant::now();

        // Make mutable references of players in order to move those references into other threads
        let player_blue = &mut player_blue;
        let player_red = &mut player_red;

        let next_move = match (
            thread_identifier,
            gamestate_helpers::whos_turn_is_it_turn_number(turn_number),
            CALCULATE_WHILE_HUMAN_IS_CHOOSING_NEXT_TURN,
        ) {
            (Some(true), PlayerColor::Blue, true) => {
                multithreading::calculate_montecarlo_while_human_chooses_turn(
                    player_red,
                    player_blue,
                    current_gamestate,
                )
            }

            (Some(false), PlayerColor::Red, true) => {
                multithreading::calculate_montecarlo_while_human_chooses_turn(
                    player_blue,
                    player_red,
                    current_gamestate,
                )
            }
            _ => {
                // Chooses the next move based on the current player who's turn it is and the engine chosen
                match gamestate_helpers::whos_turn_is_it_turn_number(turn_number) {
                    PlayerColor::Blue => player_blue.make_move(current_gamestate, elapsed),
                    PlayerColor::Red => player_red.make_move(current_gamestate, elapsed),
                }
            }
        };

        // Taking time
        elapsed = timer.elapsed().as_millis();

        // Checking whether move was valid
        if !crate::gamestate_helpers::is_allowed_move(current_gamestate, next_move, turn_number) {
            // Move is invalid, logged and game is stopped
            log.log_invalid_turn(turn_number, current_gamestate, next_move)
                .expect("Logging should be possible");

            winner = match gamestate_helpers::whos_turn_is_it_turn_number(turn_number) {
                PlayerColor::Blue => Some(PlayerColor::Red),
                PlayerColor::Red => Some(PlayerColor::Blue),
            };
            break;
        } else {
            // Move is valid and is logged
            current_gamestate = current_gamestate | next_move;
            log.log_turn(turn_number, current_gamestate, elapsed)
                .expect("Logging should be possible");
        }
        // Set winner for checking if game over?
        winner = gamestate_helpers::is_won(current_gamestate);
    }

    // Log who has won
    log.log_winner(&winner, turn_number)
        .expect("Logging should be possible");

    // Declare winner
    setup::declare_winner(&winner, turn_number, current_gamestate);
}
