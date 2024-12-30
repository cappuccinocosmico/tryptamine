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
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

pub use crate::fractals::images_fractal;
pub use crate::math::{check_primality_test, first_n_primes, miller_rabin_primality, WitnessSet};
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
    env_logger::init();
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
        .route("/run_tests", get(run_testing_code))
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
        .nest_service("/test-fractal/:resolution/:format", get(test_fractal))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // `POST /users` goes to `create_user`

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[utoipa::path(
    get,
    path = "/main.css",
    responses(
        (status = 200, description = "Returns the Tailwind CSS styles", content_type = "text/css")
    )
)]
async fn main_tailwind_styles() -> Response<Body> {
    // Build the response
    (
        [(header::CONTENT_TYPE, HeaderValue::from_static("text/css"))],
        TAILWIND_CSS,
    )
        .into_response()
}

#[utoipa::path(
    get,
    path = "/get-primes/{num_primes}",
    params(
        ("num_primes" = u32, Path, description = "Number of prime numbers to return")
    ),
    responses(
        (status = 200, description = "Returns a list of prime numbers", content_type = "application/json", body = Vec<u32>)
    )
)]
async fn get_prime_list(Path(num_primes): Path<u32>) -> impl IntoResponse {
    let primes: Vec<u32> = first_n_primes(num_primes as usize, WitnessSet::default());
    axum::Json(primes)
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Returns the main HTML template", content_type = "text/html")
    )
)]
async fn root() -> Html<String> {
    let app = AppTemplate {}; // instantiate your struct
    let test_html = app.render().unwrap();
    Html(test_html)
}

#[utoipa::path(
    get,
    path = "/is_prime/{num}",
    params(
        ("num" = u32, Path, description = "Number to check for primality")
    ),
    responses(
        (status = 200, description = "Returns whether the number is prime", content_type = "application/json", body = bool)
    )
)]
async fn is_prime(Path(num): Path<u32>) -> impl IntoResponse {
    let is_prime = small_is_prime(&num);
    axum::Json(is_prime)
}
#[utoipa::path(
    get,
    path = "/run_tests",
    responses(
        (status = 200, description = "Ran tests", content_type = "text/plain", body = bool)
    )
)]
async fn run_testing_code() -> impl IntoResponse {
    check_primality_test();
    "Success!"
}

#[utoipa::path(
    get,
    path = "/test-fractal/{resolution}/{format_str}",
    params(
        ("resolution" = u32, Path, description = "Image resolution"),
        ("format_str" = String, Path, description = "Image format (e.g., 'png', 'jpg')")
    ),
    responses(
        (status = 200, description = "Returns a fractal image", content_type = "image/*"),
        (status = 400, description = "Unknown image type", content_type = "text/plain")
    )
)]
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

#[derive(OpenApi)]
#[openapi(paths(test_fractal, is_prime, get_prime_list, root, run_testing_code))]
struct ApiDoc;
