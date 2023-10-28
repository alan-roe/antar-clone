use dioxus::prelude::*;
use dioxus_signals::Signal;
use dioxus_std::storage::*;
use serde::{Serialize, de::DeserializeOwned};

pub fn store<T: Serialize + Send + Sync + Clone + 'static>(key: impl ToString, value: T) -> bool {
    LocalStorage::set(
        key.to_string(),
        &value,
    );
    true
}

pub fn retrieve<T: Serialize + DeserializeOwned + Send + Sync + Clone + 'static>(key: impl ToString, init: impl FnOnce() -> T) -> T {
    get_from_storage::<LocalStorage, T>(key.to_string(), init)
    // init()
}

pub fn use_synced_storage<T: Serialize + DeserializeOwned + Clone + Send + Sync + PartialEq + 'static>(cx: &ScopeState, key: impl ToString, init: impl FnOnce() -> T) -> Signal<T> {
    dioxus_std::storage::use_synced_storage::<LocalStorage, T>(cx, key.to_string(), init)
    // dioxus_signals::use_signal(cx, init)
}
