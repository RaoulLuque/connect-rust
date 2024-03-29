use connect_rust::helpers::encoding_gamestates::turn_series_of_columns_to_encoded_gamestate;
use connect_rust::players::bruteforce::Engine;
use std::fs::read_to_string;
use std::time::Duration;

fn main() {
    run_all_benchmarks()
}

#[allow(dead_code)]
fn run_all_benchmarks() -> () {
    run_specific_benchmark(3, 1);

    // Run Benchmarks Test_L2_R1 and Test_L2_R2
    for i in 1..3 {
        run_specific_benchmark(2, i);
    }

    // Run Benchmarks Test_L1_R1 to Test_L1_R3
    for i in 1..4 {
        run_specific_benchmark(1, i);
    }
}

// Progress of game: 3 - lategame; 2 - midgame; 1 - earlygame
fn run_specific_benchmark(progress_of_game: u32, difficulty_of_set: u32) -> () {
    let mut total_number_of_examples: u32 = 0;
    let mut total_computation_time: Duration = Duration::new(0,0);
    let mut total_number_of_explored_nodes: u32 = 1;
    let mut total_number_of_failed_examples: u32 = 0;

    let dataset_filename: String = format!("Test_L{}_R{}", progress_of_game, difficulty_of_set);
    let dataset_path: String = format!("benchmarks/src/benchmarks/{}",dataset_filename.clone());

    for line in read_to_string(&dataset_path)
    .expect(&format!("File: {} should exist", &dataset_path)).lines() {
        let mut line = line.split_whitespace();
        
        let current_gamestate = turn_series_of_columns_to_encoded_gamestate(
            line.next().expect("Line should have gamestate"),
        );

        let expected_evaluation: i32 = line
            .next()
            .expect("Line should have expected evaluation")
            .to_owned()
            .parse::<i32>()
            .expect("Line should have expected evaluation and it should be integer");

        let (_, actual_evaluation, number_of_explored_nodes, computation_time) = Engine::make_move(current_gamestate, false);
        
        let actual_evaluation = actual_evaluation as i32;

        total_number_of_examples += 1;
        total_number_of_explored_nodes += number_of_explored_nodes;
        total_computation_time += computation_time;

        if actual_evaluation != expected_evaluation {
            println!("On the {}th example following mal-evaluation was made: The evaluation was supposed to be: {}. The engine suggested the evaluation: {}", 
            total_number_of_examples, expected_evaluation, actual_evaluation);
            total_number_of_failed_examples += 1;
        // } else {
        //     println!("The {}th example was evaluated correctly as: {}", 
        //     total_number_of_examples, actual_evaluation);
        }
    }

    println!("---");
    println!("Benchmark: {}", dataset_filename);
    println!(
        "In total {} examples were calculated",
        total_number_of_examples
    );
    println!("{} of such examples failed", total_number_of_failed_examples);

    println!(" - ");
    println!("The mean time is: {:?}, the mean number of positions: {}, the number of positions per second: {} and the total time is: {:?}", 
    total_computation_time/total_number_of_examples, total_number_of_explored_nodes/total_number_of_examples, 
    (total_number_of_explored_nodes as u128)/total_computation_time.as_micros() * 1000000, total_computation_time);
}
