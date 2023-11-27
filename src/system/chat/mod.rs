mod message;
mod messages;
mod sender;
mod chatid;

use leptos::{SignalWith, SignalUpdate, RwSignal};
pub use message::*;
pub use messages::*;
pub use sender::*;
pub use chatid::*;

use core::fmt::Debug;
use std::marker::PhantomData;

pub trait Chat<ID: ChatId, M: Message, MS: Messages<ID, M>> {
    fn new() -> Self;

    fn name(&self) -> &str;

    fn name_mut(&mut self) -> &mut String;

    /// Returns the last message in the chat
    fn last_message(&self) -> Option<M>;

    /// Return ID of sent message
    fn send_message(&mut self, message: M) -> ID;

    /// Return true if message was deleted
    fn delete_message(&mut self, id: &ID) -> bool;

    /// Updates the message content indexed by `id` using the `update_f` closure
    fn update_content<F: FnOnce(&mut Content)>(&mut self, id: &ID, update_f: F);

    /// Updates the message sender indexed by `id` using the `update_f` closure
    fn update_sender<F: FnOnce(&mut M::S)>(&mut self, id: &ID, update_f: F);

    fn messages(&self) -> RwSignal<MS>;
}

pub struct PChat<ID: ChatId + 'static, M: Message + 'static, MS: Messages<ID, M> + 'static> {
    name: String,
    messages: RwSignal<MS>,
    senders: Vec<RwSignal<M::S>>,
    phantom_data: PhantomData<(M, ID)>
}

impl<ID: ChatId + Debug + Copy, M: Message + Clone + Debug, MS: Messages<ID, M> + Clone + Debug> Chat<ID, M, MS> for PChat<ID, M, MS> {
    fn new() -> Self {
        Self {
            name: Default::default(),
            messages: RwSignal::new(Messages::new()),
            senders: Vec::new(),
            phantom_data: PhantomData
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
    
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    fn last_message(&self) -> Option<M> {
        self.messages.with(|messages| {
            messages.last().map(|message| message.1.clone() )
        })
    }

    fn send_message(&mut self, message: M) -> ID {
        let id = ID::new();
        self.messages.update(|messages| {
            if let Some(prev_message) = messages.insert(id, message.clone()) {
                log::error!("message with id \n{id:?}\n already exists, overwriting original message \n{prev_message:?}\n with \n{message:?}\nTHIS SHOULD NEVER HAPPEN");
            }
        });
        id
    }

    fn delete_message(&mut self, id: &ID) -> bool {
        let mut r = false;
        self.messages.update(|messages| r = messages.remove(id));
        r
    }

    fn update_content<F: FnOnce(&mut Content)>(&mut self, id: &ID, update_f: F) {
        self.messages
            .update(|messages|
                messages
                .get_mut(id)
                .expect("can't update message with id {id}")
                .update_content(update_f)
            )
    }

    fn update_sender<F: FnOnce(&mut M::S)>(&mut self, id: &ID, update_f: F) {
        self.messages
            .update(|messages|
                messages
                .get_mut(id)
                .expect("can't update message with id {id}")
                .update_sender(update_f)
            )
    }

    fn messages(&self) -> RwSignal<MS> {
        self.messages
    }
}
