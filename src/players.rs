pub mod bruteforce;
pub mod monte_carlo;
pub mod random;
pub mod random_glowed_up;

use std::str::FromStr;

/// Enum for types of players, e.g. Bruteforce, Human or Monte-Carlo
pub enum Player {
    /// Enum variant for human playing
    Bruteforce,
    Montecarlo,
    Random,
    RandomGlowedUp,
}

impl FromStr for Player {
    type Err = std::string::ParseError;

    fn from_str(player_as_string: &str) -> Result<Self, Self::Err> {
        match player_as_string {
            "Random" => Ok(Player::Random),
            "Random*" => Ok(Player::RandomGlowedUp),
            "Monte Carlo" => Ok(Player::Montecarlo),
            "Bruteforce" => Ok(Player::Bruteforce),
            _ => Ok(Player::Random),
        }
    }
}

impl Player {
    /// Returns
    /// u128 : gamestate
    /// i8  : score of gamestate
    /// u32  : number of visited nodes
    /// u128  : computation time in microseconds
    pub fn make_move(&self, gamestate: u128, elapsed: u128) -> (u128, i8, u32, u128) {
        match &self {
            &Player::Bruteforce => bruteforce::Engine::make_move(gamestate, true),
            &Player::Montecarlo => monte_carlo::Engine::make_move(gamestate, elapsed),
            &Player::Random => random::Engine::make_move(gamestate),
            &Player::RandomGlowedUp => random_glowed_up::Engine::make_move(gamestate),
        }
    }
}
