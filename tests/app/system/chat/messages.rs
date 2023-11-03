use let_me_talk::system::{PMessages, Messages};
use uuid::Uuid;

use super::test_structs::*;

#[test]
fn iter_messages() {
    let mut messages = PMessages::new();

    (0..2).for_each(|_| {
        let id = Uuid::new_v4(); 
        messages.insert(id, TestMessage::new(format!("Test Message1 {id}")));
    });

    messages.iter().for_each(|(i, message)| {
        assert_eq!(message, &TestMessage::new(format!("Test Message1 {i}")));
    });
}
