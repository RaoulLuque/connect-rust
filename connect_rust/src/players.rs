pub mod bruteforce;
pub mod bruteforce_n_percent;
pub mod monte_carlo;
pub mod random;
pub mod random_glowed_up;

use bruteforce_n_percent::PossiblePercentages;
use std::{str::FromStr, time::Duration};

/// Enum for types of players, e.g. bruteforce, random or monte carlo
pub enum Player {
    Bruteforce,
    MonteCarlo,
    Random,
    RandomGlowedUp,
    BruteforceNPercent(PossiblePercentages),
}

impl FromStr for Player {
    type Err = std::string::ParseError;

    /// Turns a string of the form "engine" into the corresponding Engine if spelled correctly as
    /// seen in the function body
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
    /// Let's engine make a move given the current gamestate. Returns a 4-tuple with:
    /// - The next gamestate (the previous gamestate & the move the engine wants to make)
    /// - The evaluation of the next gamestate. If engine doesn't implement this option 0 is
    /// returned instead
    /// - The number of visited nodes/gamestates while calculating the next move. If engine doesn't implement this option 0 is
    /// returned instead
    /// - The time it took to compute as a std::time::Duration
    pub fn make_move(&self, gamestate: u128) -> (u128, i8, u32, Duration) {
        match &self {
            &Player::Bruteforce => bruteforce::Engine::make_move(gamestate, true),
            &Player::MonteCarlo => monte_carlo::Engine::make_move(gamestate, 2000),
            &Player::Random => random::Engine::make_move(gamestate),
            &Player::RandomGlowedUp => random_glowed_up::Engine::make_move(gamestate),
            &Player::BruteforceNPercent(possible_percentage) => {
                bruteforce_n_percent::Engine::make_move(gamestate, possible_percentage)
            }
        }
    }
}
