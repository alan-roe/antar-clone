// https://dioxuslabs.com/learn/0.4/cookbook/state/custom_hooks
use gloo_storage::{LocalStorage, Storage};
use serde::{de::DeserializeOwned, Serialize};

use dioxus::prelude::*;

pub fn get_storage<T: DeserializeOwned>(key: impl ToString, init: impl FnOnce() -> T) -> T {
    LocalStorage::get(key.to_string()).ok().unwrap_or_else(init)
}

pub fn set_storage<T: Serialize>(key: impl ToString, value: T) {
    LocalStorage::set(key.to_string(), value);
}
