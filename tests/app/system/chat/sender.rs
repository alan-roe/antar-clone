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

    sender.update_name(|name| *name = "Changed Sender".to_string());
    assert_eq!(sender.name(), "Changed Sender");

    sender.update_description(|description| *description = "Changed sender for testing.".to_string());
    assert_eq!(sender.description(), "Changed sender for testing.");

    sender.update_colour(|colour| *colour = Colour::BLACK);
    assert_eq!(sender.colour(), &Colour::BLACK);
}