use super::{offset::Offset, valuetype::ValueType};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub(crate) enum Property {
    LeftPadX,
    LeftPadY,
    RightPadX,
    RightPadY,

    L1,
    R1,
    L2,
    R2,
    L3,
    R3,

    Options,
    Share,
    // TODO: broken
    Mute,
    // TODO: broken
    TouchPad,
    // TODO: broken
    PlayStation,

    DPad,
    Symbols,

    GyroscopeX,
    GyroscopeY,
    GyroscopeZ,

    AccelerationX,
    AccelerationY,
    AccelerationZ,

    TouchPadFinger1Active,
    TouchPad1Id,
    TouchPad1X,
    TouchPad1Y,
    TouchPadFinger2Active,
    TouchPad2Id,
    TouchPad2X,
    TouchPad2Y,
}

impl Property {
    pub(crate) fn offset(&self) -> Offset {
        match self {
            Property::LeftPadX => Offset::bytes(1..2),
            Property::LeftPadY => Offset::bytes(2..3),
            Property::RightPadX => Offset::bytes(3..4),
            Property::RightPadY => Offset::bytes(4..5),

            Property::L2 => Offset::bytes(5..6),
            Property::R2 => Offset::bytes(6..7),

            Property::L1 => Offset::bit(9, 0),
            Property::R1 => Offset::bit(9, 1),
            Property::Share => Offset::bit(9, 4),
            Property::Options => Offset::bit(9, 5),
            Property::L3 => Offset::bit(9, 6),
            Property::R3 => Offset::bit(9, 7),

            Property::DPad => Offset::bits(8, 0..4),
            Property::Symbols => Offset::bits(8, 4..8),
            Property::Mute => Offset::bit(10, 5),
            Property::TouchPad => Offset::bit(10, 6),
            Property::PlayStation => Offset::bit(10, 7),

            Property::GyroscopeX => Offset::bytes(16..18),
            Property::GyroscopeY => Offset::bytes(18..20),
            Property::GyroscopeZ => Offset::bytes(20..22),
            Property::AccelerationX => Offset::bytes(22..24),
            Property::AccelerationY => Offset::bytes(24..26),
            Property::AccelerationZ => Offset::bytes(26..28),

            Property::TouchPadFinger1Active => Offset::bytes(33..34),
            Property::TouchPad1Id => Offset::bytes(33..34),
            Property::TouchPad1X => Offset::bytes(34..36),
            Property::TouchPad1Y => Offset::bytes(35..37),
            Property::TouchPadFinger2Active => Offset::bytes(37..38),
            Property::TouchPad2Id => Offset::bytes(37..38),
            Property::TouchPad2X => Offset::bytes(38..40),
            Property::TouchPad2Y => Offset::bytes(39..41),
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

            Property::L1
            | Property::R1
            | Property::L3
            | Property::R3
            | Property::Options
            | Property::Share
            | Property::Mute
            | Property::TouchPad
            | Property::PlayStation => ValueType::Bool(*data.first().unwrap() == 1),

            // TODO: normalize values
            Property::GyroscopeX
            | Property::GyroscopeY
            | Property::GyroscopeZ
            | Property::AccelerationX
            | Property::AccelerationY
            | Property::AccelerationZ => ValueType::U16((data[1] as u16) << 8 | data[0] as u16),

            Property::TouchPadFinger1Active => ValueType::Bool(data[0] & 0x80 == 1),
            Property::TouchPad1Id => ValueType::U8(data[0] & 0x7F),
            Property::TouchPad1X => ValueType::U16(((data[1] as u16 & 0x0F) << 8) | data[0] as u16),
            Property::TouchPad1Y => {
                ValueType::U16(((data[1] as u16) << 4) | (data[0] as u16 & 0xF0) >> 4)
            }
            Property::TouchPadFinger2Active => ValueType::Bool(data[0] & 0x80 == 1),
            Property::TouchPad2Id => ValueType::U8(data[0] & 0x7F),
            Property::TouchPad2X => ValueType::U16((data[1] as u16 & 0x0F) << 8 | data[0] as u16),
            Property::TouchPad2Y => {
                ValueType::U16(((data[1] as u16) << 4) | (data[0] as u16 & 0xF0) >> 4)
            }
        }
    }
}
