use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

#[tokio::main]
async fn main() {
    // initialize tracing

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));
    // `POST /users` goes to `create_user`

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
