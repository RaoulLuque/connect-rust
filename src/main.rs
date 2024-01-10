use connectrustlibrary::response_handling::{incoming, outgoing};

use axum::{
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let service = handle_404.into_service();
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(outgoing::start_page))
        .route("/", post(incoming::accept_move))
        .fallback_service(service);
    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Nothing to see here")
}
