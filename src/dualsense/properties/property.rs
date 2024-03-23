use std::hash::Hash;

use super::{
    analog_pad::AnalogPad, dpad::DPad, offset::Offset, symbols::Symbols, trigger::Trigger,
    valuetype::ValueType,
};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub(crate) enum InputProperty {
    L1,
    R1,
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

    R2FeedbackOn,
    L2FeedbackOn,
    R2FeedbackValue,
    L2FeedbackValue,
}

impl InputProperty {
    pub(crate) fn offset(&self) -> Offset {
        match self {
            InputProperty::L1 => Offset::bit(9, 0),
            InputProperty::R1 => Offset::bit(9, 1),
            InputProperty::Share => Offset::bit(9, 4),
            InputProperty::Options => Offset::bit(9, 5),
            InputProperty::L3 => Offset::bit(9, 6),
            InputProperty::R3 => Offset::bit(9, 7),

            InputProperty::PlayStation => Offset::bit(10, 0),
            InputProperty::TouchPad => Offset::bit(10, 1),
            InputProperty::Mute => Offset::bit(10, 2),

            InputProperty::GyroscopeX => Offset::bytes(16..18),
            InputProperty::GyroscopeY => Offset::bytes(18..20),
            InputProperty::GyroscopeZ => Offset::bytes(20..22),
            InputProperty::AccelerationX => Offset::bytes(22..24),
            InputProperty::AccelerationY => Offset::bytes(24..26),
            InputProperty::AccelerationZ => Offset::bytes(26..28),

            InputProperty::TouchPadFinger1Active => Offset::bytes(33..34),
            InputProperty::TouchPad1Id => Offset::bytes(33..34),
            InputProperty::TouchPad1X => Offset::bytes(34..36),
            InputProperty::TouchPad1Y => Offset::bytes(35..37),
            InputProperty::TouchPadFinger2Active => Offset::bytes(37..38),
            InputProperty::TouchPad2Id => Offset::bytes(37..38),
            InputProperty::TouchPad2X => Offset::bytes(38..40),
            InputProperty::TouchPad2Y => Offset::bytes(39..41),

            InputProperty::R2FeedbackOn => Offset::bytes(42..43),
            InputProperty::L2FeedbackOn => Offset::bytes(43..44),
            InputProperty::R2FeedbackValue => Offset::bytes(42..43),
            InputProperty::L2FeedbackValue => Offset::bytes(43..44),
        }
    }

    pub(crate) fn convert(&self, data: &[u8]) -> ValueType {
        match self {
            InputProperty::L1
            | InputProperty::R1
            | InputProperty::L3
            | InputProperty::R3
            | InputProperty::Options
            | InputProperty::Share
            | InputProperty::Mute
            | InputProperty::TouchPad
            | InputProperty::PlayStation => ValueType::Bool(*data.first().unwrap() == 1),

            // TODO: normalize values
            InputProperty::GyroscopeX
            | InputProperty::GyroscopeY
            | InputProperty::GyroscopeZ
            | InputProperty::AccelerationX
            | InputProperty::AccelerationY
            | InputProperty::AccelerationZ => ValueType::I16(gyro_accel_into_u16(data)),

            InputProperty::TouchPadFinger1Active => ValueType::Bool(data[0] & 0x80 == 0),
            InputProperty::TouchPad1Id => ValueType::U8(data[0] & 0x7F),
            InputProperty::TouchPad1X => {
                ValueType::U16(((data[1] as u16 & 0x0F) << 8) | data[0] as u16)
            }
            InputProperty::TouchPad1Y => {
                ValueType::U16(((data[1] as u16) << 4) | (data[0] as u16 & 0xF0) >> 4)
            }
            InputProperty::TouchPadFinger2Active => ValueType::Bool(data[0] & 0x80 == 0),
            InputProperty::TouchPad2Id => ValueType::U8(data[0] & 0x7F),
            InputProperty::TouchPad2X => {
                ValueType::U16((data[1] as u16 & 0x0F) << 8 | data[0] as u16)
            }
            InputProperty::TouchPad2Y => {
                ValueType::U16(((data[1] as u16) << 4) | (data[0] as u16 & 0xF0) >> 4)
            }
            InputProperty::R2FeedbackOn | InputProperty::L2FeedbackOn => {
                ValueType::Bool(data[0] & 0x10 == 0x10)
            }
            InputProperty::R2FeedbackValue | InputProperty::L2FeedbackValue => {
                ValueType::U8(data[0] & 0x0F)
            }
        }
    }
}

fn gyro_accel_into_u16(data: &[u8]) -> i16 {
    (data[1] as i16) << 8 | data[0] as i16
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub(crate) enum OutputProperty {
    Red,
    Green,
    Blue,

    RightEffectMode,
    RightEffectParameter1,
    RightEffectParameter2,
    RightEffectParameter3,
    RightEffectParameter4,
    RightEffectParameter5,
    RightEffectParameter6,
    RightEffectParameter7,

    LeftEffectMode,
    LeftEffectParameter1,
    LeftEffectParameter2,
    LeftEffectParameter3,
    LeftEffectParameter4,
    LeftEffectParameter5,
    LeftEffectParameter6,
    LeftEffectParameter7,

    PlayerLight,
    Mute,
}

impl OutputProperty {
    pub(crate) fn byte(self) -> usize {
        match self {
            OutputProperty::Red => 45,
            OutputProperty::Green => 46,
            OutputProperty::Blue => 47,

            OutputProperty::Mute => 9,
            OutputProperty::RightEffectMode => 11,
            OutputProperty::RightEffectParameter1 => 12,
            OutputProperty::RightEffectParameter2 => 13,
            OutputProperty::RightEffectParameter3 => 14,
            OutputProperty::RightEffectParameter4 => 15,
            OutputProperty::RightEffectParameter5 => 16,
            OutputProperty::RightEffectParameter6 => 17,
            OutputProperty::RightEffectParameter7 => 18,

            OutputProperty::LeftEffectMode => 22,
            OutputProperty::LeftEffectParameter1 => 23,
            OutputProperty::LeftEffectParameter2 => 24,
            OutputProperty::LeftEffectParameter3 => 25,
            OutputProperty::LeftEffectParameter4 => 26,
            OutputProperty::LeftEffectParameter5 => 27,
            OutputProperty::LeftEffectParameter6 => 28,
            OutputProperty::LeftEffectParameter7 => 29,

            OutputProperty::PlayerLight => 44,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ComboProperty {
    Symbol(Symbols),
    DPad(DPad),
    LB(bool),
    RB(bool),
    LT(Trigger),
    RT(Trigger),
    LeftPad(AnalogPad),
    RightPad(AnalogPad),
}

impl ComboProperty {
    pub(crate) fn base(&self) -> Self {
        match self {
            ComboProperty::Symbol(_) => Self::Symbol(Symbols::None),
            ComboProperty::DPad(_) => Self::DPad(DPad::None),
            ComboProperty::LB(_) => Self::LB(false),
            ComboProperty::RB(_) => Self::RB(false),
            ComboProperty::LT(_) => Self::LT(Trigger::new(0)),
            ComboProperty::RT(_) => Self::RT(Trigger::new(0)),
            ComboProperty::LeftPad(_) => Self::LeftPad(AnalogPad::new(0, 0)),
            ComboProperty::RightPad(_) => Self::RightPad(AnalogPad::new(0, 0)),
        }
    }

    pub(crate) fn offset(self) -> Offset {
        match self {
            ComboProperty::LeftPad(_) => Offset::bytes(1..3),
            ComboProperty::RightPad(_) => Offset::bytes(3..5),
            ComboProperty::Symbol(_) => Offset::bits(8, 4..8),
            ComboProperty::DPad(_) => Offset::bits(8, 0..4),
            ComboProperty::LB(_) => Offset::bit(9, 0),
            ComboProperty::RB(_) => Offset::bit(9, 1),
            ComboProperty::LT(_) => Offset::byte(5),
            ComboProperty::RT(_) => Offset::byte(6),
        }
    }

    // TODO: there must be a better way to do this...

    pub(crate) fn to_dpad(self) -> DPad {
        match self {
            ComboProperty::Symbol(_) => todo!(),
            ComboProperty::DPad(dpad) => dpad,
            ComboProperty::LB(_) => todo!(),
            ComboProperty::RB(_) => todo!(),
            ComboProperty::LT(_) => todo!(),
            ComboProperty::RT(_) => todo!(),
            ComboProperty::LeftPad(_) => todo!(),
            ComboProperty::RightPad(_) => todo!(),
        }
    }

    pub(crate) fn to_symbols(self) -> Symbols {
        match self {
            ComboProperty::Symbol(sym) => sym,
            ComboProperty::DPad(_) => todo!(),
            ComboProperty::LB(_) => todo!(),
            ComboProperty::RB(_) => todo!(),
            ComboProperty::LT(_) => todo!(),
            ComboProperty::RT(_) => todo!(),
            ComboProperty::LeftPad(_) => todo!(),
            ComboProperty::RightPad(_) => todo!(),
        }
    }

    pub(crate) fn to_trigger(self) -> Trigger {
        match self {
            ComboProperty::Symbol(_) => todo!(),
            ComboProperty::DPad(_) => todo!(),
            ComboProperty::LB(_) => todo!(),
            ComboProperty::RB(_) => todo!(),
            ComboProperty::LT(v) => v,
            ComboProperty::RT(v) => v,
            ComboProperty::LeftPad(_) => todo!(),
            ComboProperty::RightPad(_) => todo!(),
        }
    }
}

impl ComboProperty {
    pub(crate) fn convert(&self, data: &[u8]) -> Self {
        match self {
            ComboProperty::Symbol(_) => ComboProperty::Symbol(data[0].into()),
            ComboProperty::DPad(_) => ComboProperty::DPad(data[0].into()),
            ComboProperty::LB(_) => ComboProperty::LB(data[0] == 0x01),
            ComboProperty::RB(_) => ComboProperty::RB(data[0] == 0x01),
            ComboProperty::LeftPad(_) => ComboProperty::LeftPad(AnalogPad::new(data[0], data[1])),
            ComboProperty::RightPad(_) => ComboProperty::RightPad(AnalogPad::new(data[0], data[1])),
            ComboProperty::LT(_) => ComboProperty::LT(Trigger::new(data[0])),
            ComboProperty::RT(_) => ComboProperty::RT(Trigger::new(data[0])),
        }
    }
}
