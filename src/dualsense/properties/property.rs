use super::{offset::Offset, valuetype::ValueType};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub(crate) enum Property {
    LeftPadX,
    LeftPadY,
    RightPadX,
    RightPadY,

    L1,
    R1,
    L2Value,
    R2Value,
    L3,
    R3,

    Options,
    Share,

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

            Property::L1 => Offset::new(9..10, 0..1),
            Property::R1 => Offset::new(9..10, 1..2),
            Property::Share => Offset::new(9..10, 4..5),
            Property::Options => Offset::new(9..10, 5..6),
            Property::L3 => Offset::new(9..10, 6..7),
            Property::R3 => Offset::new(9..10, 7..8),

            Property::L2Value => Offset::new(5..6, 0..8),
            Property::R2Value => Offset::new(6..7, 0..8),
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
            | Property::L2Value
            | Property::R2Value => ValueType::U8(*data.first().unwrap()),

            Property::DPad => ValueType::Pad((*data.first().unwrap()).into()),
            Property::Symbols => ValueType::Symbol((*data.first().unwrap()).into()),

            Property::L1
            | Property::R1
            | Property::L3
            | Property::R3
            | Property::Options
            | Property::Share => ValueType::Bool(*data.first().unwrap() == 1),
        }
    }
}
