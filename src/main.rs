#![allow(non_snake_case, unused)]
mod colours;
mod components;
mod data;
mod pages;

use components::*;
use data::*;
use dioxus_signals::*;
use uuid::Uuid;

use std::rc::Rc;

use dioxus::{
    html::input_data::keyboard_types::{Key, Modifiers},
    prelude::*,
};

use crate::{colours::Colour, pages::chat::Chat};

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
    cx.render(rsx! {
        div { class: "font-sans relative flex gap-2 min-h-screen max-h-screen flex-col overflow-hidden bg-gray-50 px-2 py-1 items-center",
            h1 { class: "text-4xl bg-center font-bold underline", "Antar Clone" }
            // TODO Router for different pages
            Chat {}
        }
    })
}
