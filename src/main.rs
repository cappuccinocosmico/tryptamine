pub const TAILWIND_CSS: &str = include_str!("../styles/main.css");

use askama::Template; // bring trait in scope
use axum::body::Body;
use axum::{
    http::{header, HeaderValue},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use tower_http::services::ServeDir;

use std::path::PathBuf;

use crate::website::static_html;
#[derive(Template)] // this will generate the code...
#[template(path = "app.html")] // using the template in this path, relative

// to the `templates` dir in the crate root
struct AppTemplate {
    // the name of the struct can be anything
    // in your template
}
#[tokio::main]
// Use globbing
async fn main() {
    let _md_result =
        static_html::generate_blog_html(&PathBuf::from("markdown"), &PathBuf::from("static"));
    // initialize tracing
    let project_path = std::env::current_dir().unwrap();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/main.css", get(main_tailwind_styles))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", project_path.to_str().unwrap())),
        )
        .nest_service(
            "/blog",
            ServeDir::new(format!("{}/static/bog", project_path.to_str().unwrap())),
        )
        .nest_service(
            "/recipies",
            ServeDir::new(format!(
                "{}/static/recipies",
                project_path.to_str().unwrap()
            )),
        )
        .nest_service("/fractal", get(test_fractal));

    // `POST /users` goes to `create_user`

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn main_tailwind_styles() -> Response<Body> {
    // Build the response
    (
        [(header::CONTENT_TYPE, HeaderValue::from_static("text/css"))],
        TAILWIND_CSS,
    )
        .into_response()
}

async fn test_fractal() -> Response<Body> {
    (
        [(header::CONTENT_TYPE, HeaderValue::from_static("text/css"))],
        "hallo",
    )
        .into_response()
}

async fn root() -> Html<String> {
    let app = AppTemplate {}; // instantiate your struct
    let test_html = app.render().unwrap();
    Html(test_html)
}
