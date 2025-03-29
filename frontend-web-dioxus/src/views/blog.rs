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
    let mut hasRendered = use_signal(|| false);

    let render_math = async || -> Result<(), ()> {
        // if hasRendered() {
        //     return Ok(());
        // }
        let result = document::eval(
            r#"
                        console.log("Starting math rendering");
                        try {
                            if (typeof katex === 'undefined') {
                                throw new Error('KaTeX is not loaded yet');
                            }
                            document.querySelectorAll('.language-math').forEach(element => {
                                const tex = element.textContent;
                                const isInline = element.classList.contains('math-inline');
                                
                                katex.render(tex, element, {
                                    throwOnError: false,
                                    displayMode: !isInline,
                                });
                            });
                            console.log("Math rendering completed");
                        } catch (err) {
                            console.error("Math rendering error:", err);
                        }"#,
        )
        .await;

        if let Err(e) = result {
            println!("Failed to execute math rendering: {:?}", e);
        } else {
            // hasRendered.set(true);
        };
        Ok(())
    };

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
        div {
            class: "flex flex-col justify-center content-center",
            article {
                class: "prose",
                button {
                    class: "btn btn-primary hover:scale-105 transition-transform",
                    onclick: move |_| {
                        spawn(async move {
                            render_math().await;
                        });
                    },
                    "Render Math"
                }
                Markdown {
                    content: markdown_content,
                }
            }
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
                    math_flow: true,
                    math_text: true,
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
