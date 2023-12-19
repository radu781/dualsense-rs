#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Symbols {
    Square = 1,
    Cross = 2,
    Circle = 4,
    Triangle = 8,
    None = 0,
}

impl From<u8> for Symbols {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Square,
            2 => Self::Cross,
            4 => Self::Circle,
            8 => Self::Triangle,
            _ => Self::None,
        }
    }
}
