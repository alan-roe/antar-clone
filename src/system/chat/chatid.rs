use uuid::Uuid;
use std::hash::Hash;

pub trait ChatId : PartialEq + Eq + Hash {
    fn new() -> Self;
}

impl ChatId for Uuid {
    fn new() -> Self {
        Uuid::new_v4()
    }
}
