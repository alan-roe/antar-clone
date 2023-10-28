use dioxus_signals::Signal;
use indexmap::{indexmap, indexset, IndexMap, IndexSet};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::hash::{Hash, Hasher};

use crate::storage;

#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Chats {
    chat_ids: IndexSet<Uuid>,
    #[serde(skip)]
    chats: IndexSet<Chat>,
    active_chat: Option<Uuid>,
    save_toggle: bool,
}

impl Chats {
    pub fn new(chat: Chat) -> Self {
        Chats {
            chat_ids: Default::default(),
            chats: Default::default(),
            active_chat: None,
            save_toggle: false,
        }
    }

    pub fn load_chats(&mut self, default_chat: Chat) {
        self.chat_ids.iter().for_each(|chat_id| {
            self.chats.insert(
                Chat::load_or(chat_id, default_chat),
            );
        });
    }

    pub fn new_chat(&mut self, chat: Chat) {
        let chat_id = chat.uuid;
        self.chats.insert(chat);
        self.chat_ids.insert(chat_id);
        self.active_chat = Some(chat_id);
    }

    pub fn save_active(&self) {
        if let Some(active_chat) = &self.active_chat {
            self.chats.get(active_chat).unwrap().save();
        }
    }

    pub fn get_index(&self, index: usize) -> Option<&Chat> {
        self.chats.get_index(index)
    }

    pub fn send_message(&mut self) {
        if let Some(active_chat) = &self.active_chat {
            self.chats.get(active_chat).unwrap().send();
            self.save_active()
        }
    }

    pub fn chats(&self) -> indexmap::set::Iter<Chat> {
        self.chats.iter()
    }

    pub fn active_chat_uuid(&self) -> &Option<Uuid> {
        &self.active_chat
    }

    pub fn active_chat(&self) -> Option<&Chat> {
        self.active_chat_uuid()
            .map(|active_chat| self.chats.get(&active_chat)).flatten()
    }

    pub fn set_active_chat(&mut self, uuid: Uuid) {
        self.active_chat = Some(uuid);
    }

    pub fn delete_active(&mut self) {
        if let Some(active_chat) = &self.active_chat {
            self.chats.shift_remove(active_chat);
            self.chat_ids.shift_remove(active_chat);
            self.active_chat = None;
        }
    }
}

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct Chat {
    uuid: Uuid,
    pub name: Signal<String>,
    pub messages: Signal<Messages>,
    pub active_persona: Signal<Uuid>,
    pub added_personas: Signal<IndexSet<Uuid>>,
    pub current_message: Signal<String>,
}

impl Chat {
    /// Creates a new chat with the specified Persona as starter
    pub fn new(persona_id: Uuid) -> Self {
        Chat {
            uuid: Uuid::new_v4(),
            name: Signal::new(format!("{}", chrono::Utc::now().format("%a, %h %d, %Y"))),
            active_persona: Signal::new(persona_id),
            added_personas: Signal::new(indexset! { persona_id }),
            ..Default::default()
        }
    }

    fn load_or(uuid: &Uuid, default_chat: Chat) -> Self {
        storage::retrieve(format!("ifs_chat_{}", uuid), || {
            default_chat
        })
    }

    pub fn save(&self) {
        storage::store(format!("ifs_chat_{}", &self.uuid), self.clone());
    }

    pub fn send(&self) {
        self.messages.write().msgs.push(Message {
            uuid: Uuid::new_v4(),
            msg: self.current_message.read().clone(),
            persona: *self.active_persona.read(),
        });
        self.current_message.set(String::new())
    }

    pub fn add_persona(&self, uuid: Uuid) -> bool {
        self.active_persona.set(uuid);
        self.added_personas.write().insert(uuid)
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}

impl Eq for Chat { }

impl PartialEq for Chat {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Hash for Chat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl indexmap::Equivalent<Chat> for Uuid {
    fn equivalent(&self, key: &Chat) -> bool {
        self == &key.uuid
    }
}

unsafe impl Send for Chat {}
unsafe impl Sync for Chat {}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub uuid: Uuid,
    pub msg: String,
    pub persona: Uuid,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Messages {
    pub msgs: Vec<Message>,
}
