use crate::gamestate_helpers::possible_next_gamestates;
pub struct Random;

impl Random {
    pub fn make_move(&self, current_gamestate: u128) -> u128 {
        possible_next_gamestates(current_gamestate).last().unwrap()
    }
}
