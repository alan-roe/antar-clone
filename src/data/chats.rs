use serde::{Deserialize, Serialize};
use indexmap::{indexmap, IndexMap, IndexSet, indexset};
use uuid::Uuid;
use dioxus_signals::Signal;

use dioxus_std::storage::*;

#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Chats{
    chat_ids: IndexSet<Uuid>,
    #[serde(skip)]
    chats: IndexMap<Uuid, Chat>,
    active_chat: Uuid,
    save_toggle: bool
}

impl Chats {
    pub fn new(chat: Chat) -> Self {
        let uuid = Uuid::new_v4();

        // set_storage::<LocalStorage, Chat>(format!("ifs_chat_{}", uuid), chat);
        Chats {
            chat_ids: indexset! { uuid },
            chats: indexmap! { uuid => chat },
            active_chat: uuid,
            save_toggle: false,
        }
    }

    pub fn load_chats(&mut self, default_chat: Chat) {
        self.chat_ids.iter().for_each(|chat_id| {
            self.chats.insert(*chat_id, get_from_storage::<LocalStorage, Chat>(format!("ifs_chat_{}", &chat_id), || default_chat));
        });
    }

    pub fn save_active(&self) {
        // set_storage::<LocalStorage, Chat>(format!("ifs_chat_{}", &self.active_chat), *self.chats.get(&self.active_chat).unwrap());
    } 

    pub fn get_index(&self, index: usize) -> Option<(&Uuid, &Chat)> {
        self.chats.get_index(index)
    }

    pub fn send_message(&mut self) {
        self.chats.get(&self.active_chat).unwrap().send();
        self.save_active()
    }
}

#[derive(Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct Chat {
    pub messages: Signal<Messages>,
    pub active_persona: Signal<Uuid>,
    pub added_personas: Signal<IndexSet<Uuid>>,
    pub current_message: Signal<String>,
}

impl Chat {
    /// Creates a new chat with the specified Persona as starter
    pub fn new(persona_id: Uuid) -> Self {
        Chat {
            active_persona: Signal::new(persona_id),
            added_personas: Signal::new(indexset! { persona_id }),
            ..Default::default()
        }
    }
    fn send(&self) {
        self.messages.write().msgs.push(Message {
            uuid: Uuid::new_v4(),
            msg: self.current_message.read().clone(),
            persona: *self.active_persona.read(),
        });
        self.current_message.set(String::new())
    }
}

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
