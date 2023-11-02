mod message;
mod sender;
mod chatid;

pub use message::*;
pub use sender::*;
pub use chatid::*;

use core::fmt::Debug;
use indexmap::IndexMap;

pub trait Chat<ID: ChatId, M: Message> {
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

    fn iter(&self) -> indexmap::map::Iter<ID, M>;
}

pub struct PChat<ID: ChatId, M: Message> {
    messages: IndexMap<ID, M>,
}

impl<ID: ChatId + Debug + Copy, M: Message + Clone + Debug> Chat<ID, M> for PChat<ID, M> {
    fn new() -> Self {
        Self {
            messages: Default::default(),
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
        self.messages.shift_remove(id).map_or(false, |_| true)
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

    fn iter(&self) -> indexmap::map::Iter<ID, M> {
        self.messages.iter()
    }
}
