use let_me_talk::{system::{PMessages, Messages}, web::Messages};
use uuid::Uuid;
use wasm_bindgen_test::*;

use leptos::*;

use crate::{TestMessage, web::{data_test_id, chat::get_message_content}};

type TestMessages = PMessages<Uuid, TestMessage>;

#[wasm_bindgen_test]
fn list_messages() {
    let (messages, set_messages) = create_signal(TestMessages::new());
    let ids: Vec<Uuid> = (0..3).map(|_| Uuid::new_v4()).collect();
    set_messages.update(|messages| {
        messages.insert(ids[0], TestMessage::new("Test Message 1"));
        messages.insert(ids[1], TestMessage::new("Test Message 2"));
        messages.insert(ids[2], TestMessage::new("Test Message 3"));
    });
    mount_to_body(move || view! { <Messages test_id="messages".to_string() messages= messages/> });

    let el = data_test_id("messages");
    assert_eq!(el.child_element_count(), 3);

    assert_eq!(&get_message_content(ids[0].to_string()), "Test Message 1");
    assert_eq!(&get_message_content(ids[1].to_string()), "Test Message 2");
    assert_eq!(&get_message_content(ids[2].to_string()), "Test Message 3");
}
