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
    // Determines initial value of shared state
    use_context_provider(|| Signal::new(PreviewState::Unset));
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {

    rsx! {
        div { class: "bg-slate-950 flex flex-row w-full",
            div { class: "w-1/2", Articles {}}
            div { class: "w-1/2", Preview {}}
        }
    }
}

fn Articles() -> Element {
    rsx! {
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

fn Preview() -> Element {
    // fetch from shared state
    let preview_state = consume_context::<Signal<PreviewState>>();

    match preview_state() {
        PreviewState::Unset => {
            rsx! { 
                div { class: "p-2",
                    div { class: "text-slate-400",
                        "hover over an article to preview it here" 
                    }
                }
            }
        },
        PreviewState::Loading => {
            rsx! { 
                div { class: "p-2",
                    div { class: "text-slate-400",
                        "loading..." 
                    }
                }
            }
        },
        PreviewState::Loaded(article) => {
            rsx! {
                div { class: "p-2",
                    div { class: "text-2xl text-slate-400", "story here"}
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
enum PreviewState {
    Unset, 
    Loading,
    Loaded(ArticlePageData),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArticlePageData {
    #[serde(flatten)]
    pub item: ArticleItem
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
    // Get preview state from shared state
    let mut preview_state = consume_context::<Signal<PreviewState>>();

    let ArticleItem {
        time,
        title,
        subtitle,
        author,
        ..
    } = &*article.read();

    let time = time.format("%D %l:%M %p");

    rsx! {
        div { class: "p-2 relative",
            onmouseenter: move |_event| {
                // Modify preview state
                // Dereferenced since we're messing w/ shared context!
                *preview_state
                    .write() = PreviewState::Loaded(ArticlePageData {
                        item: article(),
                    });
            },
            div { class: "text-2xl text-slate-300",
                "{title}"
            }
            div { class: "text-slate-400",
                "{subtitle}"
            }
            div { class: "text-slate-400 flex flex-row",
                div { "by {author}" }
                div { padding_left: "0.5rem", "{time}"}
            }
        }
    }
}
