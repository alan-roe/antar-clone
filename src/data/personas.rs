use crate::colours::Rgb;
use indexmap::{indexmap, IndexMap};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Persona {
    pub name: String,
    pub colour: Rgb,
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Personas(pub IndexMap<Uuid, Persona>);

impl Personas {
    pub fn new(persona: Persona) -> Self {
        Personas(indexmap! { Uuid::new_v4() => persona })
    }
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
