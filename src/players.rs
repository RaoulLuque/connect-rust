pub mod bruteforce;
pub mod bruteforce_n_percent;
pub mod monte_carlo;
pub mod random;
pub mod random_glowed_up;

use bruteforce_n_percent::PossiblePercentages;
use std::str::FromStr;

/// Enum for types of players, e.g. Bruteforce, Random or Monte-Carlo
pub enum Player {
    Bruteforce,
    MonteCarlo,
    Random,
    RandomGlowedUp,
    BruteforceNPercent(PossiblePercentages),
}

impl FromStr for Player {
    type Err = std::string::ParseError;

    fn from_str(player_as_string: &str) -> Result<Self, Self::Err> {
        match player_as_string {
            "Random" => Ok(Player::Random),
            "Random*" => Ok(Player::RandomGlowedUp),
            "Monte Carlo" => Ok(Player::MonteCarlo),
            "Bruteforce" => Ok(Player::Bruteforce),
            "Bruteforce 75%" => Ok(Player::BruteforceNPercent(PossiblePercentages::SeventyFive)),
            "Bruteforce 50%" => Ok(Player::BruteforceNPercent(PossiblePercentages::Fifty)),
            "Bruteforce 25%" => Ok(Player::BruteforceNPercent(PossiblePercentages::TwentyFive)),
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
            &Player::MonteCarlo => monte_carlo::Engine::make_move(gamestate, elapsed),
            &Player::Random => random::Engine::make_move(gamestate),
            &Player::RandomGlowedUp => random_glowed_up::Engine::make_move(gamestate),
            &Player::BruteforceNPercent(possible_percentage) => {
                bruteforce_n_percent::Engine::make_move(gamestate, possible_percentage)
            }
        }
    }
}
