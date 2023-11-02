use crate::system::Colour;

pub fn text_colour_from_bg(Colour::Rgb(r, g, b): Colour) -> Colour {
    if (u16::from(r) + u16::from(g) + u16::from(b)) >= (255 * 3 / 2) {
        Colour::Rgb(0, 0, 0)
    } else {
        Colour::Rgb(255, 255, 255)
    }
}

pub trait ToStyle {
    fn to_color(&self) -> String;
    fn to_bg_color(&self) -> String;
}

impl ToStyle for Colour {
    fn to_color(&self) -> String {
        match self {
            Colour::Rgb(r, g, b) => {
                format!("color: rgb({r}, {g}, {b});")
            },
        }
    }

    fn to_bg_color(&self) -> String {
        match self {
            Colour::Rgb(r, g, b) => {
                format!("background-color: rgb({r}, {g}, {b});")
            },
        }
    }
}