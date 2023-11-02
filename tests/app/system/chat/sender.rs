use let_me_talk::system::{PSender, Sender, Colour};

#[test]
fn new_sender() {
    let sender = PSender::new("Test Sender", "A sender for testing", Colour::BLACK);
    
    assert_eq!(sender.name(), "Test Sender");
    assert_eq!(sender.description(), "A sender for testing");
    assert_eq!(sender.colour(), &Colour::BLACK);
}

#[test]
fn change_sender() {
    let mut sender = PSender::new("Test Sender", "A sender for testing", Colour::BLACK);

    *sender.name_mut() = "Changed Sender".to_string();
    assert_eq!(sender.name(), "Changed Sender");

    *sender.description_mut() = "Changed sender for testing.".to_string();
    assert_eq!(sender.description(), "Changed sender for testing.");

    *sender.colour_mut() = Colour::BLACK;
    assert_eq!(sender.colour(), &Colour::BLACK);
}