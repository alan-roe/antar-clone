use crate::colours::{Colour, Rgb};
use dioxus::prelude::*;
use dioxus_signals::{use_signal, Signal};
use dioxus_std::storage::*;
use indexmap::{indexmap, indexset, IndexMap, IndexSet};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod chats;
pub mod personas;

pub use chats::*;
pub use personas::*;

#[derive(Clone, Copy, Default)]
pub struct AppState {
    personas: Signal<Personas>,
    chats: Signal<Chats>,
}

/// Ties together the different types of state
impl AppState {
    fn use_app_context(cx: &ScopeState) -> Self {
        *use_context(cx).expect("no app context provided, must be loaded first")
    }

    pub fn personas(cx: &ScopeState) -> Signal<Personas> {
        AppState::use_app_context(cx).personas
    }

    pub fn chats(cx: &ScopeState) -> Signal<Chats> {
        AppState::use_app_context(cx).chats
    }

    pub fn active_chat(cx: &ScopeState) -> Option<Chat> {
        AppState::chats(cx).read().active_chat()
    }

    pub fn set_active_chat(cx: &ScopeState, uuid: Uuid) {
        AppState::chats(cx).write().set_active_chat(uuid);
        use_eval(cx)(r#"document.getElementById("messageInput").focus();"#).unwrap();
    }

    pub fn delete_active_chat(cx: &ScopeState) {
        AppState::chats(cx).write().delete_active();
    }

    pub fn load(cx: &ScopeState) {
        let personas =
            use_synced_storage::<LocalStorage, Personas>(cx, "ifs_personas".to_string(), || {
                Personas::new(Persona {
                    name: "Me".to_string(),
                    colour: Rgb(0x49, 0x55, 0x65),
                })
            });
        
        let chats =
            use_synced_storage::<LocalStorage, Chats>(cx, "ifs_chats".to_string(), move || {
                let p_uuid = *personas.read().get_index(0).unwrap().0;
                let chat = Chat::new(p_uuid);
                Chats::new(chat)
            });
        let loaded = use_signal(cx, || false);
        if !*loaded.read() {
            let p_uuid = *personas.read().get_index(0).unwrap().0;
            let chat = Chat::new(p_uuid);
            chats.write().load_chats(chat);
            loaded.set(true);
        }

        let app_state = AppState { personas, chats };
        use_context_provider(cx, || app_state);
    }
}
