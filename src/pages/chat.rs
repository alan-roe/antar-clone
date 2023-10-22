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

#[inline_props]
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
        div {
            class: "grid h-full w-full",
            style: "grid-template-rows: minmax(0, 1fr) auto auto;",
            div { MessageBox {
                messages: chat.messages
            } }
            div { MessageInput {
                current_message: chat.current_message,
                active_persona: chat.active_persona,
                added_personas: chat.added_personas,
                on_send: |_| chat.save(),
            } }
            div { BottomBar {
                active_persona: chat.active_persona,
                added_personas: chat.added_personas,
                on_send: |_| chat.save()
            } }
        }
        AddPersonaDialog { 
            added_personas: chat.added_personas,
            active_persona: chat.active_persona
         }
        AddNewPersonaDialog {
            on_create: move |(persona_name, persona_colour)| {
                let p_uuid = AppState::personas(cx).write().push(Persona {
                        name: persona_name,
                        colour: persona_colour,
                    });
                chat.add_persona(p_uuid);
                chat.save();
            }
        }
    })
}

#[inline_props]
fn AddPersonaDialog(cx: Scope, added_personas: Signal<IndexSet<Uuid>>, active_persona: Signal<Uuid>) -> Element {
    let personas = AppState::personas(cx);
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
                                onclick: move |evt| {
                                    if added_personas.write().insert(uuid) {
                                        use_eval(cx)(r#"
                                            document.getElementById("addPersonaDialog").close();
                                            document.getElementById("messageInput").focus();
                                        "#).unwrap();
                                        active_persona.set(uuid);
                                    }
                                },
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

#[inline_props]
pub fn MessageBox(cx: Scope, messages: Signal<Messages>) -> Element {
    let personas = AppState::personas(cx);

    cx.render(rsx! {
        div { class: "flex flex-col border rounded-xl p-4 min-h-full w-full gap-2 max-h-full overflow-y-scroll",
            for (i , msg) in messages.read().msgs.iter().enumerate() {
                if let Some(persona) = personas.read().get(&msg.persona) {
                    rsx! {
                        div { 
                            key: "{msg.uuid}",
                            // If it's the first message we want to push it to the bottom of the div
                            class: if i == 0 { "flex-col gap-2 mt-auto" } else { "flex-col gap-2" },
                            // If it's the first message or a different persona than previous then render the persona info
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
                                class: "rounded-lg px-2 py-1 w-fit text-left",
                                style: "{Colour::BgColour(persona.colour)} {text_colour_from_bg(persona.colour)}",
                                onmounted: move |cx2| {
                                    if i == messages.read().msgs.len()-1 {
                                        cx2.inner().scroll_to(ScrollBehavior::Smooth);
                                    }
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

#[inline_props]
fn Message (cx: Scope, message: String, colour: Rgb) -> Element {
    let colour = *colour;
    cx.render(rsx!{
        div {
            class: "rounded-lg px-2 py-1 w-fit text-left",
            style: "{Colour::BgColour(colour)} {text_colour_from_bg(colour)}",
            onmounted: move |cx2| {
                cx2.inner().scroll_to(ScrollBehavior::Smooth);
            },
            span { "{message }" }
        }
    })
}

#[inline_props]
fn MessageInput<'a>(cx: Scope, current_message: Signal<String>, active_persona: Signal<Uuid>, added_personas: Signal<IndexSet<Uuid>>, on_send: EventHandler<'a, ()>) -> Element {
    let personas = AppState::personas(cx);
    let eval = use_eval(cx);
    cx.render(rsx!{
        textarea {
            id: "messageInput",
            class: "flex p-2 max-h-32 h-auto w-full rounded-xl bg-gray-200 outline-none hover:outline-none",
            rows: 1,
            placeholder: "Add message ...",
            onmounted: move |cx2| {
                cx2.inner().set_focus(true);
            },
            oninput: move |mut evt| {
                if evt.value.ends_with('\n') {
                    on_send.call(());
                    // Clear the element value for correct height resize
                    eval(r#"document.getElementById("messageInput").value = "";"#).unwrap();
                } else {
                    current_message.set(evt.value.clone());
                }
                eval(r#"
                    el = document.getElementById("messageInput");
                    el.style.height = "auto";
                    el.style.height = el.scrollHeight + "px";
                "#).unwrap();
            },
            prevent_default: "onkeydown onkeyup",
            onkeyup: move |evt| {
                let persona_index = added_personas
                    .read()
                    .iter()
                    .position(|r| *active_persona.read() == *r)
                    .unwrap();
                if evt.key() == Key::Enter && !current_message.read().is_empty() {
                    on_send.call(());
                    eval(r#"documnet.getElementById("messageInput").value = "";"#).unwrap();
                } else if evt.modifiers() == Modifiers::SHIFT && evt.key() == Key::Tab {
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
                } else if evt.key() == Key::Tab {
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

#[inline_props]
fn BottomBar<'a>(cx: Scope, active_persona: Signal<Uuid>, added_personas: Signal<IndexSet<Uuid>>, on_send: EventHandler<'a, ()>) -> Element {
    let personas = AppState::personas(cx);

    cx.render(rsx!{
        div { class: "flex h-auto gap-x-2 w-full",
            div { class: "flex items-end gap-x-2 h-auto w-full min-w-0 overflow-x-scroll",
                AddPersonaButton {
                    onclick: move |_| {
                        use_eval(cx)(r#"document.getElementById("addPersonaDialog").showModal();"#).unwrap();
                    }
                }
                PersonaSelect {
                    active_persona: *active_persona,
                    added_personas: *added_personas,
                }
            }
            button {
                class: "px-4 py-1 text-sm text-gray-900 font-semibold rounded-xl hover:text-gray-900 hover:bg-gray-200 hover:border-transparent focus:outline-none focus:ring-2 focus:ring-gray-100",
                onclick: move |_| {
                    on_send.call(());
                    use_eval(cx)(r#"document.getElementById("messageInput").focus();"#).unwrap();
                },

                SendIcon {}
            }
        }
    })
}

#[inline_props]
fn PersonaSelect(cx: Scope, active_persona: Signal<Uuid>, added_personas: Signal<IndexSet<Uuid>>) -> Element {
    let personas = AppState::personas(cx);

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
