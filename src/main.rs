use askama::Template; // bring trait in scope
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use tower_http::services::ServeDir;
#[derive(Template)] // this will generate the code...
#[template(path = "test.html")] // using the template in this path, relative
                                // to the `templates` dir in the crate root
struct HelloTemplate<'a> {
    // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}

use comrak::{markdown_to_html, Options};

assert_eq!(
    markdown_to_html("Hello, **世界**!", &Options::default()),
    "<p>Hello, <strong>世界</strong>!</p>\n"
);

use comrak::{markdown_to_html, Options};
use std::fs::{self, create_dir_all, read_to_string, write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn generate_blog_html(
    input_dir: &Path,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Iterate over each file in the provided directory.
    for entry in WalkDir::new(input_dir)
        .into_iter()
        .filter_map(Result::ok) // Skip any errors during directory traversal.
        .filter(|e| e.file_type().is_file())
    // We're only interested in files, not directories.
    {
        let input_path = entry.path();
        if input_path.extension().map_or(false, |ext| ext == "md") {
            // Check if it's a Markdown file.
            // Read the content from the markdown file.
            let markdown_content = read_to_string(input_path)?;

            // Convert the markdown content to HTML.
            let html_content = markdown_to_html(&markdown_content, &Options::default());

            // Create the corresponding output path.
            let mut relative_path = input_path.strip_prefix(input_dir)?.to_path_buf();
            relative_path.set_extension("html"); // Change the extension to .html
            let output_path = output_dir.join(relative_path);

            // Create necessary directories in the output path.
            if let Some(parent_dir) = output_path.parent() {
                create_dir_all(parent_dir)?;
            }

            // Write the HTML content to the output path.
            write(output_path, html_content)?;
        }
    }
    Ok(())
}

#[tokio::main]
// Use globbing
async fn main() {
    // initialize tracing
    let assets_path = std::env::current_dir().unwrap();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );
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
