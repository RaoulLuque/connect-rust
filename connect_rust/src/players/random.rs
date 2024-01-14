use std::time::Duration;

use crate::helpers::moves::possible_next_gamestates;

/// The random engine plays completely random. Nonetheless, according to the rules of course.
pub struct Engine;

use rand::seq::SliceRandom;

impl Engine {
    /// Returns a valid random move given an encoded gamestate
    pub fn make_move(current_gamestate: u128) -> (u128, i8, u32, Duration) {
        let possible_move: Vec<u128> = possible_next_gamestates(current_gamestate).collect();
        (
            *possible_move.choose(&mut rand::thread_rng()).unwrap(),
            0,
            0,
            Duration::new(0,0),
        )
    }
}
