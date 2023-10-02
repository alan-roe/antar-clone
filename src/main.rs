#![allow(non_snake_case, unused)]
use std::rc::Rc;

use dioxus::{
    html::input_data::keyboard_types::{Key, Modifiers},
    prelude::*,
};
// use dioxus_fullstack::prelude::*;

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

type Personas = Vec<Persona>;

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Messages { msgs: Vec::new() });
    let msgs = use_shared_state::<Messages>(cx).unwrap();
    let personas: &UseRef<Personas> = use_ref(cx, || {
        vec![Persona {
            name: "Me".to_string(),
            colour: "text-grey-600".to_string(),
        }]
    });
    {
        // personas.with_mut(|personas| {
        //     // personas.push(Persona {
        //     //     name: "Me".to_string(),
        //     //     colour: "text-grey-600".to_string(),
        //     // });
        //     personas.push(Persona {
        //         name: "Coder".to_string(),
        //         colour: "text-blue-600".to_string(),
        //     });
        //     personas.push(Persona {
        //         name: "Project Manager".to_string(),
        //         colour: "text-green-600".to_string(),
        //     });
        // });
    }
    let input_str = use_state(cx, || "".to_string());

    let persona_index: &UseState<usize> = use_state(cx, || 0);
    let personas_lock = personas.read();

    let scroll_pos = use_state(cx, || 0);

    let send_msg = || {
        msgs.write().msgs.push(Message {
            persona: personas
                .with(|personas| personas.get(*persona_index.current()).unwrap().clone()),
            msg: (*input_str.current()).clone(),
        });
        input_str.set("".to_string());
    };

    use_shared_state_provider::<Option<Rc<MountedData>>>(cx, || None);

    cx.render(rsx! {
        div { class: "font-sans relative flex min-h-screen max-h-screen flex-col overflow-hidden bg-gray-50 px-2 py-1 items-center",
            h1 { class: "text-4xl bg-center font-bold underline", "Antar Clone" }
            MessageBox {
            }
            input {
                class: "flex my-2 max-w-2xl p-2 h-auto w-full rounded-xl bg-gray-100 outline-none focus:outline-none",
                placeholder: "Add message ...",
                oninput: move |evt| { input_str.set(evt.value.clone()) },
                onkeyup: move |evt| {
                    if evt.key() == Key::Enter && !input_str.is_empty() {
                        send_msg();
                        // let message_box_el = use_shared_state::<Option<Rc<MountedData>>>(cx);
                        // if let Some(msg_box_el) = message_box_el {
                        //     println!("trying to scroll");
                        //     if let Some(msg_box_el) = msg_box_el.write().clone() {
                        //         msg_box_el.scroll_to(ScrollBehavior::Smooth);
                        //     }
                        // }
                    } else if evt.modifiers() == Modifiers::CONTROL
                        && evt.key() == Key::Character("]".into())
                    {
                        if *persona_index.current() < personas.with(Vec::len) - 1 {
                            persona_index.modify(|x| *x + 1);
                        } else {
                            persona_index.set(0);
                        }
                    } else if evt.modifiers() == Modifiers::CONTROL
                        && evt.key() == Key::Character("[".into())
                    {
                        if *persona_index.current() > 0 {
                            persona_index.modify(|x| *x - 1);
                        } else {
                            persona_index.set(personas.with(Vec::len)-1)
                        }
                    } else if evt.key() == Key::Character("[".into()) {
                        personas.with_mut(|personas| {
                            personas.push(Persona {
                                name: "Coder".to_string(),
                                colour: "text-blue-600".to_string(),
                            });
                            personas.push(Persona {
                                name: "Project Manager".to_string(),
                                colour: "text-green-600".to_string(),
                            });
                        });
                    }
                },
                value: "{input_str.to_string()}"
            }
            div { class: "flex h-auto gap-x-2 w-full max-w-2xl",
                AddPersonaButton {
                    onclick: move |_| {}
                }
                div { class: "flex items-start gap-x-2 h-auto w-full min-w-0 pb-3 overflow-x-scroll",
                    for (i, persona) in personas_lock.iter().enumerate() {
                        div {
                            class: "flex flex-col justify-center items-center",
                            PersonaButton { name: persona.name.clone(), colour: persona.colour.clone(),
                                onclick: move |_| persona_index.set(i) }
                            if i == *persona_index.get() {
                                rsx! {
                                    svg {
                                        view_box: "0 0 24 24",
                                        stroke: "currentColor",
                                        stroke_width: "1.5",
                                        fill: "none",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        class: "w-4 h-4",
                                        path { d: "M4.5 15.75l7.5-7.5 7.5 7.5", stroke_linejoin: "round", stroke_linecap: "round" }
                                }}
                            }
                        }

                    }
                }
                button {
                    class: "pb-3 px-4 py-1 text-sm text-gray-900 font-semibold rounded-xl hover:text-gray-900 hover:bg-gray-200 hover:border-transparent focus:outline-none focus:ring-2 focus:ring-gray-100",
                    onclick: move |_| send_msg(),
                    SendIcon {}
                }
            }
        }
    })
}

#[derive(Clone, Debug, Default, PartialEq, Props)]
struct Message {
    pub msg: String,
    pub persona: Persona,
}
struct Messages {
    pub msgs: Vec<Message>,
}

fn PersonaMessage(cx: Scope<Message>) -> Element {
    let bg_colour = cx.props.persona.colour.replace("text", "bg");
    println!("{bg_colour}");
    cx.render(rsx! {
        div { class: "flex-col gap-2",
            div { class: "flex items-center",
                PersonaIcon { colour: cx.props.persona.colour.clone() }
                span { "{cx.props.persona.name}" }
            }
            div { class: "{bg_colour} rounded-lg px-2 py-1 w-fit", span { "{cx.props.msg}" } }
        }
    })
}

#[component]
fn MessageBox(cx: Scope) -> Element {
    let msgs = use_shared_state::<Messages>(cx).unwrap();
    // let scroll_top = use_shared_state::<Option<_>>(cx).unwrap();
    cx.render(rsx! {
        div { 
            class: "flex flex-col flex-grow border rounded-lg p-4 w-full max-w-2xl gap-2 overflow-y-scroll",
            for (i, msg) in msgs.read().msgs.iter().enumerate() {
                // PersonaMessage { msg: msg.msg.clone(), persona: msg.persona.clone() }
                div {
                    class: "flex-col gap-2",
                    if i == 0 || !msg.persona.eq(&msgs.read().msgs.get(i-1).unwrap().persona) {
                        rsx! {
                            div { class: "flex items-center",
                                PersonaIcon { colour: msg.persona.colour.clone() }
                                span { "{msg.persona.name}" }
                            }
                        }
                    }
                    div {
                        class: "{msg.persona.colour.replace(\"text\", \"bg\")} rounded-lg px-2 py-1 w-fit",
                        onmounted: move |cx2| {
                            // let inner = cx2.inner().scroll_to(ScrollBehavior::Smooth);
                            cx2.inner().scroll_to(ScrollBehavior::Smooth);
                            println!("setting scroll element");
                            // *(use_shared_state::<Option<Rc<MountedData>>>(cx).unwrap().write()) = Some(inner);
                        },
                        span { "{msg.msg}" } }
                }
            }
        }
    })
}

#[derive(Clone, Debug, Default, PartialEq, Props)]
struct Persona {
    pub name: String,
    pub colour: String,
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
        div { class: "flex flex-col items-center w-auto h-auto leading-none pb-3",
            button { class: "leading-none ", onclick: |evt| onclick.call(evt), AddPersonaIcon {}}
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

#[component]
fn PersonaButton<'a>(
    cx: Scope,
    name: String,
    colour: String,
    onclick: EventHandler<'a, MouseEvent>,
) -> Element {
    cx.render(rsx! {
        div { class: "flex flex-col items-center w-auto h-auto leading-none",
            button { onclick: move |evt| onclick.call(evt), PersonaIcon { colour: colour.clone() } }
            p { class: "text-xs whitespace-nowrap", "{name}" }
        }
    })
}

#[component]
fn PersonaIcon(cx: Scope, colour: String) -> Element {
    cx.render(rsx! {
        div {
            svg {
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "currentColor",
                class: "w-9 h-9 {colour}",
                path {
                    clip_rule: "evenodd",
                    fill_rule: "evenodd",
                    d: "M4.848 2.771A49.144 49.144 0 0112 2.25c2.43 0 4.817.178 7.152.52 1.978.292 3.348 2.024 3.348 3.97v6.02c0 1.946-1.37 3.678-3.348 3.97a48.901 48.901 0 01-3.476.383.39.39 0 00-.297.17l-2.755 4.133a.75.75 0 01-1.248 0l-2.755-4.133a.39.39 0 00-.297-.17 48.9 48.9 0 01-3.476-.384c-1.978-.29-3.348-2.024-3.348-3.97V6.741c0-1.946 1.37-3.68 3.348-3.97z"
                }
            }
        }
    })
}
