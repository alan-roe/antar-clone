use crate::colours::{Colour, Rgb};
use dioxus::prelude::*;
use dioxus_signals::{use_signal, Signal};
use indexmap::{indexmap, indexset, IndexMap, IndexSet};
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

    pub fn load(cx: Scope) {
        let personas = dioxus_std::storage::use_synced_storage::<
            dioxus_std::storage::LocalStorage,
            Personas,
        >(cx, "ifs_personas".to_string(), || {
            Personas(indexmap![Uuid::new_v4() => Persona {
                name: "Me".to_string(),
                colour: Rgb(0x49, 0x55, 0x65),
            }])
        });
        let app_data = AppData {
            personas,
            chats: dioxus_std::storage::use_synced_storage::<
                dioxus_std::storage::LocalStorage,
                Chats,
            >(cx, "ifs_chats".to_string(), move || {
                let c_uuid = Uuid::new_v4();
                let p_uuid = *personas.read().get_index(0).unwrap().0;
                Chats{
                    chats: indexmap! {c_uuid => Chat {
                        messages: Default::default(),
                        active_persona: Signal::new_in_scope(p_uuid, cx.scope_id()),
                        added_personas: Signal::new_in_scope(indexset!{p_uuid}, cx.scope_id()),
                        current_message: Signal::new_in_scope(String::new(), cx.scope_id()),
                    }},
                    active_chat: c_uuid,
                    save_toggle: true,
                }
            }),
        };
        use_context_provider(cx, || app_data);
    }
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Chats{
    chats: IndexMap<Uuid, Chat>,
    active_chat: Uuid,
    save_toggle: bool
}

impl Chats {
    pub fn get_index(&self, index: usize) -> Option<(&Uuid, &Chat)> {
        self.chats.get_index(index)
    }

    pub fn send_message(&mut self) {
        self.chats.get(&self.active_chat).unwrap().send();
        self.save_toggle = !self.save_toggle;
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
            persona: *self.active_persona.read(),
        });
        self.current_message.set(String::new())
    }
}

#[derive(Clone, Default, PartialEq, Props, Serialize, Deserialize)]
pub struct Persona {
    pub name: String,
    pub colour: Rgb,
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
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
