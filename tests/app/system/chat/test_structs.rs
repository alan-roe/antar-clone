use let_me_talk::system::{Sender, Content, Message, Colour};


#[derive(PartialEq, Clone, Debug)]
pub struct TestSender {
    name: String,
}

impl TestSender {
    pub fn new(name: impl ToString) -> Self {
        TestSender {
            name: name.to_string(),
        }
    }
}

impl Default for TestSender {
    fn default() -> Self {
        TestSender {
            name: "Test Sender".to_string(),
        }
    }
}

impl Sender for TestSender {
    fn name(&self) -> &str {
        &self.name
    }

    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    fn colour(&self) -> &Colour {
        &Colour::BLACK
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