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

use crate::{colours::Colour, pages::chat::ChatPage};

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    dioxus_desktop::launch_cfg(
        app,
        dioxus_desktop::Config::new()
            .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string()),
    );
    #[cfg(target_arch = "wasm32")]
    {
        wasm_logger::init(wasm_logger::Config::default());
        console_error_panic_hook::set_once();
        dioxus_web::launch(app);
    }
}

fn app(cx: Scope) -> Element {
    AppState::load(cx);

    cx.render(rsx! {
        div {
            class: "flex flex-1 font-sans w-full h-screen",
            SideBar {},
            div {
                class: "grid gap-y-2 h-full w-full pb-2 bg-gray-50 items-center text-center",
                style: "grid-template-rows: auto minmax(0, 1fr);",
                h1 { class: "text-4xl font-bold mb-auto pb-2 w-full bg-gray-200", "Antar Clone" }
                // TODO Router for different pages
                div { class: "mx-auto px-2 w-full h-full max-w-3xl",
                    ChatPage { chat: AppState::active_chat(cx) }
                }
            }
        }
    })
}

fn SideBar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "hidden md:flex md:bg-gray-300",
            "style": "width: 260px;"
        }
    })
}
