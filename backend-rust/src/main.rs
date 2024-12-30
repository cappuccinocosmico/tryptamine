mod colors;
mod fractals;
mod math;
mod website; // Declare the module
pub const TAILWIND_CSS: &str = include_str!("../styles/main.css");

use askama::Template; // bring trait in scope
use axum::body::Body;
use axum::{
    extract::Path,
    http::{header, HeaderValue},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use fractals::images_fractal::str_image_extension;
use math::small_is_prime;
use tower_http::services::ServeDir;

use num_bigint::BigUint;
use std::path::PathBuf;

pub use crate::fractals::images_fractal;
pub use crate::math::{first_n_primes, miller_rabin_primality};
pub use crate::website::static_html;
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
    // let _md_result =
    //     static_html::generate_blog_html(&PathBuf::from("markdown"), &PathBuf::from("static"));
    // // initialize tracing
    // let project_path = std::env::current_dir().unwrap();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/main.css", get(main_tailwind_styles))
        .route("/get-primes/:num_primes", get(get_prime_list))
        .route("/is_prime/:num_primes", get(is_prime))
        // .nest_service(
        //     "/assets",
        //     ServeDir::new(format!("{}/assets", project_path.to_str().unwrap())),
        // )
        // .nest_service(
        //     "/blog",
        //     ServeDir::new(format!("{}/static/bog", project_path.to_str().unwrap())),
        // )
        // .nest_service(
        //     "/recipies",
        //     ServeDir::new(format!(
        //         "{}/static/recipies",
        //         project_path.to_str().unwrap()
        //     )),
        // )
        .nest_service("/test-fractal/:resolution/:format", get(test_fractal));

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

async fn get_prime_list(Path(num_primes): Path<u32>) -> impl IntoResponse {
    let primes: Vec<u32> = first_n_primes(num_primes as usize);
    axum::Json(primes)
}

async fn is_prime(Path(num): Path<u32>) -> impl IntoResponse {
    let is_prime = small_is_prime(&num);
    axum::Json(is_prime)
}

async fn test_fractal(Path((resolution, format_str)): Path<(u32, String)>) -> Response<Body> {
    // Build the response
    let image_type = match str_image_extension(&format_str) {
        Some(image_type) => image_type,
        None => {
            return "Unknown image type".to_string().into_response();
        }
    };
    let header_str = format!("image/{}", format_str);
    (
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_str(&header_str).unwrap(),
        )],
        images_fractal::test_image(resolution, image_type),
    )
        .into_response()
}

async fn root() -> Html<String> {
    let app = AppTemplate {}; // instantiate your struct
    let test_html = app.render().unwrap();
    Html(test_html)
}
