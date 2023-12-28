use super::*;
use crate::players::random;
use crate::Player;

use axum::extract::Form;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct GameMoveInput {
    current_gamestate: Option<String>,
    column: Option<u32>,
    engine: Option<String>,
}

pub async fn accept_move(Form(turn): Form<GameMoveInput>) -> Html<String> {
    let (column_player_wants_to_play, current_gamestate, engine_to_play_against) =
        read_in_response(turn);

    outgoing::generate_response(
        current_gamestate,
        column_player_wants_to_play,
        engine_to_play_against,
    )
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
