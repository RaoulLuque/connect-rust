use std::time::Duration;

use super::bruteforce::Engine as Engine_bruteforce;
use super::random_glowed_up::Engine as Engine_random_glowed_up;

use rand::{thread_rng, Rng};

pub enum PossiblePercentages {
    TwentyFive,
    Fifty,
    SeventyFive,
}

/// The bruteforce N% engine plays as Bruteforce N% of the time.
/// Otherwise the moves are made according to the random* engine.
pub struct Engine;

impl Engine {
    /// Returns a move with chance_of_playing_as_bruteforce being the chance bruteforce move is used
    /// given an encoded gamestate. Otherwise plays as random*. In the first turn the bruteforce
    /// move will always be chosen.
    pub fn make_move(
        current_gamestate: u128,
        chance_of_playing_as_bruteforce: &PossiblePercentages,
    ) -> (u128, i8, u32, Duration) {
        let chance_of_playing_as_bruteforce: f64 = match chance_of_playing_as_bruteforce {
            PossiblePercentages::TwentyFive => 25.0,
            PossiblePercentages::Fifty => 50.0,
            PossiblePercentages::SeventyFive => 75.0,
        };

        let move_from_random = Engine_random_glowed_up::make_move(current_gamestate).0;

        let (move_from_bruteforce, evaluation, number_of_visited_nodes, time) =
            Engine_bruteforce::make_move(current_gamestate, true);

        let mut rng = thread_rng();
        let mut make_bruteforce_move = rng.gen_bool(chance_of_playing_as_bruteforce / 100.0);

        // Play as bruteforce if its still the first turn
        if current_gamestate.count_ones() < 2 {
            make_bruteforce_move = true;
        }

        match make_bruteforce_move {
            true => (
                move_from_bruteforce,
                evaluation,
                number_of_visited_nodes,
                time,
            ),
            false => (move_from_random, evaluation, number_of_visited_nodes, time),
        }
    }
}
