use crate::Route;
use dioxus::prelude::*;
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use markdown::{to_html_with_options, CompileOptions, Options};

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

    let current_index = crate::views::blog::SLUGS
        .iter()
        .position(|s| s == &slug)
        .unwrap_or(0);
    let prev_slug = current_index
        .checked_sub(1)
        .and_then(|i| crate::views::blog::SLUGS.get(i));
    let next_slug = crate::views::blog::SLUGS.get(current_index + 1);
    let class = use_signal(|| String::from("content"));

    rsx! {
        link { rel: "stylesheet", href: BLOG_CSS }
        div { id: "blog-container",
            Markdown {
                content: markdown_content,
            }
        }
    }
}
fn RenderMarkdown(markdown: &str) -> String {
    to_html_with_options(
        markdown,
        &Options {
            compile: CompileOptions {
                allow_dangerous_html: true,
                allow_dangerous_protocol: true,
                ..CompileOptions::default()
            },
            ..Options::default()
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
pub fn Markdown(props: MarkdownProps) -> Element {
    let content = &*props.content.read();
    let html_buf = RenderMarkdown(content);
    rsx! {
        div {
            id: "{&*props.id.read()}",
            class: "{&*props.class.read()}",
            dangerous_inner_html: "{html_buf}"
        }
    }
}
