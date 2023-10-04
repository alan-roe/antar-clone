use std::fmt::Display;

pub type Rgb = (u8, u8, u8);

#[derive(Clone, Copy, PartialEq)]
pub enum Colour{
  Colour(Rgb),
  BgColour(Rgb),
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let mut c_type: String;
      match self {
        Colour::Colour((r, g, b)) => {
          c_type = format!("color: rgb({r}, {g}, {b});");
        },
        Colour::BgColour((r, g, b)) => {
          c_type = format!("background-color: rgb({r}, {g}, {b});");
        },
      }
      write!(f, "{}", c_type)
    }
}