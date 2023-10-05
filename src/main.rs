#![allow(non_snake_case, unused)]
mod components;
mod data;
mod colours;

use components::*;
use data::*;
use dioxus_signals::*;
use uuid::Uuid;

use std::rc::Rc;

use dioxus::{
    html::input_data::keyboard_types::{Key, Modifiers},
    prelude::*,
};

use crate::colours::Colour;

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
    // Shared State
    let msgs = use_signal(cx, || Messages { msgs: Vec::new() });
    let personas = use_signal(cx, || {
        vec![Persona {
            uuid: Uuid::new_v4(),
            name: "Me".to_string(),
            colour: (0x49, 0x55, 0x65),
        }]
    });
    let persona_index: Signal<usize> = use_signal(cx, || 0);

    // Local State
    let input_str = use_signal(cx, String::new);

    let send_msg = move || {
        msgs.write().msgs.push(Message {
            uuid: Uuid::new_v4(),
            persona: personas
                .with(|personas| personas.get(*persona_index.read()).unwrap().clone()),
            msg: (*input_str.read()).clone(),
        });
        input_str.set("".to_string());
    };

    cx.render(rsx! {
        div { 
            class: "font-sans relative flex gap-2 min-h-screen max-h-screen flex-col overflow-hidden bg-gray-50 px-2 py-1 items-center",
            h1 { 
                class: "text-4xl bg-center font-bold underline",
                "Antar Clone" 
            }
            MessageBox {
                msgs: msgs
            }
            input {
                class: "flex max-w-2xl p-2 h-auto w-full rounded-xl bg-gray-100 outline-none focus:outline-none",
                placeholder: "Add message ...",
                oninput: move |evt| { input_str.set(evt.value.clone()) },
                onkeyup: move |evt| {
                    if evt.key() == Key::Enter && !input_str.read().is_empty() {
                        send_msg();
                    } else if evt.modifiers() == Modifiers::CONTROL
                        && evt.key() == Key::Character("]".into())
                    {
                        if *persona_index.read() < personas.with(Vec::len) - 1 {
                            *persona_index.write() += 1;
                        } else {
                            persona_index.set(0);
                        }
                    } else if evt.modifiers() == Modifiers::CONTROL
                        && evt.key() == Key::Character("[".into())
                    {
                        if *persona_index.read() > 0 {
                            *persona_index.write() -= 1;
                        } else {
                            persona_index.set(personas.with(Vec::len)-1)
                        }
                    } else if evt.key() == Key::Character("[".into()) {
                        personas.with_mut(|personas| {
                            personas.push(Persona {
                                uuid: Uuid::new_v4(),
                                name: "Coder".to_string(),
                                colour: (0x25, 0x25, 0x25),
                            });
                            personas.push(Persona {
                                uuid: Uuid::new_v4(),
                                name: "Project Manager".to_string(),
                                colour: (0xF2, 0x72, 0x4A),
                            });
                        });
                    }
                },
                value: "{input_str.to_string()}"
            }
            div { class: "flex h-auto gap-x-2 w-full max-w-2xl",
                div { class: "flex items-end gap-x-2 h-auto w-full min-w-0 overflow-x-scroll",
                    AddPersonaButton {
                        onclick: move |_| {}
                    }
                    for (i, persona) in personas.read().iter().enumerate() {
                        div {
                            key: "{persona.uuid}",
                            class: "flex flex-col justify-center items-center",
                            if i == *persona_index.read() {
                                rsx! {
                                    svg {
                                        view_box: "0 0 24 24",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        fill: "none",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        class: "w-4 h-4",
                                        path { d: "M4.5 15.75l7.5-7.5 7.5 7.5", stroke_linejoin: "round", stroke_linecap: "round" }
                                }}
                            }
                            PersonaButton { name: persona.name.clone(), colour: persona.colour,
                                onclick: move |_| persona_index.set(i) }
                        }

                    }
                }
                button {
                    class: "px-4 py-1 text-sm text-gray-900 font-semibold rounded-xl hover:text-gray-900 hover:bg-gray-200 hover:border-transparent focus:outline-none focus:ring-2 focus:ring-gray-100",
                    onclick: move |_| send_msg(),
                    SendIcon {}
                }
            }
        }
    })
}

fn SendIcon(cx: Scope) -> Element {
    cx.render(rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "1.5",
            stroke: "currentColor",
            class: "w-8 h-8",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M6 12L3.269 3.126A59.768 59.768 0 0121.485 12 59.77 59.77 0 013.27 20.876L5.999 12zm0 0h7.5"
            }
        }
    })
}

#[component]
fn AddPersonaButton<'a>(cx: Scope, onclick: EventHandler<'a, MouseEvent>) -> Element {
    cx.render(rsx! {
        div { 
            class: "flex flex-col items-center text-black justify-end w-auto h-auto leading-none mr-3",
            button { class: "leading-none", onclick: |evt| onclick.call(evt), AddPersonaIcon {}}
            span { class: "text-xs", "Add" }
        }
    })
}

fn AddPersonaIcon(cx: Scope) -> Element {
    cx.render(rsx! {
        svg { class: "w-9 h-9", view_box: "0 0 24 24",
            svg {
                view_box: "0 0 24 24",
                stroke: "currentColor",
                stroke_width: "1.5",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                class: "w-6 h-6",
                path {
                    d: "M2.25 12.76c0 1.6 1.123 2.994 2.707 3.227 1.068.157 2.148.279 3.238.364.466.037.893.281 1.153.671L12 21l2.652-3.978c.26-.39.687-.634 1.153-.67 1.09-.086 2.17-.208 3.238-.365 1.584-.233 2.707-1.626 2.707-3.228V6.741c0-1.602-1.123-2.995-2.707-3.228A48.394 48.394 0 0012 3c-2.392 0-4.744.175-7.043.513C3.373 3.746 2.25 5.14 2.25 6.741v6.018z",
                    stroke_linecap: "round",
                    stroke_linejoin: "round"
                }
            }
            svg {
                view_box: "0 0 20 20",
                y: "-1.5",
                fill: "currentColor",
                xmlns: "http://www.w3.org/2000/svg",
                class: "w-5 h-5",
                path { d: "M10.75 6.75a.75.75 0 00-1.5 0v2.5h-2.5a.75.75 0 000 1.5h2.5v2.5a.75.75 0 001.5 0v-2.5h2.5a.75.75 0 000-1.5h-2.5v-2.5z" }
            }
        }
    })
}

