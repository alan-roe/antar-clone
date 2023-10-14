use crate::colours::{Colour, Rgb};
use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Clone, PartialEq, Props, Serialize, Deserialize)]
pub struct Persona {
    pub uuid: Uuid,
    pub name: String,
    pub colour: Rgb,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Personas(pub Vec<Persona>);

impl Personas {
    pub fn get(&self, index: usize) -> Option<&Persona> {
        self.0.get(index)
    }

    pub fn push(&mut self, value: Persona) {
        self.0.push(value)
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> core::slice::Iter<Persona> {
        self.0.iter()
    }
}

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
