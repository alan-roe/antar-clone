use crate::system::Colour;

pub trait Sender : Clone {
    fn name(&self) -> &str;
    fn name_mut(&mut self) -> &mut String;

    fn colour(&self) -> &Colour;
}

#[derive(PartialEq, Clone, Debug)]
pub struct PSender {
    name: String,
    description: String,
    colour: Colour
}

impl PSender {
    pub fn new(name: impl ToString, description: impl ToString, colour: Colour) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            colour,
        }
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn description_mut(&mut self) -> &mut String {
        &mut self.description
    }

    
    pub fn colour_mut(&mut self) -> &mut Colour {
        &mut self.colour
    }
}

impl Sender for PSender {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    fn colour(&self) -> &Colour {
        &self.colour
    }
}