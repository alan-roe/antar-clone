use std::marker::PhantomData;

use crate::{
    system::{ChatId, Content, Message, Messages, Sender, Colour},
    web::{text_colour_from_bg, ToStyle},
};
use leptos::{html::*, *};
use web_sys::KeyboardEvent;

/// A Chat component
#[component]
pub fn Messages<ID, M, MS>(
    #[prop(optional)] test_id: String,
    #[prop(optional)] _phantom: PhantomData<(ID, M)>,
    messages: ReadSignal<MS>,
) -> impl IntoView
where
    ID: ChatId + Copy + ToString + 'static,
    M: Message + Clone + 'static,
    MS: Messages<ID, M> + Clone + 'static,
{
    let testing = !test_id.is_empty();

    let mut element = view! {
        <ul>
        <For
            each={messages}
            key=|message| message.0
            children=move |(id, message)| {
                let (message, set_message) = create_signal(message);
                view! {
                    <div>
                        { move || if testing {
                            view! {
                                <Message test_id=id.to_string() message=message />
                            }
                        } else {
                            view! {
                                <Message message=message />
                            }
                        } }
                        <button
                            on:click = move |_| {
                                set_message.update(|message| {
                                    message.update_content(|content| {
                                        let Content::Text(text) = content;
                                        *text += "YA JUST GOT UPDATED";
                                    });
                                    message.update_sender(|sender| {
                                        sender.update_colour(|colour| {
                                            *colour = Colour::Rgb(100, 240, 255)
                                        });
                                    })
                                })
                            }
                        >
                            "Edit"
                        </button>
                    </div>
                }
            }
        />
    </ul>
    };

    if testing {
        element = element.attr("data-testid", test_id);
    }

    element
}

#[component]
pub fn Message<M>(#[prop(optional)] test_id: String, #[prop(into)] message: Signal<M>) -> impl IntoView
where
    M: Message + 'static,
{
    let sender = Signal::derive(move || message.with(|message| message.sender().to_owned()));
    let name = Signal::derive(move || sender.with(|sender| sender.name().to_owned()));
    let bg_colour = Signal::derive(move || sender.with(|sender| sender.colour().to_bg_color()));
    let text_colour = Signal::derive(move || sender.with(|sender| text_colour_from_bg(*sender.colour()).to_color()));
    let content = Signal::derive(move || message.with(|message| {
        let Content::Text(text) = message.content();
        text.to_owned()
    }));

    let mut element: HtmlElement<Div> = view! {
        <div>
            <div>
                {name}
            </div>
            <div
                style=Signal::derive(move || bg_colour.with(|bg_colour| text_colour.with(|text_colour| bg_colour.to_owned() + text_colour )) )
            >
                { content }
            </div>
        </ div>
    };

    if !test_id.is_empty() {
        element = element.attr("data-testid", test_id);
    }

    element
}

#[component]
pub fn Input<F: FnMut(String) + 'static>(value: RwSignal<String>, mut on_submit: F) -> impl IntoView {
    let mut submit = move |evt : KeyboardEvent| {
        (on_submit)(event_target_value(&evt))
    };
    view! {
        <textarea
            on:keydown= move |evt| {
                if !evt.shift_key() && &evt.key() == "Enter" {
                    evt.prevent_default();
                    if !value.with(|value| value.trim().is_empty()) {
                        submit(evt);
                    }
                }
            }
            on:input= move |evt| {
                value.update(|value| *value = event_target_value(&evt));
            }
            prop:value=value
            placeholder="Start Talking..."
        />
    }
}
