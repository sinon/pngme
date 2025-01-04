use axum::{
    routing::{get, post},
    Router,
};

mod handlers;
mod views;

#[tokio::main]
async fn main() {
    // Define routes
    let app = Router::new()
        .route("/", get(handlers::get_form))
        .route("/submit", post(handlers::handle_submit));

    // Set up the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
