pub mod bruteforce;
pub mod human;
pub mod monte_carlo;
pub mod random;

use std::str::FromStr;
use std::sync::mpsc::Receiver;
use std::time::Instant;

use crate::gamestate_helpers::PlayerColor;

/// Enum for types of players, e.g. Bruteforce, Human or Monte-Carlo
pub enum Player {
    /// Enum variant for human playing
    Human(human::Engine),
    Bruteforce(bruteforce::Engine),
    Montecarlo(monte_carlo::Engine),
    Random(random::Random),
}

impl FromStr for Player {
    type Err = std::string::ParseError;

    fn from_str(player_as_string: &str) -> Result<Self, Self::Err> {
        match player_as_string {
            "Random" => Ok(Player::Random(random::Random)),
            "Monte Carlo" => Ok(Player::Montecarlo(monte_carlo::Engine::new(
                PlayerColor::Red,
            ))),
            _ => Ok(Player::Random(random::Random)),
        }
    }
}

impl Player {
    pub fn make_move(&mut self, gamestate: u128, elapsed: u128) -> u128 {
        match self {
            Player::Human(e) => e.make_move(gamestate),
            Player::Bruteforce(e) => e.make_move(gamestate),
            Player::Montecarlo(e) => e.make_move(gamestate, elapsed),
            Player::Random(e) => e.make_move(gamestate),
        }
    }

    pub fn monte_carlo_intermission_loop(
        &mut self,
        gamestate: u128,
        timer: Instant,
        time: u128,
        rx: Receiver<bool>,
    ) {
        match self {
            Player::Montecarlo(e) => e.monte_carlo_loop(gamestate, timer, time, rx),
            _ => {}
        }
    }
}
