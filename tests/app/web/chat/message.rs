use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

use leptos::*;
use web_sys::HtmlElement;

use let_me_talk::system::{PMessage, Sender};
use let_me_talk::web::{text_colour_from_bg, Message, ToStyle};

use crate::system::test_structs::*;

#[wasm_bindgen_test]
fn show_message() {
    let sender = TestSender::default();
    let message = PMessage::new(sender.clone(), "New Message!".into());
    mount_to_body(move || view! { <Message message= message.clone()/> });

    let document = leptos::document();
    let div = document.query_selector("div").unwrap().unwrap();

    let node_list = div.query_selector_all("div").unwrap();
    assert_eq!(
        node_list.item(0).unwrap().text_content(),
        Some("Test Sender".to_string())
    );

    let message_contents = node_list.item(1).unwrap();
    assert_eq!(
        message_contents.text_content(),
        Some("New Message!".to_string())
    );

    let contents_style = message_contents
        .dyn_into::<HtmlElement>()
        .unwrap()
        .get_attribute("style")
        .unwrap();

    assert!(contents_style.contains(&sender.colour().to_bg_color()));
    assert!(contents_style.contains(&(text_colour_from_bg(*sender.colour())).to_color()));
}
