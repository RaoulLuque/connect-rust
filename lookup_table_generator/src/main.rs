/// This little tool generates the responses for all the possible responses from a list of gamestates.
/// More percisely. Given the gamestate 1313 this tool generates the responses for 13131, 13132,
/// and so on and write them in a text file.
use connect_rust_webserver_and_engine::helpers::encoding_gamestates::turn_series_of_columns_to_encoded_gamestate;
use connect_rust_webserver_and_engine::helpers::moves::possible_next_gamestates;
use connect_rust_webserver_and_engine::players::bruteforce::Engine;
use std::fs::read_to_string;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::u128;

const FILE_NAME_FOR_GAMESTATES_TO_RESPOND_TO: &str = "moves_for_which_to_generate_responses.txt";
const MOVES_ARE_GIVEN_AS_SEQUENCE_OF_COLUMNS: bool = true;

fn main() {
    println!("The current path is: {:?}", std::env::current_dir());
    let mut file_to_write_to = OpenOptions::new()
        .write(true)
        .append(true)
        .open("responses/response_lookup_table.txt")
        .expect("File should be creatable");

    for line in read_to_string(format!(
        "{}{}",
        "responses/", FILE_NAME_FOR_GAMESTATES_TO_RESPOND_TO
    ))
    .expect(&format!(
        "File: {} should exist",
        FILE_NAME_FOR_GAMESTATES_TO_RESPOND_TO
    ))
    .lines()
    {
        let line = line.trim();
        let gamestate_for_which_to_generate_responses = match MOVES_ARE_GIVEN_AS_SEQUENCE_OF_COLUMNS
        {
            true => turn_series_of_columns_to_encoded_gamestate(line),
            false => line.parse::<u128>().expect("Lines should be u128 numbers"),
        };

        for move_to_respond_to in
            possible_next_gamestates(gamestate_for_which_to_generate_responses)
        {
            let response = Engine::make_move(move_to_respond_to, false).0;
            write_response_to_file(&mut file_to_write_to, response, move_to_respond_to);
        }
    }
}

fn write_response_to_file(
    file_to_write_to: &mut File,
    response: u128,
    move_to_respond_to: u128,
) -> () {
    writeln!(file_to_write_to, "{} => {},", move_to_respond_to, response)
        .expect("File should be writable");
    println!("Wrote move: {} to file", response);
}
