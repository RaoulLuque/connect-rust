use crate::players::Player;

use std::thread;
use std::time::Instant;
use std::sync::mpsc;

/// Functions that multithreads computations for montecarlo engine while waiting for human player
/// to input new turn
pub fn calculate_montecarlo_while_human_chooses_turn(montecarlo: &mut Player, human: &mut Player, gamestate: u128) -> u128 {
    let (tx_blue, rx_blue) = mpsc::channel();
    let (tx_blue_back, rx_blue_back) = mpsc::channel();

    thread::scope(|s| {
        s.spawn(move || {
            let timer = Instant::now();
            montecarlo.monte_carlo_intermission_loop(gamestate, timer, 1000000, rx_blue);
        });

        s.spawn(move || {
            let next_move = human.make_move(gamestate, 0);

            // Tell other thread that reading in is finished and montecarlo calculations should be stopped
            tx_blue.send(true).unwrap();

            tx_blue_back.send(next_move)
        });
    });

    rx_blue_back.recv().unwrap()
}