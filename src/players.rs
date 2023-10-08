pub mod human;
pub mod bruteforce;
pub mod monte_carlo;

/// Enum for types of players, e.g. Bruteforce, Human or Monte-Carlo
pub enum Player {
    /// Enum variant for human playing
    Human(human::Engine),
    Bruteforce(bruteforce::Engine),
    Montecarlo(monte_carlo::Engine),
}

impl Player {
    pub fn make_move (&mut self, gamestate: u128, elapsed: u128) -> u128 {
        match self {
            Player::Human(e) => e.make_move(gamestate),
            Player::Bruteforce(e) => e.make_move(gamestate),
            Player::Montecarlo(e) => e.make_move(gamestate, elapsed),
        }
    }

    // pub fn monte_carlo_intermission_loop (&mut self, gamestate: u128, timer: Instant, time: u128, rx: Receiver<bool>) {
    //     match self {
    //         Player::Montecarlo(e) => e.monte_carlo_loop(gamestate, timer, time, rx),
    //         _ => {},
    //     }
    // }
}