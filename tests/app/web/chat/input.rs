use leptos::{*, leptos_dom::{*}, html::Textarea};
use let_me_talk::web::{TextInput, TextInputProps};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{Event, InputEvent};

use crate::web::data_test_id;

#[wasm_bindgen_test]
fn input_text() {
    let text = RwSignal::new("".to_string());

    mount_to_body(move || {
        view! {
            <TextInput test_id="input_text".to_string() on_submit= move |v| text.set(v) />
        }
    });
    let el = data_test_id("input_text");
    let el: web_sys::HtmlElement = el.unchecked_into();
    let event = InputEvent::new("input").unwrap();
    el.dispatch_event(&event).unwrap();
    assert_eq!(text(), el.text_content().unwrap());
}