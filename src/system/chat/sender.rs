use crate::system::Colour;

pub trait Sender : Clone {
    fn name(&self) -> &str;
    fn update_name<F: FnMut(&mut String)>(&mut self, f: F);

    fn colour(&self) -> &Colour;
    fn update_colour<F: FnMut(&mut Colour)>(&mut self, f: F);

    fn description(&self) -> &str;
    fn update_description<F: FnMut(&mut String)>(&mut self, f: F);
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
}

impl Sender for PSender {
    fn name(&self) -> &str {
        &self.name
    }

    fn update_name<F: FnMut(&mut String)>(&mut self, mut f: F) {
        (f)(&mut self.name)
    }
    
    fn colour(& self) -> & Colour {
        &self.colour
    }

    fn update_colour<F: FnMut(&mut Colour)>(&mut self, mut f: F) {
        (f)(&mut self.colour)
    }
    
    fn description(&self) -> &str {
        &self.description
    }

    fn update_description<F: FnMut(&mut String)>(&mut self, mut f: F) {
        (f)(&mut self.description)
    }
}