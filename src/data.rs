use crate::colours::{Colour, Rgb};
use crate::storage::*;
use dioxus::prelude::*;
use dioxus_signals::Signal;
use indexmap::{indexmap, IndexMap};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Default)]
pub struct AppData {
    personas: Signal<Personas>,
    chats: Signal<Chats>,
}

impl AppData {
    pub fn use_app_context(cx: Scope) -> AppData {
        *use_context(cx).expect("no app context provided, must be loaded first")
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
            .set(get_storage("ifs_chats", move || Chats(Vec::default())))
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Chats(Vec<Chat>);

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Chat {
    messages: Messages,
    active_persona: Option<Uuid>,
    added_personas: Vec<Uuid>,
    current_message: Message,
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

#[derive(Clone, Default, PartialEq, Props, Serialize, Deserialize)]
pub struct Message {
    pub uuid: Uuid,
    pub msg: String,
    pub persona: Uuid,
}
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Messages {
    pub msgs: Vec<Message>,
}
