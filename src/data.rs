use crate::colours::Colour;
use dioxus::prelude::*;
use uuid::Uuid;

#[derive(Clone, PartialEq, Props)]
pub struct Persona {
    pub uuid: Uuid,
    pub name: String,
    pub colour: (u8, u8, u8),
}

pub type Personas = Vec<Persona>;

#[derive(Clone, PartialEq, Props)]
pub struct Message {
    pub uuid: Uuid,
    pub msg: String,
    pub persona: Persona,
}
#[derive(Default)]
pub struct Messages {
    pub msgs: Vec<Message>,
}
