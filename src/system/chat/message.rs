use super::Sender;

#[derive(Debug)]
#[derive(Clone, PartialEq)]
pub enum Content {
    Text(String)
}

impl Content {
    pub fn text(text: impl ToString) -> Self {
        Content::Text(text.to_string())
    } 
}

impl From<&str> for Content {
    fn from(value: &str) -> Self {
        Content::text(value)
    }
}

pub trait Message {
    type S: Sender;

    fn sender(&self) -> &Self::S;
    fn content(&self) -> &Content;
    fn update_sender<F: FnOnce(&mut Self::S)>(&mut self, update_f: F);
    fn update_content<F: FnOnce(&mut Content)>(&mut self, update_f: F);
}

#[derive(Clone, PartialEq, Debug)]
pub struct PMessage<S: Sender> {
    sender: S,
    content: Content,
}

impl<S: Sender> PMessage<S> {
    pub fn new(sender: S, content: Content) -> Self {
        Self { sender, content }
    }
}

impl<S: Sender> Message for PMessage<S> {
    type S = S;

    fn sender(&self) -> &Self::S {
        &self.sender
    }

    fn content(&self) -> &Content {
        &self.content
    }

    fn update_content<F: FnOnce(&mut Content)>(&mut self, update_f: F) {
        (update_f)(&mut self.content)
    }

    fn update_sender<F: FnOnce(&mut Self::S)>(&mut self, update_f: F) {
        (update_f)(&mut self.sender)
    }
}
