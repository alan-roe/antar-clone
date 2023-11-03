use let_me_talk::system::{Sender, Content, Message, Colour};


#[derive(PartialEq, Clone, Debug)]
pub struct TestSender {
    name: String,
    colour: Colour,
    description: String
}

impl TestSender {
    pub fn new(name: impl ToString) -> Self {
        TestSender {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

impl Default for TestSender {
    fn default() -> Self {
        TestSender {
            name: "Test Sender".to_string(),
            colour: Colour::BLACK,
            description: Default::default()
        }
    }
}

impl Sender for TestSender {
    fn name(&self) -> &str {
        &self.name
    }

    fn update_name<F: FnMut(&mut String)>(&mut self, mut f: F) {
        (f)(&mut self.name)
    }
    
    fn colour(& self) -> & Colour {
        &self.colour
    }

    fn update_colour<F: FnMut(&mut Colour)>(&mut self, mut f: F) {
        (f)(&mut self.colour)
    }
    
    fn description(&self) -> &str {
        &self.description
    }

    fn update_description<F: FnMut(&mut String)>(&mut self, mut f: F) {
        (f)(&mut self.description)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct TestMessage {
    content: Content,
    sender: TestSender,
}

impl TestMessage {
    pub fn new(content: impl ToString) -> Self {
        TestMessage {
            content: Content::text(content),
            sender: TestSender::default(),
        }
    }

    pub fn with_sender(sender: impl ToString, content: impl ToString) -> Self {
        TestMessage {
            content: Content::text(content),
            sender: TestSender::new(sender),
        }
    }
}

impl Message for TestMessage {
    type S = TestSender;
    fn content(&self) -> &Content {
        &self.content
    }

    fn sender(&self) -> &TestSender {
        &self.sender
    }

    fn update_content<F: FnOnce(&mut Content)>(&mut self, update_f: F) {
        (update_f)(&mut self.content)
    }

    fn update_sender<F: FnOnce(&mut TestSender)>(&mut self, update_f: F) {
        (update_f)(&mut self.sender)
    }
}