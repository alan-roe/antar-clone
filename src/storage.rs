// https://dioxuslabs.com/learn/0.4/cookbook/state/custom_hooks
use gloo_storage::{LocalStorage, Storage};
use serde::{de::DeserializeOwned, Serialize};

use dioxus::prelude::*;

pub fn get_storage<T: DeserializeOwned>(key: &str, init: impl FnOnce() -> T) -> T {
    LocalStorage::get(key).ok().unwrap_or_else(init)
}

pub fn set_storage<T: Serialize>(key: &str, value: T) {
    LocalStorage::set(key, value);
}
