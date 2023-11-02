use let_me_talk::system::*;
use super::test_structs::*;

#[test]
fn new_message() {
    let sender = TestSender::default();
    let content = Content::text("content");
    let message = PMessage::new(sender.clone(), content.clone());

    assert_eq!(message.sender(), &sender);
    assert_eq!(message.content(), &content);
}

#[test]
fn edit_content() {
    let sender = TestSender::default();
    let content = Content::text("content");
    let mut message = PMessage::new(sender.clone(), content.clone());

    message.update_content(|content| {
        let Content::Text(content) = content;
        content.pop();
    });

    assert_eq!(message.content(), &Content::text("conten"));
}

#[test]
fn edit_sender() {
    let sender = TestSender::default();
    let content = Content::text("content");
    let mut message = PMessage::new(sender.clone(), content.clone());

    message.update_sender(|sender| {
        *sender = TestSender::new("Updated Sender"); 
    });

    assert_eq!(message.sender(), &TestSender::new("Updated Sender"));

}