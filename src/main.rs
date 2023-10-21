#![allow(non_snake_case, unused)]
mod colours;
mod components;
mod data;
mod pages;

use components::*;
use data::*;
use dioxus_signals::*;
use uuid::Uuid;

use std::{rc::Rc, ops::Deref};

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
        div { class: "flex flex-1 font-sans w-full h-screen",
            SideBar {}
            div {
                class: "grid gap-y-2 h-full w-full pb-2 bg-gray-50 items-center text-center",
                style: "grid-template-rows: auto minmax(0, 1fr);",
                h1 { class: "text-4xl font-bold mb-auto pb-2 w-full bg-gray-200", "Antar Clone" }
                // TODO Router for different pages
                div { class: "mx-auto px-2 w-full h-full max-w-3xl", 
                    if let Some(chat) = AppState::active_chat(cx).read().deref() {
                        rsx! {
                            ChatPage { chat: *chat }
                        }
                    }
                }
            }
        }
    })
}

fn SideBar(cx: Scope) -> Element {
    let chats = AppState::chats(cx);
    let rename = use_signal(cx, || false);

    // Just for mobile
    let sidebar_open = use_signal(cx, || false);
    let sidebar_style = use_signal(cx, || "hidden");
    let open_sidebar_style = use_signal(cx, || "flex");
    dioxus_signals::use_effect(cx, move || {
        if *sidebar_open.read() {
            sidebar_style.set("flex flex-col");
            open_sidebar_style.set("hidden");
        } else {
            open_sidebar_style.set("flex");
            sidebar_style.set("hidden");
        }
    });
    
    cx.render(rsx! {
        button {
            class: "bg-gray-950 text-gray-50 {open_sidebar_style} absolute md:hidden",
            "style": "height: 40px;",
            onclick: move |_| {
                sidebar_open.set(true);
            },
            "OPEN"
        }
        div {
            class: "{sidebar_style} md:flex md:flex-col bg-gray-300",
            "style": "width: 260px;",
            div { class: "flex",
                button {
                    class: "bg-gray-600",
                    onclick: move |_| {
                        AppState::new_chat(cx, Chat::new(*AppState::personas(cx).read().get_index(0).unwrap().0));
                    },
                    "New Chat"
                }
            }
            chats.read().chats().map(|(uuid, chat)| {
                let uuid = *uuid;
                    let selected = chats.read().active_chat_uuid()  == &Some(uuid);
                    rsx! {
                        div {
                            class: "flex gap-2 justify-between",
                            if *rename.read() && selected {
                                rsx!{
                                    textarea {
                                        rows: "1",
                                        oninput: move |evt| {
                                            if evt.value.ends_with('\n') {
                                                chats.write().save_active();
                                                rename.set(false);
                                            } else {
                                                AppState::active_chat(cx).read().unwrap().name.set(evt.value.clone())
                                            }
                                        },
                                        onkeyup: move |evt| {
                                            if evt.key() == Key::Enter {
                                                chats.write().save_active();
                                                rename.set(false);
                                            }
                                        },
                                        value: "{chat.name}"
                                    }
                                }
                            } else {
                                rsx!{
                                    button {
                                        class: if selected { "bg-gray-400"} else { "" },
                                        onclick: move |_| {
                                            AppState::set_active_chat(cx, uuid);
                                            sidebar_open.set(false);
                                        },
                                        "{chat.name}"
                                    }
                                }
                            }
                            if selected {
                                rsx!{
                                    div {
                                        class: "flex gap-2",
                                        button {
                                            class: "bg-gray-400",
                                            onclick: move |_| rename.set(true),
                                            "R"
                                        }
                                        button {
                                            class: "bg-gray-400",
                                            onclick: move |_| AppState::delete_active_chat(cx),
                                            "x"
                                        }
                                    }
                                }
                            }
                        }
                    }
                })
        }
    })
}
