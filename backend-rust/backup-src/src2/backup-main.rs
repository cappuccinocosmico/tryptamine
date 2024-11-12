#![allow(non_snake_case)]

use dioxus::prelude::*;
use manganis::*;
const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));
pub const TEST_IMG: manganis::ImageAsset = manganis::mg!(image("./public/example-image.png")
    // Manganis uses the builder pattern inside the macro. You can set the image size in pixels at compile time to send the smallest possible image to the client
    .size(500, 300)
    // You can also convert the image to a web friendly format at compile time. This can make your images significantly smaller
    .format(ImageType::Webp)
    // You can even tell manganis to preload the image so it's ready to be displayed as soon as it's needed
    .preload());
fn main() {
    launch(app);
}

pub fn app() -> Element {
    rsx!(
        div {
            header {
                class: "text-gray-400 body-font",
                // you can use optional attributes to optionally apply a tailwind class
                div { class: "container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center",
                    a { class: "flex title-font font-medium items-center text-white mb-4 md:mb-0",
                        StacksIcon {}
                        span { class: "ml-3 text-xl", "Tryptamine: A High Performance Rust Based Fractal Generator " }
                    }
                    nav { class: "md:ml-auto flex flex-wrap items-center text-base justify-center",
                    }
                    button { class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0",
                        "Try out our main application Kessler"
                        RightArrowIcon {}
                    }
                }
            }

            section { class: "text-gray-400 bg-gray-900 body-font",
                div { class: "container mx-auto flex px-5 py-24 md:flex-row flex-col items-center",
                    div { class: "lg:flex-grow md:w-1/2 lg:pr-24 md:pr-16 flex flex-col md:items-start md:text-left mb-16 md:mb-0 items-center text-center",
                        h1 { class: "title-font sm:text-4xl text-3xl mb-4 font-medium text-white",
                            br { class: "hidden lg:inline-block" }
                            "Dioxus Sneak Peek"
                        }
                        p { class: "mb-8 leading-relaxed",

                            "Dioxus is a new UI framework that makes it easy and simple to write cross-platform apps using web
                            technologies! It is functional, fast, and portable. Dioxus can run on the web, on the desktop, and
                            on mobile and embedded platforms."
                        }
                        div { class: "flex justify-center",
                            button { class: "inline-flex text-white bg-indigo-500 border-0 py-2 px-6 focus:outline-none hover:bg-indigo-600 rounded text-lg",
                                "Learn more"
                            }
                            button { class: "ml-4 inline-flex text-gray-400 bg-gray-800 border-0 py-2 px-6 focus:outline-none hover:bg-gray-700 hover:text-white rounded text-lg",
                                "Build an app"
                            }
                        }
                    }
                    div { class: "lg:max-w-lg lg:w-full md:w-1/2 w-5/6",
                        img {
                            class: "object-cover object-center rounded",
                            src: "{TEST_IMG}",
                            referrerpolicy: "no-referrer",
                            alt: "hero"
                        }
                    }
                }
            }
        }
    )
}

pub fn StacksIcon() -> Element {
    rsx!(
        svg {
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            class: "w-10 h-10 text-white p-2 bg-indigo-500 rounded-full",
            view_box: "0 0 24 24",
            path { d: "M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" }
        }
    )
}

pub fn RightArrowIcon() -> Element {
    rsx!(
        svg {
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            class: "w-4 h-4 ml-1",
            view_box: "0 0 24 24",
            path { d: "M5 12h14M12 5l7 7-7 7" }
        }
    )
}
