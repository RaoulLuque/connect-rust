mod gamestate_helpers;
mod html_template;
mod players;
mod response_handling;

use players::Player;
use response_handling::{incoming, outgoing};

use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(outgoing::start_page))
        .route("/", post(incoming::accept_move));
    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
