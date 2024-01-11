use crate::helpers::moves::possible_next_gamestates;
pub struct Engine;

use rand::seq::SliceRandom;

impl Engine {
    pub fn make_move(current_gamestate: u128) -> (u128, i8, u32, u128) {
        let possible_move: Vec<u128> = possible_next_gamestates(current_gamestate).collect();
        (
            *possible_move.choose(&mut rand::thread_rng()).unwrap(),
            0,
            0,
            0,
        )
    }
}
