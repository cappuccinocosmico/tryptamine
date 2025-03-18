use crate::Route;
use comrak;
use dioxus::prelude::*;
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use markdown;

const BLOG_CSS: &str = "/assets/styling/blog.css";
const BLOG_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/blog");

lazy_static! {
    pub static ref SLUGS: Vec<String> = {
        let mut slugs = Vec::new();
        for entry in BLOG_DIR.files() {
            let path = entry
                .path()
                .to_str()
                .unwrap()
                .strip_suffix(".md")
                .unwrap()
                .replace('\\', "/");
            slugs.push(path.to_string());
        }
        slugs.sort();
        slugs
    };
}

#[component]
pub fn BlogPost(slug: String) -> Element {
    let markdown_content = BLOG_DIR
        .get_file(&format!("{}.md", slug.replace("/", "\\")))
        .and_then(|file| file.contents_utf8())
        .unwrap_or("Blog post not found");

    // let current_index = crate::views::blog::SLUGS
    //     .iter()
    //     .position(|s| s == &slug)
    //     .unwrap_or(0);
    // let prev_slug = current_index
    //     .checked_sub(1)
    //     .and_then(|i| crate::views::blog::SLUGS.get(i));
    // let next_slug = crate::views::blog::SLUGS.get(current_index + 1);
    let class = use_signal(|| String::from("content"));

    rsx! {
        link { rel: "stylesheet", href: BLOG_CSS }
        link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/katex@0.16.21/dist/katex.css",
            integrity: "sha384-FkTZUsHjYVyYpU6dse+5AzszY5617FqhnLpcMIIAlLKTbdmeVMO/7K6BrdHWM28V",
            crossorigin: "anonymous"
        }
        script {
            defer: true,
            src: "https://cdn.jsdelivr.net/npm/katex@0.16.21/dist/katex.js",
            integrity: "sha384-CAltQiu9myJj3FAllEacN6FT+rOyXo+hFZKGuR2p4HB8JvJlyUHm31eLfL4eEiJL",
            crossorigin: "anonymous"
        }
        script {
            defer: true,
            src: "https://cdn.jsdelivr.net/npm/katex@0.16.21/dist/contrib/auto-render.min.js",
            integrity: "sha384-hCXGrW6PitJEwbkoStFjeJxv+fSOOQKOPbJxSfM6G5sWZjAyWhXiTIIAmQqnlLlh",
            crossorigin: "anonymous"
        }
        script {
            dangerous_inner_html: "document.addEventListener('DOMContentLoaded', function() {renderMathInElement(document.body);});",
        }
        Markdown {
            content: markdown_content,
        }
    }
}

// fn render_markdown(markdown: &str) -> String {
//     let mut options = comrak::Options::default();
//     options.extension.math_dollars = true;
//     options.extension.spoiler = true;
//     comrak::markdown_to_html(markdown, &options)
// }
fn render_markdown(markdown: &str) -> String {
    markdown::to_html_with_options(
        markdown,
        &markdown::Options {
            parse: markdown::ParseOptions {
                constructs: markdown::Constructs {
                    // math_flow: true,
                    // math_text: true,
                    mdx_esm: true,
                    mdx_expression_flow: true,
                    mdx_expression_text: true,
                    mdx_jsx_flow: true,
                    mdx_jsx_text: true,
                    ..markdown::Constructs::default()
                },
                // math_text_single_dollar: true,
                ..markdown::ParseOptions::default()
            },
            compile: markdown::CompileOptions {
                allow_dangerous_html: true,
                allow_dangerous_protocol: true,
                ..markdown::CompileOptions::default()
            },
            ..markdown::Options::default()
        },
    )
    .unwrap_or("<p>Failed To Parse Markdown</p>".to_string())
}

#[derive(Props, Clone, PartialEq)]
pub struct MarkdownProps {
    #[props(default)]
    id: Signal<String>,
    #[props(default)]
    class: Signal<String>,

    content: ReadOnlySignal<String>,
}

#[component]
pub fn Markdown(props: MarkdownProps) -> Element {
    let content = &*props.content.read();
    let html_buf = render_markdown(content);
    rsx! {
        div {
            id: "{&*props.id.read()}",
            class: "{&*props.class.read()}",
            dangerous_inner_html: "{html_buf}"
        }
    }
}
