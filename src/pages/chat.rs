use std::str::FromStr;

use crate::components::*;
use crate::data::*;
use dioxus::html::input_data::keyboard_types::Key;
use dioxus::html::input_data::keyboard_types::KeyboardEvent;
use dioxus::html::input_data::keyboard_types::Modifiers;
use indexmap::indexmap;
use uuid::Uuid;

use crate::colours::*;
use crate::storage::*;
use dioxus::prelude::*;
use dioxus_signals::*;

#[derive(Clone, Copy, Default, PartialEq, Props)]
pub struct ChatData {
    messages: Signal<Messages>,
    pub personas: Signal<Personas>,
    active_persona: Signal<Uuid>,
    current_message: Signal<String>,

    // Add Persona Dialog state
    new_persona_name: Signal<String>,
    new_persona_colour: Signal<Rgb>,
}

impl ChatData {
    fn init(self) -> Self {
        self.personas.set(get_storage("personas", || {
            Personas(indexmap! {Uuid::new_v4() => Persona {
                name: "Me".to_string(),
                colour: Rgb(0x49, 0x55, 0x65),
            }})
        }));

        self.active_persona
            .set(*self.personas.read().get_index(0).unwrap().0);
        self
    }
    fn on_send(&self) {
        let Self {
            messages,
            personas,
            active_persona,
            current_message,
            ..
        } = self;
        messages.write().msgs.push(Message {
            uuid: Uuid::new_v4(),
            persona: *active_persona.read(),
            msg: (*current_message.read()).clone(),
        });
        current_message.set("".to_string());
    }
}

pub fn use_chat_context(cx: Scope) -> ChatData {
    *use_context(cx).expect("no chat context provided")
}

#[component]
pub fn ChatPage(cx: Scope) -> Element {
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
        AddPersonaDialog {}
    })
}

fn AddPersonaDialog(cx: Scope) -> Element {
    let chat_data @ ChatData { personas, .. } = use_chat_context(cx);
    cx.render(rsx! {
        dialog {
            id: "addPersonaDialog",
            class: "p-4 pt-7, rounded-2xl max-w-full",
            div {
                class: "flex flex-col gap-2",
                div {
                    class: "grid grid-cols-2 place-content-between",
                    "My Personas",
                    button {
                        class: "bg-gray-300",
                        onclick: move |_| {
                            use_eval(cx)(r#"document.getElementById("addNewPersonaDialog").showModal();"#).unwrap();
                            use_eval(cx)(r#"document.getElementById("addPersonaDialog").close();"#).unwrap();

                        },
                        "Add New"
                    }
                }
                div {
                    class: "grid grid-cols-3 gap-4 max-w-full w-auto",
                    for (uuid, persona) in personas.read().iter() {
                        rsx! {
                            div {
                                key: "{uuid}",
                                class: "grid grid-rows-2 w-auto h-auto place-content-center place-items-center", 
                                PersonaIcon {
                                    colour: persona.colour
                                },
                                "{persona.name}"
                            }
                        }
                    }
                }}
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
        personas.write().push(Persona {
            name: new_persona_name.read().clone(),
            colour: *new_persona_colour.read(),
        });
        set_storage("personas", personas);
        use_eval(cx)(r#"document.getElementById("addNewPersonaDialog").close();"#).unwrap();
    };

    cx.render(rsx! {
        dialog {
            id: "addNewPersonaDialog",
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
    let ChatData {
        messages, personas, ..
    } = use_chat_context(cx);
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

fn MessageInput(cx: Scope) -> Element {
    let chat_data @ ChatData {
        current_message,
        active_persona,
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
                let persona_index = personas.read().get_index_of(&active_persona.read()).unwrap();
                if evt.key() == Key::Enter && !current_message.read().is_empty() {
                    chat_data.on_send();
                } else if evt.modifiers() == Modifiers::CONTROL
                    && evt.key() == Key::Character("]".into())
                {
                    if persona_index < personas.with(Personas::count) - 1 {
                        active_persona.set(*personas.read().get_index(persona_index+1).unwrap().0);
                    } else {
                        active_persona.set(*personas.read().get_index(0).unwrap().0);
                    }
                } else if evt.modifiers() == Modifiers::CONTROL
                    && evt.key() == Key::Character("[".into())
                {
                    if persona_index > 0 {
                        active_persona.set(*personas.read().get_index(persona_index-1).unwrap().0);
                    } else {
                        active_persona.set(*personas.read().get_index(personas.with(|personas| personas.count()-1)).unwrap().0);
                    }
                }
            },
            value: "{current_message.to_string()}"
        }
    })
}

fn BottomBar(cx: Scope) -> Element {
    let chat_data @ ChatData {
        active_persona,
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
        active_persona,
        ..
    } = use_chat_context(cx);
    cx.render(rsx!{
        personas.read().iter().map({ |(uuid, persona)|{
            let x = 0;
            rsx! {
                div {}
            }
        }
        })

    personas.read().iter().map( |(uuid, persona)| { 
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
    }})})
}
