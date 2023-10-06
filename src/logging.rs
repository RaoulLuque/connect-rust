/// Logger for connect four game with file output
use std::fs::File;
use std::io::Write;
use crate::gamestate_helpers;
use crate::gamestate_helpers::PlayerColor;

/// Struct for logging
pub struct Logger {
    /// File the logs are done in
    file: File,
}

impl Logger {
    /// Create logger with standard file name log.txt
    pub fn new() -> Logger {
        std::fs::remove_file("log.txt").err();
        if let Ok(file_type) = File::create("log.txt") {
            return Logger {
                file: file_type,
            }
        } else {
            panic!("Not able to create log.txt file");
        }
    }

    /// Create logger with specified name
    pub fn new_with_name(name :&str) -> Logger {
        std::fs::remove_file("log.txt").err();
        let mut path = name.to_owned();
        path.push_str("txt");
        if let Ok(file_type) = File::create(path) {
            return Logger {
                file: file_type,
            }
        } else {
            panic!("Not able to create {}.txt file", name);
        }
    }

    /// Logs the time it took to initialize color engines
    pub fn log_initialization(&mut self, elapsed_time_blue: u128, elapsed_time_red: u128) -> std::io::Result<()> {
        self.file.write_all(format!("Blue took {} milliseconds to initialize \n", elapsed_time_blue).as_bytes())?;

        self.file.write_all(format!("Red took {} milliseconds to initialize \n \n", elapsed_time_red).as_bytes())?;

        Ok(())
    }

    /// Logs a turn that has been made via the resulting gamestate and the turn number
    /// Returns result with error, if writing to the log file was not possible
    pub fn log_turn (&mut self, turn_number: usize, gamestate: u128, elapsed_time: u128) -> std::io::Result<()> {
        // Log header
        self.log_header(turn_number)?;

        // Get player who's turn it is
        let player_whos_turn_it_is = gamestate_helpers::whos_turn_is_it_turn_number(turn_number);

        self.file.write_all(format!("It is {:?}'s turn \n \n", player_whos_turn_it_is).as_bytes())?;
        
        // Turn gamestate into readable string
        let playing_field = gamestate_helpers::encoded_gamestate_to_str(gamestate);

        self.file.write_all(playing_field.as_bytes())?;
        self.file.write_all(format!("The turn took {} miliseconds \n", elapsed_time).as_bytes())?;

        Ok(())
    }

    /// Logs an invalid turn in the log with the turn number and the player whose turn it was
    pub fn log_invalid_turn (&mut self, turn_number: usize, gamestate: u128, current_move: u128) -> std::io::Result<()> {
        // Log header
        self.log_header(turn_number)?;

        // Get player who's turn it is
        let player_whos_turn_it_is = gamestate_helpers::whos_turn_is_it_turn_number(turn_number);

        // Error insert
        self.file.write_all(b"Invalid turn has been made: \n")?;

        self.file.write_all(format!("It was {:?}'s turn \n \n", player_whos_turn_it_is).as_bytes())?;
        
        // Turn gamestate into readable string
        let playing_field = gamestate_helpers::encoded_gamestate_to_str(gamestate);

        self.file.write_all(playing_field.as_bytes())?;

        // Add which move was tried
        if !gamestate_helpers::is_valid_move(current_move) {
            self.file.write_all(b"The given move was not valid in that it was not a power of 2 with exponent from 0 to 31 \n")?;
        } else {
            // Turn invalid move to readable tuple
            let invalid_move_as_tuple = gamestate_helpers::move_to_tuple(current_move);
            self.file.write_all(format!("{:?} tried to make the following move: {:?} \n", player_whos_turn_it_is, invalid_move_as_tuple).as_bytes())?;

            // Check whether space was unreachable or space was taken
            if gamestate & current_move == current_move {
                self.file.write_all(b"The move was invalid because the space was already taken! \n")?;
            } else {
                self.file.write_all(b"The move was invalid because the space was unreachable! \n")?;
            }
        }
        

        Ok(())
    }

    /// Creates the header of a log with -'s and the Turn number in the log
    fn log_header (&mut self, turn_number: usize) -> std::io::Result<()> {
        self.file.write_all(b"- - - - - - - - \n")?;
        self.file.write_all(format!("Turn number: {} \n", turn_number).as_bytes())?;

        Ok(())
    }

    /// Logs the winner of the game or that nobody won if game is over without winner
    pub fn log_winner (&mut self, winner: &Option<PlayerColor>, turn_number: usize) -> std::io::Result<()>{
        self.log_header(turn_number)?;

        let winner_string: &str = match winner {
            &Some(PlayerColor::Blue) => "Blue",
            &Some(PlayerColor::Red) => "Red",
            &None => "Nobody",
        };
        self.file.write_all(format!("The game has ended and {} has won \n", winner_string).as_bytes())?;
        Ok(())
    }

}


// To do: Test initialisation time
#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use gamestate_helpers::*;

    #[test]
    #[serial]
    fn creating_logger() {
        let _log = Logger::new();
    }

    #[test]
    #[serial]
    fn creating_multiple_loggers_with_same_name() {
        let _log_one = Logger::new();
        let _log_two = Logger::new();
    }

    #[test]
    #[serial]
    fn log_initialization_given_valid_initialization_log() {
        let mut log = Logger::new();
        log.log_initialization(1,1).unwrap();
    }

    #[test]
    #[serial]
    fn convert_gamestate_to_loggable_string() {
        let gamestate = 1;
        let gamestate_str = encoded_gamestate_to_str(gamestate);
        assert_eq!(&gamestate_str[..1], "R");
        assert_ne!(&gamestate_str[..1], "B");

        let gamestate_str = encoded_gamestate_to_str(2);
        assert_eq!(&gamestate_str[..1], "B");
        
        let gamestate_str = encoded_gamestate_to_str(2147483648);
        assert_eq!(&gamestate_str[33..34], "B");
    }

    #[test]
    #[serial]
    fn creating_log_header() {
        let mut log = Logger::new();
        log.log_header(1).unwrap();
    }

    #[test]
    #[serial]
    fn using_logger_to_log_turn() {
        let mut log = Logger::new();
        log.log_turn(1, 1, 10).unwrap();
    }

    #[test]
    #[serial]
    fn using_logger_to_log_invalid_move() {
        let mut log = Logger::new();
        log.log_invalid_turn(1, 2, 1).unwrap();
    }

    #[test]
    #[serial]
    fn using_logger_to_log_number_that_is_not_power_of_two() {
        let mut log = Logger::new();
        log.log_invalid_turn(1, 2, 3).unwrap();
    }

    #[test]
    #[serial]
    fn using_logger_to_someone_winning() {
        let mut log = Logger::new();
        log.log_winner(&Some(PlayerColor::Blue), 1).unwrap();
    }
}