use axum::{
    Json, Router,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use maud::html;
use serde::{Deserialize, Serialize};

use crate::components::base::base;
pub mod components;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/test", get(test_typst))
        // `POST /users` goes to `create_user`
        .route("/user", post(create_user));

    // run our app with hyper, listening globally on port 3141
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6283").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    base(html! { p {"Blaah blah blaah"}})
}

async fn test_typst() -> impl IntoResponse {
    "This is a test"
}
async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
