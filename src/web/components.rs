use std::marker::PhantomData;

use crate::{
    system::{ChatId, Content, Message, Messages, Sender},
    web::{text_colour_from_bg, ToStyle},
};
use leptos::{html::*, *};

/// A Chat component
#[component]
pub fn Messages<
    ID: ChatId + Copy + ToString + 'static,
    M: Message + Clone + 'static,
    MS: Messages<ID, M> + Clone + 'static,
>(
    #[prop(optional)] test_id: String,
    messages: ReadSignal<MS>,
    #[prop(optional)] _phantom: PhantomData<(ID, M)>,
) -> impl IntoView {
    let testing = !test_id.is_empty();

    let mut element = view! {
        <ul>
        <For
            each=messages
            key=|message| message.0
            children=move |(id, message)| {
                    if testing {
                        view! {
                            <Message test_id=id.to_string() message=message />
    }                } else {
                     view!{   <Message message=message />
                    }
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
pub fn Message<M: Message + Clone>(#[prop(optional)] test_id: String, message: M) -> impl IntoView {
    let sender = message.sender();
    let name = sender.name().to_owned();
    let bg_colour = sender.colour().to_bg_color();
    let text_colour = text_colour_from_bg(*sender.colour()).to_color();
    let Content::Text(content) = message.content();

    let mut element: HtmlElement<Div> = view! {
        <div>
            <div>
                { name }
            </div>
            <div
                style={bg_colour + &text_colour}
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
