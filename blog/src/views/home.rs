use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn BlogIndex() -> Element {
    fn display_slug(slug: &str) -> String {
        // Could you pass the slug through a helper subfunction that would remove the unerlines and minuses, replace them with dashes, capitalize the words, and if it was longer then 30 chars, truncated it and added an elipsis?
        //
        let processed = slug.replace(['_', '-'], " ");
        let words: Vec<String> = processed
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => {
                        let first = c.to_uppercase().to_string();
                        let rest: String = chars.collect::<String>().to_lowercase();
                        format!("{}{}", first, rest)
                    }
                }
            })
            .collect();
        let title = words.join(" ");

        if title.len() > 30 {
            let mut truncated = title.chars().take(27).collect::<String>();
            truncated.push_str("...");
            truncated
        } else {
            title
        }
    }
    rsx! {
        div { class: "min-h-screen bg-base-200",
            // Hero section
            div { class: "hero bg-gradient-to-r from-primary to-secondary",
                div { class: "hero-content text-center text-neutral-content py-20",
                    div { class: "max-w-4xl",
                        h1 { class: "text-5xl font-bold mb-6", "Blog Posts" }
                        p { class: "text-xl opacity-90", "Explore our latest articles and tutorials" }
                    }
                }
            }

            // Blog posts grid
            div { class: "container mx-auto px-4 py-16",
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                    for slug in crate::views::blog::SLUGS.iter() {
                        div { class: "card bg-base-100 shadow-xl hover:shadow-2xl transition-shadow duration-300",
                            figure { class: "px-4 pt-4",
                                img {
                                    class: "rounded-xl h-48 w-full object-cover",
                                    src: "https://picsum.photos/400/300",
                                    alt: "Blog post thumbnail"
                                }
                            }
                            div { class: "card-body",
                                h2 { class: "card-title text-2xl", "{display_slug(&slug)}" }
                                p { class: "text-base-content/70", "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore." }
                                div { class: "card-actions justify-end mt-4",
                                    Link {
                                        class: "btn btn-primary hover:scale-105 transition-transform",
                                        to: Route::BlogPost { slug: slug.clone() },
                                        "Read More"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
#[component]
pub fn Home() -> Element {
    rsx! {
        BlogIndex {}
    }
}
