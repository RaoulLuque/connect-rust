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
use players::{random, Player};

use axum::{
    extract::Form,
    response::Html,
    routing::{get, post},
    Router,
};
use minijinja::render;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::time::Instant;

const CALCULATE_WHILE_HUMAN_IS_CHOOSING_NEXT_TURN: bool = false;

#[derive(Debug, Deserialize)]
struct GameMoveInput {
    current_gamestate: Option<String>,
    column: Option<u32>,
    engine: Option<String>,
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
    let (column_player_wants_to_play, current_gamestate, engine_to_play_against) =
        read_in_response(turn);

    let (new_gamestate, move_was_valid) =
        match calculate_new_gamestate(column_player_wants_to_play, current_gamestate) {
            Some(i) => (i, true),
            None => (
                possible_next_gamestates(current_gamestate).last().unwrap() | current_gamestate,
                false,
            ),
        };

    let response = generate_response(new_gamestate, move_was_valid, engine_to_play_against);
    let response = generate_response_string(response);

    Html(response)
}

fn generate_response_string(response: GameMoveOutput) -> String {
    match response.game_not_over {
        true => render!(START_PAGE_TEMPLATE, turn => response),
        false => render!(START_PAGE_TEMPLATE, turn => response, over => true),
    }
}

fn read_in_response(turn: GameMoveInput) -> (u32, u128, Player) {
    match (turn.column, turn.current_gamestate, turn.engine) {
        (Some(column_as_integer), Some(current_gamestate_as_string), Some(engine_as_string)) => (
            column_as_integer,
            current_gamestate_as_string
                .parse::<u128>()
                .expect("Current gamestate should be an u128"),
            Player::from_str(&engine_as_string).expect("From str should always return ok"),
        ),
        (_, Some(current_gamestate_as_string), Some(engine_as_string)) => (
            0,
            current_gamestate_as_string
                .parse::<u128>()
                .expect("Current gamestate should be an u128"),
            Player::from_str(&engine_as_string).expect("From str should always return ok"),
        ),
        (_, Some(current_gamestate_as_string), _) => (
            0,
            current_gamestate_as_string
                .parse::<u128>()
                .expect("Current gamestate should be an u128"),
            Player::Random(random::Random),
        ),
        _ => (0, 0, Player::Random(random::Random)),
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

fn generate_response(
    current_gamestate: u128,
    move_was_valid: bool,
    mut engine_to_play_against: Player,
) -> GameMoveOutput {
    if gamestate_helpers::is_over(current_gamestate) {
        GameMoveOutput {
            board_as_string: encoded_gamestate_as_string_for_web(current_gamestate, move_was_valid),
            current_gamestate_encoded: format!("{}", 0),
            game_not_over: false,
            who_won: gamestate_helpers::is_won(current_gamestate),
        }
    } else {
        let new_gamestate = engine_to_play_against.make_move(current_gamestate, 0);
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
