#![allow(non_snake_case)]

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use tracing::Level;
use serde::{Serialize, Deserialize};

// const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {}
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
fn Home() -> Element {

    rsx! {
        div { "Home!" }
        ArticleListing {
            article: ArticleItem {
                id: 0,
                time: Utc::now(),
                title: "oh my lonely soul".to_string(),
                subtitle: "on isolation via the connected web".to_string(),
                author: "pasha_nemerov".to_string()
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArticleItem {
    pub id: i64,
    #[serde(with="chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub title: String,
    pub subtitle: String,
    pub author: String,
}

#[component]
fn ArticleListing(article: ReadOnlySignal<ArticleItem>) -> Element {
    let ArticleItem {
        id,
        time,
        title,
        subtitle,
        author
    } = &*article.read();

    let time = time.format("%D %l:%M %p");

    rsx! {
        div {padding: "0.5rem", position: "relative",
            div { font_size: "1.5rem", color: "gray",
                "{title}"
            }
            div { color: "gray",
                "{subtitle}"
            }
            div { display: "flex", flex_direction: "row", color: "gray",
                div { "by {author}" }
                div { padding_left: "0.5rem", "{time}"}
            }
        }
    }
}
