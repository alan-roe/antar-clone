use dioxus::prelude::*;
use uuid::Uuid;
use crate::colours::Colour;

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
pub struct Messages {
    pub msgs: Vec<Message>,
}