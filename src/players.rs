pub mod bruteforce;
pub mod monte_carlo;
pub mod random;

use std::str::FromStr;

/// Enum for types of players, e.g. Bruteforce, Human or Monte-Carlo
pub enum Player {
    /// Enum variant for human playing
    Bruteforce,
    Montecarlo,
    Random,
}

impl FromStr for Player {
    type Err = std::string::ParseError;

    fn from_str(player_as_string: &str) -> Result<Self, Self::Err> {
        match player_as_string {
            "Random" => Ok(Player::Random),
            "Monte Carlo" => Ok(Player::Montecarlo),
            _ => Ok(Player::Random),
        }
    }
}

impl Player {
    pub fn make_move(&self, gamestate: u128, elapsed: u128) -> (u128, i32, u32, u32) {
        match &self {
            &Player::Bruteforce => bruteforce::Engine::make_move(gamestate),
            &Player::Montecarlo => monte_carlo::Engine::make_move(gamestate, elapsed),
            &Player::Random => random::Engine::make_move(gamestate),
        }
    }
}
