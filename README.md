[<img src="https://i.ibb.co/VY4LJ6S/connect-rust.png">](https://www.youtube.com/watch?v=dQw4w9WgXcQ)

# Connect-Rust
This is a web server with different simple and more complex engines for the game connect four written entirely in rust. The different engines are explained more in depth in the [engines](#engines) chapter. The purpose of this project was getting to know rust and experimenting with concepts like webservers, multithreading, monte carlo search and of course having some fun along the way !

The current state of the project can be seen on this [connect-rust.dev.fly](https://connect-rust.fly.dev/).

![Screenshot of Webpage as rendered with Firefox on Ubuntu.](https://github.com/RaoulLuque/connect-rust/assets/125205120/c0f815e5-a443-48c8-b5e5-0d633af783f6)

For saving of gamestates in a graph in the montecarlo engine a [graph library](https://github.com/RaoulLuque/connect-rust-graphs) I wrote was used.

## Starting the Webserver locally
To start the webserver Rust and Cargo need to be installed. Visit [rust-up](https://rustup.rs/) for more information about that. With those dependencies just go into the project directory and run the command: <br>

``` cargo run --bin connect-rust --release ``` <br>

 The server will be reachable at localhost:8080

## Benchmarks
### Bruteforce
The following is a table with the benchmarks from Pascal Pons mentioned in the bruteforce engine explanation. The number after the L in the name of the benchmark indicates how far the games in the benchmark have progressed (higher being further) and the number after the R refers to the difficulty of the instances (higher being more difficult).

| **Benchmark** | # of examples | # of failed examples | Mean time / example | Mean # of positions / example | Mean # of positions / second | Total time |
|---------------|--------------------|---------------------------|-----------------------|--------------------------------------|-------------------------------------|------------|
| L3 R1         | 1000               | 0                         | 23.860 µs             | 58                                   | 2000000                             | 23.860 ms  |
| L2 R1         | 1000               | 0                         | 221.184 µs            | 606                                  | 2000000                             | 221.184 ms |
| L2 R2         | 1000               | 0                         | 38.972 s              | 121551                               | 3000000                             | 38.972 s   |
| L1 R1         | 1000               | 0                         | 1.229 ms              | 4129                                 | 3000000                             | 1.229 s    |

The benchmarks can be run locally by compiling the benchmarks. In order to do that run the following command in the project directory:

``` cargo run --bin benchmark --release ``` <br>

Now copy the benchmarks folder from the project directory into the target/release folder and just run the benchmark binary.

## Game Framework (Backend)
The framework is found in the [connect_rust](connect_rust) crate.
### Players
The Players in the game are encoded as a rust enum PlayerColor which has possible values blue and red which are also displayed as such in the web frontend.

### Encoding of gamestates
The connect four engine uses gamestates encoded in the u128 rust standard library data type. Each field is represented by two bits enabling for the three different states each field can be in. <br>
In order to understand this encoding the 6x7 connect-four board can be thought of as a series of 0s and 1s in the following way:
```
(0,0) (0,0) (0,0) (0,0) (0,0) (0,0) (0,0) 
(0,0) (0,0) (0,0) (0,0) (0,0) (0,0) (0,0) 
(0,0) (0,0) (0,0) (0,0) (0,0) (0,0) (0,0) 
(0,0) (0,0) (0,0) (0,0) (0,0) (0,0) (0,0) 
(0,0) (0,0) (0,0) (0,0) (0,0) (0,0) (0,0) 
(0,0) (0,0) (0,0) (0,0) (0,0) (0,0) (0,0)
```
Where the first bit of the u128 is the left entry of the leftmost-upmost tuple in the visualization
and the 84th bit of the u128 is the right entry of the rightmost-lowermost tuple in the visualization.
The bits continue right to left and bottom to top. Each tuple of course representing the state
of one field, e.g. if there is a red or blue token.
(1,0) would be signaling that there is a red token (0,1) a blue one and (0,0) no token yet. <br> 

The python files [encoding_to_game_board.py](encoding_to_game_board.py) and [game_board_to_encoding.py](game_board_to_encoding.py) visualize this encoding and enable for translation between encoding and human perceived boards. <br> <br>

Another - arguably simpler - encoding is just encoding the gamestate as a string of numbers. Where each number would indicate a column that was played. The first number/char would correspond to the first turn and so on. This encoding is used for communication between front and backend.

## Webserver and Frontend
The webserver and frontend can be found in the [connect_rust](connect_rust) crate and the [response_handling](connect_rust/src/response_handling) module within it.
### Webserver
The webserver is based on the [axum](https://github.com/tokio-rs/axum) framework which enables easy routing with multithreading. This project just uses a tiny bit of the framework's possibilities. For serializing and deserializing [serde](https://github.com/serde-rs/serde) derive is used.

### Frontend
The frontend is Html and CSS only. One might ask how the 0% Html and 0% CSS in the repo are achieved in that case. Actually the Html and CSS are embedded in the rust code in the response_handling module as  strings. This is in part due to using [minijinja](https://github.com/mitsuhiko/minijinja), a templating engine which enables if statements and loops for html templating.

## Engines
### Bruteforce
The bruteforce engine works by calculating the best possible move by considering all the possible next moves/gamestates. This is done using alpha-beta pruning or rather a negamax algorithm. Hereby some possible next gamestates are ruled out for consideration if they are irrelevant saving computation time. The implementation is very heavily based on the blog about solving connect four by [Pascal Pons](http://blog.gamesolver.org/). <br>

Although heavily optimized the engine is still not fast enough to be used in usual play for the first three turns. Which is why the first three turns are saved in a lookup table. This allows for a more natural game flow. From the fourth turn on the bruteforce engine calculates the moves on the fly.

### Montecarlo AI
The montecarlo engine works by simulating games and using these simulations to determine which of the possible next moves might be the best (from a stochastic point of view).

### Random*
The random* engine plays randomly except when there are three in a row for the human. In which case the fourth token is placed to avoid loosing.

### Bruteforce N%
The bruteforce N% engine plays as Bruteforce N% of the time. Otherwise the moves are made according to the random* engine.

### Random
The random engine plays completely random. Nonetheless, according to the rules of course.

## Lookup Table Generator
The Lookup table generator crate is a tool to generate responses to gamestates. It is used for generating the lookup table for the bruteforce engine.
