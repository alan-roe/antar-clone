mod message;
mod messages;
mod sender;
mod chatid;

pub use message::*;
pub use messages::*;
pub use sender::*;
pub use chatid::*;

use core::fmt::Debug;
use std::marker::PhantomData;

pub trait Chat<ID: ChatId, M: Message, MS: Messages<ID, M>> {
    fn new() -> Self;

    /// Returns the last message in the chat
    fn last_message(&self) -> Option<&M>;

    /// Return ID of sent message
    fn send_message(&mut self, message: M) -> ID;

    /// Return true if message was deleted
    fn delete_message(&mut self, id: &ID) -> bool;

    /// Updates the message content indexed by `id` using the `update_f` closure
    fn update_content<F: FnOnce(&mut Content)>(&mut self, id: &ID, update_f: F);

    /// Updates the message sender indexed by `id` using the `update_f` closure
    fn update_sender<F: FnOnce(&mut M::S)>(&mut self, id: &ID, update_f: F);

    fn messages(&self) -> &MS;
}

pub struct PChat<ID: ChatId, M: Message, MS: Messages<ID, M>> {
    messages: MS,
    phantom_data: PhantomData<(M, ID)>
}

impl<ID: ChatId + Debug + Copy, M: Message + Clone + Debug, MS: Messages<ID, M> + Clone + Debug> Chat<ID, M, MS> for PChat<ID, M, MS> {
    fn new() -> Self {
        Self {
            messages: Messages::new(),
            phantom_data: PhantomData
        }
    }

    fn last_message(&self) -> Option<&M> {
        if let Some(message) = self.messages.last() {
            Some(message.1)
        } else {
            None
        }
    }

    fn send_message(&mut self, message: M) -> ID {
        let id = ID::new();
        if let Some(prev_message) = self.messages.insert(id, message.clone()) {
            log::error!("message with id \n{id:?}\n already exists, overwriting original message \n{prev_message:?}\n with \n{message:?}\nTHIS SHOULD NEVER HAPPEN");
        }
        id
    }

    fn delete_message(&mut self, id: &ID) -> bool {
        self.messages.remove(id)
    }

    fn update_content<F: FnOnce(&mut Content)>(&mut self, id: &ID, update_f: F) {
        self.messages
            .get_mut(id)
            .expect("can't update message with id {id}")
            .update_content(update_f);
    }

    fn update_sender<F: FnOnce(&mut M::S)>(&mut self, id: &ID, update_f: F) {
        self.messages
            .get_mut(id)
            .expect("can't update message with id {id}")
            .update_sender(update_f);
    }

    fn messages(&self) -> &MS {
        &self.messages
    }
}
