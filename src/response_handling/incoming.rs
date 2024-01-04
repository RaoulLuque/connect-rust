use super::*;

use axum::extract::Form;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct GameMoveInput {
    // Gamestates separated by '#' from current to last gamestate in column encoding
    current_and_previous_gamestates: Option<String>,
    column: Option<u32>,
    engine: Option<String>,
}

pub async fn accept_move(Form(turn): Form<GameMoveInput>) -> Html<String> {
    let (column_player_wants_to_play, current_and_previous_gamestates, engine_to_play_against) =
        read_in_response(turn);

    outgoing::generate_response(
        current_and_previous_gamestates,
        column_player_wants_to_play,
        engine_to_play_against,
    )
}

fn read_in_response(turn: GameMoveInput) -> (u32, String, Player) {
    match (
        turn.column,
        turn.current_and_previous_gamestates,
        turn.engine,
    ) {
        (
            Some(column_as_integer),
            Some(current_and_previous_gamestates),
            Some(engine_as_string),
        ) => (
            column_as_integer,
            current_and_previous_gamestates,
            Player::from_str(&engine_as_string).expect("From str should always return ok"),
        ),
        (_, Some(current_and_previous_gamestates), Some(engine_as_string)) => (
            0,
            current_and_previous_gamestates,
            Player::from_str(&engine_as_string).expect("From str should always return ok"),
        ),
        (_, Some(current_and_previous_gamestates), _) => {
            (0, current_and_previous_gamestates, Player::Random)
        }
        _ => (0, "".to_owned(), Player::Random),
    }
}
