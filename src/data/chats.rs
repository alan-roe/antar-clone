use serde::{Deserialize, Serialize};
use indexmap::{indexmap, IndexMap, IndexSet, indexset};
use uuid::Uuid;
use dioxus_signals::Signal;

#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Chats{
    chats: IndexMap<Uuid, Chat>,
    active_chat: Uuid,
    save_toggle: bool
}

impl Chats {
    pub fn new(chat: Chat) -> Self {
        let uuid = Uuid::new_v4();
        Chats {
            chats: indexmap! { uuid => chat },
            active_chat: uuid,
            save_toggle: false,
        }
    }

    pub fn get_index(&self, index: usize) -> Option<(&Uuid, &Chat)> {
        self.chats.get_index(index)
    }

    pub fn send_message(&mut self) {
        self.chats.get(&self.active_chat).unwrap().send();
        self.save_toggle = !self.save_toggle;
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
    pub fn send(&self) {
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
