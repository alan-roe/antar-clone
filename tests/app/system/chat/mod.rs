mod message;
mod messages;
mod sender;

pub mod test_structs;

use test_structs::*;
use uuid::Uuid;
use let_me_talk::system::{Chat, PChat, Content, PMessages, Messages};

type TestChat = PChat<Uuid, TestMessage, PMessages<Uuid, TestMessage>>;

#[test]
fn new_chat() {
    let chat = TestChat::new();
    assert_eq!(chat.last_message(), None);
}

#[test]
fn send_message() {
    let mut chat = TestChat::new();
    let test_message = TestMessage::new("Test Message");
    chat.send_message(test_message.clone());
    assert_eq!(chat.last_message(), Some(&test_message));
}

#[test]
fn delete_message() {
    let mut chat = TestChat::new();
    let test_message1 = TestMessage::new("Test Message1");
    let test_message2 = TestMessage::new("Test Message2");

    chat.send_message(test_message1.clone());
    let id = chat.send_message(test_message2.clone());

    assert_eq!(chat.last_message(), Some(&test_message2));

    chat.delete_message(&id);

    assert_eq!(chat.last_message(), Some(&test_message1));
}

#[test]
fn update_message() {
    let mut chat = TestChat::new();
    let test_message = TestMessage::new(" Test Message1 ");

    let id1 = chat.send_message(test_message);

    chat.update_content(&id1, |content| {
        let Content::Text(content) = content;
        *content = content.trim().to_string();
    });

    let expected_message = TestMessage::new("Test Message1");
    assert_eq!(chat.last_message(), Some(&expected_message));

    let sender2 = TestSender::new("Test Sender2");
    let moved_sender = sender2.clone();
    chat.update_sender(&id1, move |s| {
        *s = moved_sender;
    });

    let expected_message = TestMessage::with_sender("Test Sender2", "Test Message1");
    assert_eq!(chat.last_message(), Some(&expected_message));
}

#[test]
fn iter_messages() {
    let mut chat = TestChat::new();
    let test_message = TestMessage::new(" Test Message1 ");

    (0..2).for_each(|_| {
        chat.send_message(test_message.clone());
    });

    chat.messages().iter().for_each(|(_, message)| {
        assert_eq!(message, &test_message);
    });
}
