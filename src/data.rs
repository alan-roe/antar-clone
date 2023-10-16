use crate::colours::{Colour, Rgb};
use crate::storage::*;
use dioxus::prelude::*;
use dioxus_signals::Signal;
use indexmap::{indexmap, IndexMap, indexset, IndexSet};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Default)]
pub struct AppData {
    personas: Signal<Personas>,
    chats: Signal<Chats>,
}

impl AppData {
    fn use_app_context(cx: &ScopeState) -> AppData {
        *use_context(cx).expect("no app context provided, must be loaded first")
    }

    pub fn personas(cx: &ScopeState) -> Signal<Personas> {
        AppData::use_app_context(cx).personas
    }

    pub fn chats(cx: &ScopeState) -> Signal<Chats> {
        AppData::use_app_context(cx).chats
    }

    pub fn save_personas(cx: &ScopeState) {
        set_storage("ifs_personas", AppData::use_app_context(cx).personas)
    }

    pub fn save_chats(cx: &ScopeState) {
        set_storage("ifs_chats", AppData::use_app_context(cx).chats)
    }

    pub fn load(cx: Scope) {
        use_context_provider(cx, AppData::default);
        let app_context = Self::use_app_context(cx);
        app_context
            .personas
            .set(get_storage("ifs_personas", move || {
                Personas(indexmap![Uuid::new_v4() => Persona {
                    name: "Me".to_string(),
                    colour: Rgb(0x49, 0x55, 0x65),
                }])
            }));

        app_context
            .chats
            .set(get_storage("ifs_chats", move || {
                let active_persona = *app_context.personas.read().get_index(0).unwrap().0;
                Chats(indexmap!{Uuid::new_v4() => Chat {
                    messages: Default::default(),
                    active_persona: Signal::new_in_scope(active_persona, cx.scope_id()),
                    added_personas: Signal::new_in_scope(indexset!{active_persona}, cx.scope_id()),
                    current_message: Signal::new_in_scope(String::new(), cx.scope_id())
                }})}
            ))
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Chats(IndexMap<Uuid, Chat>);

impl Chats {
    pub fn get_index(&self, index: usize) -> Option<(&Uuid, &Chat)> {
        self.0.get_index(index)
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Props)]
pub struct Chat {
    pub messages: Signal<Messages>,
    pub active_persona: Signal<Uuid>,
    pub added_personas: Signal<IndexSet<Uuid>>,
    pub current_message: Signal<String>,
}

impl Chat {
    pub fn send(&self) {
        self.messages.write().msgs.push(Message {
            uuid: Uuid::new_v4(),
            msg: self.current_message.read().clone(),
            persona: *self.active_persona.read()
        });
        self.current_message.set(String::new())
    }
}

#[derive(Clone, Default, PartialEq, Props, Serialize, Deserialize)]
pub struct Persona {
    pub name: String,
    pub colour: Rgb,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Personas(pub IndexMap<Uuid, Persona>);

impl Personas {
    pub fn get(&self, key: &Uuid) -> Option<&Persona> {
        self.0.get(key)
    }

    pub fn get_index_of(&self, key: &Uuid) -> Option<usize> {
        self.0.get_index_of(key)
    }

    pub fn get_index(&self, index: usize) -> Option<(&Uuid, &Persona)> {
        self.0.get_index(index)
    }

    pub fn push(&mut self, value: Persona) {
        self.0.insert(Uuid::new_v4(), value);
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> indexmap::map::Iter<Uuid, Persona> {
        self.0.iter()
    }
}

#[derive(Clone, PartialEq, Props, Serialize, Deserialize)]
pub struct Message {
    pub uuid: Uuid,
    pub msg: String,
    pub persona: Uuid,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Messages {
    pub msgs: Vec<Message>,
}
