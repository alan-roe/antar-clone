#![allow(non_snake_case, unused)]
mod colours;
mod components;
mod data;
mod pages;
mod storage;

use components::*;
use data::*;
use dioxus_signals::*;
use uuid::Uuid;

use std::rc::Rc;

use dioxus::{
    html::input_data::keyboard_types::{Key, Modifiers},
    prelude::*,
};

use crate::storage::*;
use crate::{colours::Colour, pages::chat::ChatPage};

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    dioxus_desktop::launch_cfg(
        app,
        dioxus_desktop::Config::new()
            .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string()),
    );
    #[cfg(target_arch = "wasm32")]
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    AppData::load(cx);
    let app_data = Signal::new(AppData::use_app_context(cx));

    cx.render(rsx! {
        div { 
            class: "grid font-sans gap-y-2 min-h-screen min-w-full max-h-screen max-w-full bg-gray-50 items-center",
            style: "grid-template-rows: auto minmax(0, 1fr);",
            h1 { class: "text-4xl font-bold underline mx-auto mb-auto", "Antar Clone" }
            // TODO Router for different pages
            div {
                class: "w-full max-w-2xl h-full mx-auto",
                ChatPage {}
            }
        }
    })
}
