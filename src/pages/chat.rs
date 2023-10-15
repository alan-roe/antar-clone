use std::str::FromStr;

use crate::components::*;
use crate::data::*;
use dioxus::html::input_data::keyboard_types::Key;
use dioxus::html::input_data::keyboard_types::KeyboardEvent;
use dioxus::html::input_data::keyboard_types::Modifiers;
use uuid::Uuid;

use crate::colours::*;
use dioxus::prelude::*;
use dioxus_signals::*;
use crate::storage::*;

#[derive(Clone, Copy, Default, PartialEq, Props)]
struct ChatData {
    messages: Signal<Messages>,
    personas: Signal<Personas>,
    persona_index: Signal<usize>,
    current_message: Signal<String>,

    // Add Persona Dialog state
    new_persona_name: Signal<String>,
    new_persona_colour: Signal<Rgb>,
}

impl ChatData {
    fn init(self) -> Self {
        self.personas.set(get_storage("personas", || Personas(
            vec![Persona {
                uuid: Uuid::new_v4(),
                name: "Me".to_string(),
                colour: Rgb(0x49, 0x55, 0x65),
            }]
        )));
        self
    }
    fn on_send(&self) {
        let Self {
            messages,
            personas,
            persona_index,
            current_message,
            ..
        } = self;
        messages.write().msgs.push(Message {
            uuid: Uuid::new_v4(),
            persona: personas.with(|personas| personas.get(*persona_index.read()).unwrap().clone()),
            msg: (*current_message.read()).clone(),
        });
        current_message.set("".to_string());
    }
}

fn use_chat_context(cx: Scope) -> ChatData {
    *use_context(cx).expect("no chat context provided")
}

pub fn Chat(cx: Scope) -> Element {
    // Shared State
    use_context_provider(cx, || ChatData::default().init());

    cx.render(rsx! {
        // TODO 3 Row Grid Layout
        div {
            class: "grid grid-rows-3 h-full w-full",
            style: "grid-template-rows: minmax(0, 1fr) auto auto;",
            div {MessageBox {}}
            div {MessageInput {}}
            div {BottomBar {}}
        }
        AddNewPersonaDialog{}
    })
}

fn AddNewPersonaDialog(cx: Scope) -> Element {
    let chat_data @ ChatData {
        new_persona_name,
        new_persona_colour,
        personas,
        ..
    } = use_chat_context(cx);

    let add_persona = move || {
        personas.write().push(
            Persona {
                uuid: Uuid::new_v4(),
                name: new_persona_name.read().clone(),
                colour: *new_persona_colour.read()
            }
        );
        set_storage("personas", personas);
        use_eval(cx)(r#"document.getElementById("addPersonaDialog").close();"#).unwrap();
    };

    cx.render(rsx! {
        dialog {
            id: "addPersonaDialog",
            class: "p-4 pt-7 rounded-2xl",
            // div within dialog to prevent display: flex causing dialog to show even when not open
            div {
                class: "flex flex-col gap-2",
                input {
                    placeholder: "Persona Name",
                    oninput: move |evt| { new_persona_name.set(evt.value.clone()) },
                    onkeyup: move |evt| {
                        if evt.key() == Key::Enter && !new_persona_name.read().is_empty() {
                            add_persona()
                        } 
                    },
                    value: "{new_persona_name.read()}"
                }
                div {
                    class: "flex flex-col gap-0",
                    "Choose a colour: "
                    input {
                        r#type: "color",
                        onchange: move |evt| new_persona_colour.set(Rgb::from_str(&evt.value).unwrap())
                    }
                }
                button {
                    class: "w-full bg-gray-950 hover:bg-gray-800 text-white font-bold py-2 px-4 shadow rounded-xl",
                    onclick: move |_| add_persona(),
                    AddNewPersonaButton{}
                }
            }
            
        }
    })
}

#[component]
fn AddNewPersonaButton(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex justify-between",
            div {"Add Persona",}
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
pub fn MessageBox(cx: Scope) -> Element {
    let ChatData { messages, .. } = use_chat_context(cx);
    cx.render(rsx! {
        div { class: "flex flex-col flex-grow border rounded-xl p-4 min-h-full w-full max-w-2xl gap-2 max-h-full overflow-y-scroll",
            for (i , msg) in messages.read().msgs.iter().enumerate() {
                // PersonaMessage { msg: msg.msg.clone(), persona: msg.persona.clone() }
                div { key: "{msg.uuid}", class: if i == 0 { "flex-col gap-2 mt-auto" } else { "flex-col gap-2" },
                    if i == 0 || !msg.persona.eq(&messages.read().msgs.get(i-1).unwrap().persona) {
                        rsx! {
                            div {
                                class: "flex items-center",
                                PersonaIcon { colour: msg.persona.colour }
                                span { "{msg.persona.name}" }
                            }
                        }
                    }
                    div {
                        class: "rounded-lg px-2 py-1 w-fit",
                        style: "{Colour::BgColour(msg.persona.colour)} {text_colour_from_bg(msg.persona.colour)}",
                        onmounted: move |cx2| {
                            cx2.inner().scroll_to(ScrollBehavior::Smooth);
                        },
                        span { "{msg.msg}" }
                    }
                }
            }
        }
    })
}

fn MessageInput(cx: Scope) -> Element {
    let chat_data @ ChatData {
        current_message,
        persona_index,
        personas,
        ..
    } = use_chat_context(cx);
    cx.render(rsx!{
        input {
            id: "messageInput",
            class: "flex p-2 h-full max-h-16 w-full rounded-xl bg-gray-200 outline-none hover:outline-none",
            placeholder: "Add message ...",
            oninput: move |evt| { current_message.set(evt.value.clone()) },
            onkeyup: move |evt| {
                if evt.key() == Key::Enter && !current_message.read().is_empty() {
                    chat_data.on_send();
                } else if evt.modifiers() == Modifiers::CONTROL
                    && evt.key() == Key::Character("]".into())
                {
                    if *persona_index.read() < personas.with(Personas::count) - 1 {
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
                        persona_index.set(personas.with(Personas::count) - 1)
                    }
                } else if evt.key() == Key::Character("[".into()) {
                    personas
                        .with_mut(|personas| {
                            personas
                                .push(Persona {
                                    uuid: Uuid::new_v4(),
                                    name: "Coder".to_string(),
                                    colour: Rgb(0x25, 0x25, 0x25),
                                });
                            personas
                                .push(Persona {
                                    uuid: Uuid::new_v4(),
                                    name: "Project Manager".to_string(),
                                    colour: Rgb(0xF2, 0x72, 0x4A),
                                });
                        });
                }
            },
            value: "{current_message.to_string()}"
        }
    })
}

fn BottomBar(cx: Scope) -> Element {
    let chat_data @ ChatData {
        persona_index,
        personas,
        ..
    } = use_chat_context(cx);
    cx.render(rsx!{
        div {
            class: "flex h-auto gap-x-2 w-full max-w-2xl",
            div { class: "flex items-end gap-x-2 h-auto w-full min-w-0 overflow-x-scroll",
                AddPersonaButton { onclick: move |_| {
                    use_eval(cx)(r#"document.getElementById("addPersonaDialog").showModal();"#).unwrap();
                } }
                PersonaSelect {
                }
            }
            button {
                class: "px-4 py-1 text-sm text-gray-900 font-semibold rounded-xl hover:text-gray-900 hover:bg-gray-200 hover:border-transparent focus:outline-none focus:ring-2 focus:ring-gray-100",
                SendIcon {}
            }
        }
    })
}

fn PersonaSelect(cx: Scope) -> Element {
    let ChatData {
        personas,
        persona_index,
        ..
    } = use_chat_context(cx);
    cx.render(rsx!{
    for (i , persona) in personas.read().iter().enumerate() {
        div { key: "{persona.uuid}", class: "flex flex-col justify-center items-center",
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
            PersonaButton {
                name: persona.name.clone(),
                colour: persona.colour,
                onclick: move |_| {
                    persona_index.set(i);
                    use_eval(cx)(r#"document.getElementById("messageInput").focus();"#).unwrap();
                }
            }
        }
    }})
}
