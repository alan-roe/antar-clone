#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Colour {
    Rgb(u8, u8, u8),
}

impl Colour {
    pub const BLACK: Colour = Colour::Rgb(0, 0, 0);
}