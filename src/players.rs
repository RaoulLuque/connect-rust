pub mod human;
pub mod bruteforce;

/// Enum for types of players, e.g. Bruteforce, Human or Monte-Carlo
pub enum Player {
    /// Enum variant for human playing
    Human(human::Engine),
    Bruteforce(bruteforce::Engine),

}

impl Player {
    pub fn make_move (&mut self, gamestate: u32) -> u32 {
        match self {
            Player::Human(e) => e.make_move(gamestate),
            Player::Bruteforce(e) => e.make_move(gamestate),
        }
    }
}