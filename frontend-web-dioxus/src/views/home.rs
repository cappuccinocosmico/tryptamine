use crate::components::{Echo, Hero};
use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn BlogIndex() -> Element {
    rsx! {
        div { class: "blog-index",
            h1 { "All Blog Posts" }
            ul {
                for slug in crate::views::blog::SLUGS.iter() {
                    li {
                        Link {
                            to: Route::BlogPost { slug: slug.clone() },
                            "{slug}"
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
        Hero {}
        Echo {}
        BlogIndex {}
    }
}
