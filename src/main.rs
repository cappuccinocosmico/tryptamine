use askama::Template; // bring trait in scope
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use tower_http::services::ServeDir;
#[derive(Template)] // this will generate the code...
#[template(path = "app.html")] // using the template in this path, relative
                               // to the `templates` dir in the crate root
struct AppTemplate<'a> {
    // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}
#[derive(Template)] // this will generate the code...
#[template(path = "post.html", escape = "none")] // using the template in this path, relative
                                                 // to the `templates` dir in the crate root
struct PostTemplate<'a> {
    // the name of the struct can be anything
    title: &'a str,
    author: &'a str,
    date: &'a str,
    html: &'a str, // the field name should match the variable name
                   // in your template
}

use comrak::{markdown_to_html, Options};
use std::fs::{self, create_dir_all, read_to_string, write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use std::error::Error;
use yaml_rust::yaml::{Hash, Yaml};
use yaml_rust::YamlLoader;

fn generate_markdown_wrapper() -> impl Fn(&str) -> String {
    |md_post: &str| -> String {
        let mut markdown_options = Options::default();
        markdown_options.extension.strikethrough = true;
        markdown_options.extension.table = true;
        markdown_options.extension.description_lists = true;
        markdown_options.extension.footnotes = true;
        markdown_options.extension.autolink = true;
        markdown_options.extension.tagfilter = true;
        markdown_options.extension.math_dollars = true;

        let md_post = md_post.trim_start();
        let parts: Vec<&str> = md_post.splitn(2, "\n---\n").collect();
        if parts.len() == 2 {
            let yaml_str = parts[0];
            let yaml_str = if yaml_str.starts_with("---\n") {
                &yaml_str[4..]
            } else {
                yaml_str
            };
            let markdown_str = parts[1];

            let header_hashmap = YamlLoader::load_from_str(yaml_str).unwrap();
            let header = header_hashmap[0].clone();
            let get_field = |field: &str| header[field].as_str().unwrap_or("Unknown").to_string();
            let title = get_field("title");
            let author = get_field("author");
            let date = get_field("date");
            let raw_html = markdown_to_html(markdown_str, &markdown_options);
            let post_html = (PostTemplate {
                html: &raw_html,
                title: &title,
                author: &author,
                date: &date,
            })
            .render()
            .unwrap();
            return post_html;
        }
        println!("No YAML found in markdown file, just compiling as if it didnt exist.");
        let raw_html = markdown_to_html(md_post, &Options::default());
        let post_html = (PostTemplate {
            html: &raw_html,
            title: "Unknown",
            author: "Unknown",
            date: "Unknown",
        })
        .render()
        .unwrap();
        return post_html;
    }
}

pub fn generate_blog_html(
    input_dir: &Path,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let md_wrapper = generate_markdown_wrapper();
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
            let html_content = md_wrapper(&markdown_content);

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
    generate_blog_html(&PathBuf::from("markdown"), &PathBuf::from("static"));
    // initialize tracing
    let project_path = std::env::current_dir().unwrap();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", project_path.to_str().unwrap())),
        )
        .nest_service(
            "/static",
            ServeDir::new(format!("{}/static", project_path.to_str().unwrap())),
        );
    // `POST /users` goes to `create_user`

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn root() -> Html<String> {
    let app = AppTemplate { name: "world" }; // instantiate your struct
    let test_html = app.render().unwrap();
    Html(test_html)
}
