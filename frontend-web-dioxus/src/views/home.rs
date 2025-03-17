use crate::components::{Echo, Hero};
use dioxus::prelude::*;

#[component]
pub fn BlogIndex() -> Element {
    rsx! {
        div { class: "blog-index",
            h1 { "All Blog Posts" }
            ul {
                for slug in SLUGS.iter() {
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
