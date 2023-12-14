// mod gamestate_helpers;
// mod logging;
// mod multithreading;
// mod players;
// mod setup;

// use gamestate_helpers::PlayerColor;
// use logging::Logger;
// use players::Player;

// use std::time::Instant;

// const CALCULATE_WHILE_HUMAN_IS_CHOOSING_NEXT_TURN: bool = false;

// /// Plays the connect four game and asks which players/engines should play against which.
// /// If human players are playing, gamestates are shown in console directly otherwise they are visible in logs
// fn main() {
//     // Printing explanation of game
//     setup::print_introduction();

//     // Choosing who to play as/against (choosing players)
//     let (mut player_blue, mut player_red, elapsed_blue, elapsed_red) = setup::read_in_players();

//     // Setup of variables for game
//     let mut current_gamestate: u128 = 0;
//     let mut turn_number: usize = 0;
//     let mut log = Logger::new();
//     let mut winner: Option<PlayerColor> = None;
//     let mut elapsed: u128 = 1000;

//     // Log the initialization of the game
//     log.log_initialization(elapsed_blue, elapsed_red)
//         .expect("Logging should be possible");

//     // Check if multithreading is necessary in case human is playing against montecarlo
//     let thread_identifier = match (&player_blue, &player_red) {
//         (Player::Human(_), Player::Montecarlo(_)) => Some(true),
//         (Player::Montecarlo(_), Player::Human(_)) => Some(false),
//         _ => None,
//     };

//     // Running the game
//     while winner == None && !gamestate_helpers::is_full(current_gamestate) {
//         // Increment turn number
//         turn_number += 1;

//         // Timing how long it took to calculate turn
//         let timer = Instant::now();

//         // Make mutable references of players in order to move those references into other threads
//         let player_blue = &mut player_blue;
//         let player_red = &mut player_red;

//         let next_move = match (
//             thread_identifier,
//             gamestate_helpers::whos_turn_is_it_turn_number(turn_number),
//             CALCULATE_WHILE_HUMAN_IS_CHOOSING_NEXT_TURN,
//         ) {
//             (Some(true), PlayerColor::Blue, true) => {
//                 multithreading::calculate_montecarlo_while_human_chooses_turn(
//                     player_red,
//                     player_blue,
//                     current_gamestate,
//                 )
//             }

//             (Some(false), PlayerColor::Red, true) => {
//                 multithreading::calculate_montecarlo_while_human_chooses_turn(
//                     player_blue,
//                     player_red,
//                     current_gamestate,
//                 )
//             }
//             _ => {
//                 // Chooses the next move based on the current player who's turn it is and the engine chosen
//                 match gamestate_helpers::whos_turn_is_it_turn_number(turn_number) {
//                     PlayerColor::Blue => player_blue.make_move(current_gamestate, elapsed),
//                     PlayerColor::Red => player_red.make_move(current_gamestate, elapsed),
//                 }
//             }
//         };

//         // Taking time
//         elapsed = timer.elapsed().as_millis();

//         // Checking whether move was valid
//         if !crate::gamestate_helpers::is_allowed_move(current_gamestate, next_move, turn_number) {
//             // Move is invalid, logged and game is stopped
//             log.log_invalid_turn(turn_number, current_gamestate, next_move)
//                 .expect("Logging should be possible");

//             winner = match gamestate_helpers::whos_turn_is_it_turn_number(turn_number) {
//                 PlayerColor::Blue => Some(PlayerColor::Red),
//                 PlayerColor::Red => Some(PlayerColor::Blue),
//             };
//             break;
//         } else {
//             // Move is valid and is logged
//             current_gamestate = current_gamestate | next_move;
//             log.log_turn(turn_number, current_gamestate, elapsed)
//                 .expect("Logging should be possible");
//         }
//         // Set winner for checking if game over?
//         winner = gamestate_helpers::is_won(current_gamestate);
//     }

//     // Log who has won
//     log.log_winner(&winner, turn_number)
//         .expect("Logging should be possible");

//     // Declare winner
//     setup::declare_winner(&winner, turn_number, current_gamestate);
// }

mod html_template;

use html_template::START_PAGE_TEMPLATE;

use axum::{
    extract::Form,
    response::Html,
    routing::{get, post},
    Router,
};
use minijinja::render;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(start_page))
        .route("/", post(accept_move));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn accept_move(Form(turn): Form<GameMoveInput>) -> Html<String> {
    let response = match turn.column {
        Some(a) => GameMoveOutput { column: a },
        None => GameMoveOutput { column: 100 },
    };
    println!("The given number is: {:?}", response);

    let r = render!(START_PAGE_TEMPLATE, turn => response);
    Html(r)
}

async fn start_page() -> Html<&'static str> {
    Html(
        r#"<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Connect-Rust</title>
    </head>
    <body>
    <h1>What turn would you like to make?</h1>
        <br>
        <h2>Turn</h2>
        <form action="/" method="post">
            <!-- turn -->
            <label for="column">Turn (Enter the number of the column you'd like to drop a token into as an arabic number)</label>
            <br>
            <input type="text" name="column" id="column">
            <br> <br>
            <input type="submit" value="Submit">
        </form>
    </body>
    </html>
    "#,
    )
}

#[derive(Debug, Deserialize)]
struct GameMoveInput {
    column: Option<u16>,
}

#[derive(Debug, Serialize)]
struct GameMoveOutput {
    column: u16,
}
