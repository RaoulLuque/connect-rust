# connect-rust
This is a simple engine and two AI's for connect four. There is no real challenge in creating a bruteforce AI for a game like connect four. The purpose of this project was getting to know rust and experimenting with monte carlo search.
For saving of gamestates in a graph in bruteforce and montecarlo AI the graph library I [wrote](https://github.com/RaoulLuque/connect-rust-graphs) was used.

## Game engine
The connect four engine uses gamestates encoded in the u128 rust standard library data type. Each field is represented by two bits enabling for the three different states each field can be in.

## Bruteforce AI
The bruteforce AI works by calculating best moves for all possible gamestates at initialization and then choosing the calculated best move in each turn resulting in longer initialization and short game time. 
Because of this several tweaks had to be made to the usual alpha-beta pruning. Furthermore naive move sorting and a mirror detection for detecting mirrored gamestates and saving only one of the mirrored gamestates is implemented.

## Montecarlo AI
The montecarlo AI works by simulating games and using these simulations to determine which is the most probably best move. For this naive move sorting is also used. If playing against human simulations are done in a seperate thread while human is choosing next move for better decision making.
