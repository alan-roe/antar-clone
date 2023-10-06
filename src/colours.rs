use std::{fmt::Display, str::FromStr};

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Rgb(pub u8, pub u8, pub u8);

#[derive(Debug, PartialEq, Eq)]
pub struct ParseRgbError;

impl FromStr for Rgb {
    type Err = ParseRgbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix('#').ok_or(ParseRgbError)?;
        let (r, g, b) = (
            s.get(0..2).ok_or(ParseRgbError)?,
            s.get(2..4).ok_or(ParseRgbError)?,
            s.get(4..6).ok_or(ParseRgbError)?,
        );
        let (r, g, b) = (
            u8::from_str_radix(r, 16).map_err(|_| ParseRgbError)?,
            u8::from_str_radix(g, 16).map_err(|_| ParseRgbError)?,
            u8::from_str_radix(b, 16).map_err(|_| ParseRgbError)?,
        );
        Ok(Rgb(r, g, b))
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Colour {
    Colour(Rgb),
    BgColour(Rgb),
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut c_type: String;
        match self {
            Colour::Colour(Rgb(r, g, b)) => {
                c_type = format!("color: rgb({r}, {g}, {b});");
            }
            Colour::BgColour(Rgb(r, g, b)) => {
                c_type = format!("background-color: rgb({r}, {g}, {b});");
            }
        }
        write!(f, "{}", c_type)
    }
}
