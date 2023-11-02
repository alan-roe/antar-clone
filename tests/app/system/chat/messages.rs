use let_me_talk::system::{PMessages, Messages};
use uuid::Uuid;

use super::test_structs::*;

#[test]
fn iter_messages() {
    let mut messages = PMessages::new();

    (0..2).for_each(|i| {
        messages.insert(Uuid::new_v4(), TestMessage::new(format!("Test Message1 {i}")));
    });

    messages.iter().for_each(|(i, message)| {
        assert_eq!(message, &TestMessage::new(format!("Test Message1 {i}")));
    });
}
