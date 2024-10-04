use askama::Template; // bring trait in scope
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
#[derive(Template)] // this will generate the code...
#[template(path = "test.html")] // using the template in this path, relative
                                // to the `templates` dir in the crate root
struct HelloTemplate<'a> {
    // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}

#[tokio::main]
// Use globbing
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
async fn root() -> Html<String> {
    let hello = HelloTemplate { name: "world" }; // instantiate your struct
    let test_html = hello.render().unwrap();
    Html(test_html)
}
