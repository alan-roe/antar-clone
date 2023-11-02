use leptos::*;
use crate::{system::{Message, Content, Sender}, web::{ToStyle, text_colour_from_bg}};

/// A Chat component
#[component]
pub fn Messages() -> impl IntoView {
    view! {
        
    }
}

#[component]
pub fn Message<M: Message + Clone>(message: M) -> impl IntoView {
    let message = message.clone();
    let sender = message.sender();
    let name = sender.name().to_owned();
    let bg_colour = sender.colour().to_bg_color();
    let text_colour = text_colour_from_bg(*sender.colour()).to_color();
    let Content::Text(content) = message.content();
    
    view! {
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
    }
}
