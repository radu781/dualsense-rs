#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DPad {
    Up = 0,
    UpRight = 1,
    Right = 2,
    DownRight = 3,
    Down = 4,
    DownLeft = 5,
    Left = 6,
    UpLeft = 7,
    None = 8,
}

impl From<u8> for DPad {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Up,
            1 => Self::UpRight,
            2 => Self::Right,
            3 => Self::DownRight,
            4 => Self::Down,
            5 => Self::DownLeft,
            6 => Self::Left,
            7 => Self::UpLeft,
            _ => Self::None,
        }
    }
}
