/// Helper crate for handling of requests to webserver

pub mod how_to_play_html_template;
pub mod incoming;
pub mod outgoing;
pub mod start_page_html_template;

use crate::helpers::PlayerColor;
use crate::players::Player;

use axum::response::Html;
