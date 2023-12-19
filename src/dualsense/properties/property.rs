use super::{offset::Offset, valuetype::ValueType};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub(crate) enum Property {
    LeftPadX,
    LeftPadY,
    RightPadX,
    RightPadY,

    L2,
    R2,

    DPad,
    Symbols,
}

impl Property {
    pub(crate) fn offset(&self) -> Offset {
        match self {
            Property::LeftPadX => Offset::new(1..2, 0..8),
            Property::LeftPadY => Offset::new(2..3, 0..8),
            Property::RightPadX => Offset::new(3..4, 0..8),
            Property::RightPadY => Offset::new(4..5, 0..8),
            Property::L2 => Offset::new(5..6, 0..8),
            Property::R2 => Offset::new(6..7, 0..8),
            Property::DPad => Offset::new(8..9, 0..4),
            Property::Symbols => Offset::new(8..9, 4..8),
        }
    }

    pub(crate) fn convert(&self, data: &[u8]) -> ValueType {
        match self {
            Property::LeftPadX
            | Property::LeftPadY
            | Property::RightPadX
            | Property::RightPadY
            | Property::L2
            | Property::R2 => ValueType::U8(*data.first().unwrap()),
            Property::DPad => ValueType::Pad((*data.first().unwrap()).into()),
            Property::Symbols => ValueType::Symbol((*data.first().unwrap()).into()),
        }
    }
}
