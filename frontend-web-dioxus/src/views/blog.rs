use crate::Route;
use dioxus::prelude::*;
use dioxus_markdown::Markdown;
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;

const BLOG_CSS: &str = "/assets/styling/blog.css";
const BLOG_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/blog");

lazy_static! {
    static ref SLUGS: Vec<String> = {
        let mut slugs = Vec::new();
        for entry in BLOG_DIR.find("**/*.md").unwrap() {
            if let Some(file) = entry.as_file() {
                let path = file.path().to_str()
                    .unwrap()
                    .strip_suffix(".md")
                    .unwrap()
                    .replace('\\', "/");
                slugs.push(path.to_string());
            }
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

    let current_index = SLUGS.iter().position(|s| s == &slug).unwrap_or(0);
    let prev_slug = current_index.checked_sub(1).and_then(|i| SLUGS.get(i));
    let next_slug = SLUGS.get(current_index + 1);

    rsx! {
        link { rel: "stylesheet", href: BLOG_CSS }
        div { id: "blog-container",
            Markdown { content: markdown_content }
            div { class: "blog-navigation",
                if let Some(prev) = prev_slug {
                    rsx! {
                        Link {
                            to: Route::BlogPost { slug: prev.clone() },
                            class: "blog-nav-button",
                            "← {prev}"
                        }
                    }
                }
                if let Some(next) = next_slug {
                    rsx! {
                        Link {
                            to: Route::BlogPost { slug: next.clone() },
                            class: "blog-nav-button",
                            "{next} →"
                        }
                    }
                }
            }
        }
    }
}
