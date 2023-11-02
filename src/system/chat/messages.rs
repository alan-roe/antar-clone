use indexmap::IndexMap;
use core::fmt::Debug;
use super::{ChatId, Message};

pub trait Messages<ID: ChatId, M: Message> : IntoIterator<Item = (ID, M)> {
    fn new() -> Self;
    fn insert(&mut self, key: ID, value: M) -> Option<M>;
    fn remove(&mut self, id: &ID) -> bool;
    fn get_mut(&mut self, id: &ID) -> Option<&mut M>;
    fn last(&self) -> Option<(&ID, &M)>;
    fn iter(&self) -> indexmap::map::Iter<ID, M>;
}

#[derive(Clone, Debug)]
pub struct PMessages<ID: ChatId, M: Message> {
    messages: IndexMap<ID, M>,
}

impl<ID: ChatId, M: Message> IntoIterator for PMessages<ID, M> {
    type Item =(ID, M);

    type IntoIter = indexmap::map::IntoIter<ID, M>;

    fn into_iter(self) -> Self::IntoIter {
        self.messages.into_iter()
    }
}

impl<ID: ChatId + Debug + Copy, M: Message + Clone + Debug> Messages<ID, M> for PMessages<ID, M> {

    fn new() -> Self {
        PMessages { messages: Default::default() }
    }

    fn insert(&mut self, key: ID, value: M) -> Option<M> {
        self.messages.insert(key, value)
    }

    fn remove(&mut self, id: &ID) -> bool {
        self.messages.shift_remove(id).map_or(false, |_| true)
    }

    fn get_mut(&mut self, id: &ID) -> Option<&mut M> {
        self.messages.get_mut(id)
    }

    fn last(&self) -> Option<(&ID, &M)> {
        self.messages.last()
    }

    fn iter(&self) -> indexmap::map::Iter<ID, M> {
        self.messages.iter()
    }
}

