#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

// const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string());
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // link { rel: "stylesheet", href: "main.css" }
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            Link {
                to: Route::Blog {
                    id: count()
                },
                "Go to blog"
            }
        }
        div {
            h1 { class: "text-green-500 text-4xl font-bold", "Feed" }
        }
        div {
            h1 { class: "text-green-500 text-4xl font-bold", "High-Five counter: {count}" }
            button { class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded", onclick: move |_| count += 1, "Up high!" }
            button { class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded", onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
