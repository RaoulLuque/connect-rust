//! Library for playing connect four with different bots (engines)
//! 
//! Provides the framework needed to play connect four with bitboards, checking whether gamestates
//! indicate that the game is over or checking what are the next possible moves.
//! The main purpose of this library is supplying the back-bone for the binary which starts a
//! webserver to play against the different engines implemented. For more information see the 
//! [GitHub Repository](https://github.com/RaoulLuque/connect-rust)

pub mod helpers;
pub mod players;
pub mod webserver_handling;
