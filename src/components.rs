use dioxus::prelude::*;
use dioxus_signals::Signal;
use uuid::Uuid;
use crate::data::*;
use crate::colours::*;

fn text_colour_from_bg((r, g, b): Rgb) -> Colour {
  if (u16::from(r) + u16::from(g) + u16::from(b)) >= (255 * 3 / 2) {
    Colour::Colour((0, 0, 0))
  } else {
    Colour::Colour((255, 255, 255))
  }
}

#[component]
pub fn MessageBox(cx: Scope, msgs: Signal<Messages>) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-col flex-grow border rounded-xl p-4 w-full max-w-2xl gap-2 overflow-y-scroll",
            for (i, msg) in msgs.read().msgs.iter().enumerate() {
                // PersonaMessage { msg: msg.msg.clone(), persona: msg.persona.clone() }
                div {
                    key: "{msg.uuid}",
                    class: if i == 0 {
                      "flex-col gap-2 mt-auto"
                    } else { "flex-col gap-2" },
                    if i == 0 || !msg.persona.eq(&msgs.read().msgs.get(i-1).unwrap().persona) {
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
                        span { "{msg.msg}" } }
                }
            }
        }
    })
}

#[component]
pub fn PersonaButton<'a>(
    cx: Scope,
    name: String,
    colour: Rgb,
    onclick: EventHandler<'a, MouseEvent>,
) -> Element {
    cx.render(rsx! {
        div {
          class: "flex flex-col items-center w-auto h-auto leading-none",
          button { onclick: move |evt| onclick.call(evt), PersonaIcon { colour: *colour } }
          p { class: "text-xs whitespace-nowrap", "{name}" }
        }
    })
}

pub fn PersonaMessage(cx: Scope<Message>) -> Element {
let text_colour = {
  let (r, g, b) = cx.props.persona.colour;
  if (u16::from(r) + u16::from(g) + u16::from(b)) >= (255 * 3 / 2) {
    Colour::Colour((0, 0, 0))
  } else {
    Colour::Colour((255, 255, 255))
  }
};
  cx.render(rsx! {
      div { class: "flex-col gap-2",
          div { class: "flex items-center",
              PersonaIcon { colour: cx.props.persona.colour }
              span { "{cx.props.persona.name}" }
          }
          div {
            class: "rounded-lg px-2 py-1 w-fit",
            style: r"
              {Colour::BgColour(cx.props.persona.colour)};
              {text_colour};",
            span { "{cx.props.msg}" } }
      }
  })
}

#[component]
fn PersonaIcon(cx: Scope, colour: Rgb) -> Element {
    cx.render(rsx! {
        div {
            svg {
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "currentColor",
                class: "w-9 h-9",
                style: "{Colour::Colour(*colour)}",
                path {
                    clip_rule: "evenodd",
                    fill_rule: "evenodd",
                    d: "M4.848 2.771A49.144 49.144 0 0112 2.25c2.43 0 4.817.178 7.152.52 1.978.292 3.348 2.024 3.348 3.97v6.02c0 1.946-1.37 3.678-3.348 3.97a48.901 48.901 0 01-3.476.383.39.39 0 00-.297.17l-2.755 4.133a.75.75 0 01-1.248 0l-2.755-4.133a.39.39 0 00-.297-.17 48.9 48.9 0 01-3.476-.384c-1.978-.29-3.348-2.024-3.348-3.97V6.741c0-1.946 1.37-3.68 3.348-3.97z"
                }
            }
        }
    })
}
