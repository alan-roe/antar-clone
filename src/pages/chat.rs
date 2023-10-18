use std::str::FromStr;

use crate::components::*;
use crate::data::*;
use dioxus::html::input_data::keyboard_types::Key;
use dioxus::html::input_data::keyboard_types::KeyboardEvent;
use dioxus::html::input_data::keyboard_types::Modifiers;
use indexmap::{indexmap, IndexSet};
use uuid::Uuid;

use crate::colours::*;
use dioxus::prelude::*;
use dioxus_signals::*;

#[component]
pub fn ChatPage(cx: Scope, chat: Chat) -> Element {
    let js = r#"
    if (!window.eventsRegistered) {
        document.addEventListener("keyup", (evt) => {
            if (evt.target.id !== "messageInput") return;
            else if (evt.key === "Tab") {
                evt.preventDefault();
            }
        });
        document.addEventListener("keydown", (evt) => {
            if (evt.target.id !== "messageInput") return;
            else if (evt.key === "Tab") {
                evt.preventDefault();
            }
          });
        window.eventsRegistered = true;
    }
    "#;
    use_eval(cx)(js);

    cx.render(rsx! {
        // TODO 3 Row Grid Layout
        div {
            class: "grid grid-rows-3 h-full w-full",
            style: "grid-template-rows: minmax(0, 1fr) auto auto;",
            div { MessageBox { chat: *chat } }
            div { MessageInput { chat: *chat } }
            div { BottomBar { chat: *chat } }
        }
        AddPersonaDialog { chat: *chat }
        AddNewPersonaDialog {}
    })
}

#[component]
fn AddPersonaDialog(cx: Scope, chat: Chat) -> Element {
    let Chat { added_personas, .. } = chat;
    let personas = AppData::personas(cx);
    cx.render(rsx! {
        dialog { id: "addPersonaDialog", class: "p-4 pt-7, rounded-2xl max-w-full",
            div { class: "flex flex-col gap-2",
                div { class: "grid grid-cols-2 place-content-between",
                    "My Personas"
                    button {
                        class: "bg-gray-300",
                        onclick: move |_| {
                            use_eval(cx)(r#"document.getElementById("addNewPersonaDialog").showModal();"#)
                                .unwrap();
                            use_eval(cx)(r#"document.getElementById("addPersonaDialog").close();"#).unwrap();
                        },
                        "Add New"
                    }
                }
                div { class: "grid grid-cols-3 gap-4 max-w-full w-auto",
                    personas.read().iter().map (|(uuid, persona)| {
                        let uuid = *uuid;
                        rsx! {
                            button {
                                key: "{uuid}",
                                class: "grid grid-rows-2 w-auto h-auto place-content-center place-items-center", 
                                onclick: move |_| { added_personas.write().insert(uuid); },
                                PersonaIcon {
                                    colour: persona.colour
                                },
                                "{persona.name}"
                            }
                        }
                    })
                }
            }
        }
    })
}

#[component]
fn AddNewPersonaDialog(cx: Scope) -> Element {
    let new_persona_name = use_state(cx, String::new);
    let new_persona_colour: &UseState<Rgb> = use_state(cx, Rgb::default);

    let personas = AppData::personas(cx);

    let add_persona = move || {
        personas.with_mut(|personas| {
            personas.push(Persona {
                name: new_persona_name.current().to_string(),
                colour: *new_persona_colour.current(),
            })
        });
        use_eval(cx)(r#"document.getElementById("addNewPersonaDialog").close();"#).unwrap();
    };

    cx.render(rsx! {
        dialog { id: "addNewPersonaDialog", class: "p-4 pt-7 rounded-2xl",
            // div within dialog to prevent display: flex causing dialog to show even when not open
            div { class: "flex flex-col gap-2",
                input {
                    placeholder: "Persona Name",
                    oninput: move |evt| { new_persona_name.set(evt.value.clone()) },
                    onkeyup: move |evt| {
                        if evt.key() == Key::Enter && !new_persona_name.current().is_empty() {
                            add_persona()
                        }
                    },
                    value: "{new_persona_name.current()}"
                }
                div { class: "flex flex-col gap-0",
                    "Choose a colour: "
                    input {
                        r#type: "color",
                        onchange: move |evt| new_persona_colour.set(Rgb::from_str(&evt.value).unwrap())
                    }
                }
                button {
                    class: "w-full bg-gray-950 hover:bg-gray-800 text-white font-bold py-2 px-4 shadow rounded-xl",
                    onclick: move |_| add_persona(),
                    AddNewPersonaButton {}
                }
            }
        }
    })
}

#[component]
fn AddNewPersonaButton(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "flex justify-between",
            div { "Add Persona" }
            svg {
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                stroke_width: "1.5",
                fill: "none",
                stroke: "currentColor",
                class: "w-6 h-6",
                path { stroke_linecap: "round", stroke_linejoin: "round", d: "M4.5 12.75l6 6 9-13.5" }
            }
        }
    })
}

#[component]
pub fn MessageBox(cx: Scope, chat: Chat) -> Element {
    let Chat { messages, .. } = chat;
    let personas = AppData::personas(cx);

    cx.render(rsx! {
        div { class: "flex flex-col flex-grow border rounded-xl p-4 min-h-full w-full max-w-2xl gap-2 max-h-full overflow-y-scroll",
            for (i , msg) in messages.read().msgs.iter().enumerate() {
                if let Some(persona) = personas.read().get(&msg.persona) {
                    // PersonaMessage { msg: msg.msg.clone(), persona: msg.persona.clone() }
                    rsx! {
                        div { key: "{msg.uuid}", class: if i == 0 { "flex-col gap-2 mt-auto" } else { "flex-col gap-2" },
                            if i == 0 || !msg.persona.eq(&messages.read().msgs.get(i-1).unwrap().persona) {
                                rsx! {
                                    div {
                                        class: "flex items-center",
                                        PersonaIcon { colour: persona.colour }
                                        span { "{persona.name}" }
                                    }
                                }
                            }
                            div {
                                class: "rounded-lg px-2 py-1 w-fit",
                                style: "{Colour::BgColour(persona.colour)} {text_colour_from_bg(persona.colour)}",
                                onmounted: move |cx2| {
                                    cx2.inner().scroll_to(ScrollBehavior::Smooth);
                                },
                                span { "{msg.msg}" }
                            }
                        }
                    }
                }
            }
        }
    })
}

#[component]
fn MessageInput(cx: Scope, chat: Chat) -> Element {
    let Chat {
        current_message,
        active_persona,
        added_personas,
        ..
    } = *chat;
    let personas = AppData::personas(cx);

    cx.render(rsx!{
        input {
            id: "messageInput",
            class: "flex p-2 h-full max-h-16 w-full rounded-xl bg-gray-200 outline-none hover:outline-none",
            placeholder: "Add message ...",
            onmounted: move |cx2| {
                cx2.inner().set_focus(true);
            },
            oninput: move |evt| { current_message.set(evt.value.clone()) },
            prevent_default: "onkeydown",
            onkeyup: move |evt| {
                let persona_index = added_personas
                    .read()
                    .iter()
                    .position(|r| *active_persona.read() == *r)
                    .unwrap();
                if evt.key() == Key::Enter && !current_message.read().is_empty() {
                    chat.send();
                } else if evt.modifiers() == Modifiers::SHIFT
                && evt.key() == Key::Tab
                {
                    if persona_index > 0 {
                        active_persona
                            .set(*added_personas.read().get_index(persona_index - 1).unwrap());
                    } else {
                        active_persona
                            .set(
                                *added_personas
                                    .read()
                                    .get_index(added_personas.with(|personas| personas.len() - 1))
                                    .unwrap(),
                            );
                    }
                } else if evt.key() == Key::Tab
                {
                    if persona_index < added_personas.with(IndexSet::len) - 1 {
                        active_persona
                            .set(*added_personas.read().get_index(persona_index + 1).unwrap());
                    } else {
                        active_persona.set(*added_personas.read().get_index(0).unwrap());
                    }
                } 
            },
            value: "{current_message}"
        }
    })
}

#[component]
fn BottomBar(cx: Scope, chat: Chat) -> Element {
    let Chat { active_persona, .. } = chat;

    let personas = AppData::personas(cx);

    cx.render(rsx!{
        div { class: "flex h-auto gap-x-2 w-full max-w-2xl",
            div { class: "flex items-end gap-x-2 h-auto w-full min-w-0 overflow-x-scroll",
                AddPersonaButton {
                    onclick: move |_| {
                        use_eval(cx)(r#"document.getElementById("addPersonaDialog").showModal();"#).unwrap();
                    }
                }
                PersonaSelect { chat: *chat }
            }
            button { class: "px-4 py-1 text-sm text-gray-900 font-semibold rounded-xl hover:text-gray-900 hover:bg-gray-200 hover:border-transparent focus:outline-none focus:ring-2 focus:ring-gray-100",
                SendIcon {}
            }
        }
    })
}

#[component]
fn PersonaSelect(cx: Scope, chat: Chat) -> Element {
    let Chat {
        active_persona,
        added_personas,
        ..
    } = chat;
    let personas = AppData::personas(cx);

    cx.render(rsx!{
        added_personas.read().iter().map(|uuid| {
            rsx! {
                if let Some(persona) = personas.read().get(uuid) {
                    let uuid = *uuid;
        
                    rsx! {
                        div { key: "{uuid}", class: "flex flex-col justify-center items-center",
                            if active_persona.read().eq(&uuid) {
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
                                rsx!{
                                    PersonaButton {
                                    name: persona.name.clone(),
                                    colour: persona.colour,
                                    onclick: move |_| {
                                        active_persona.set(uuid);
                                        use_eval(cx)(r#"document.getElementById("messageInput").focus();"#).unwrap();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    })
}
