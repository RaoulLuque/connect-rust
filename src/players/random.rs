use crate::gamestate_helpers::possible_next_gamestates;
use crate::players::CanPlay;
pub struct RandomPlayer;

pub fn make_move(current_gamestate: u128) -> u128 {
    possible_next_gamestates(current_gamestate).last().unwrap()
}
