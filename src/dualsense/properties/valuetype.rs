use super::{
    analog_pad::AnalogPad, dpad::DPad, property::ComboProperty, symbols::Symbols, trigger::Trigger,
};

#[derive(Clone, Copy, Debug)]
pub(crate) enum ValueType {
    U8(u8),
    U16(u16),
    I16(i16),
    Pad(DPad),
    Symbol(Symbols),
    Bool(bool),
    Combo(ComboProperty),
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum NewValueType {
    Combo(ComboProperty),
}

impl ValueType {
    pub(crate) fn to_u8(self) -> u8 {
        match self {
            ValueType::U8(v) => v,
            ValueType::U16(_) => todo!(),
            ValueType::I16(_) => todo!(),
            ValueType::Pad(_) => todo!(),
            ValueType::Symbol(_) => todo!(),
            ValueType::Bool(_) => todo!(),
            ValueType::Combo(_) => todo!(),
        }
    }

    pub(crate) fn to_u16(self) -> u16 {
        match self {
            ValueType::U8(_) => todo!(),
            ValueType::U16(v) => v,
            ValueType::I16(_) => todo!(),
            ValueType::Pad(_) => todo!(),
            ValueType::Symbol(_) => todo!(),
            ValueType::Bool(_) => todo!(),
            ValueType::Combo(_) => todo!(),
        }
    }

    pub(crate) fn to_i16(self) -> i16 {
        match self {
            ValueType::U8(_) => todo!(),
            ValueType::U16(_) => todo!(),
            ValueType::I16(v) => v,
            ValueType::Pad(_) => todo!(),
            ValueType::Symbol(_) => todo!(),
            ValueType::Bool(_) => todo!(),
            ValueType::Combo(_) => todo!(),
        }
    }

    pub(crate) fn to_dpad(self) -> DPad {
        match self {
            ValueType::U8(_) => todo!(),
            ValueType::U16(_) => todo!(),
            ValueType::I16(_) => todo!(),
            ValueType::Pad(v) => v,
            ValueType::Symbol(_) => todo!(),
            ValueType::Bool(_) => todo!(),
            ValueType::Combo(_) => todo!(),
        }
    }

    pub(crate) fn to_symbol(self) -> Symbols {
        match self {
            ValueType::U8(_) => todo!(),
            ValueType::U16(_) => todo!(),
            ValueType::I16(_) => todo!(),
            ValueType::Pad(_) => todo!(),
            ValueType::Symbol(v) => v,
            ValueType::Bool(_) => todo!(),
            ValueType::Combo(_) => todo!(),
        }
    }

    pub(crate) fn to_bool(self) -> bool {
        match self {
            ValueType::U8(_) => todo!(),
            ValueType::U16(_) => todo!(),
            ValueType::I16(_) => todo!(),
            ValueType::Pad(_) => todo!(),
            ValueType::Symbol(_) => todo!(),
            ValueType::Bool(v) => v,
            ValueType::Combo(_) => todo!(),
        }
    }
    pub(crate) fn to_combo(self) -> ComboProperty {
        match self {
            ValueType::U8(_) => todo!(),
            ValueType::U16(_) => todo!(),
            ValueType::I16(_) => todo!(),
            ValueType::Pad(_) => todo!(),
            ValueType::Symbol(_) => todo!(),
            ValueType::Bool(_) => todo!(),
            ValueType::Combo(v) => v,
        }
    }

    pub(crate) fn to_analog(self) -> AnalogPad {
        match self {
            ValueType::U8(_) => todo!(),
            ValueType::U16(_) => todo!(),
            ValueType::I16(_) => todo!(),
            ValueType::Pad(_) => todo!(),
            ValueType::Symbol(_) => todo!(),
            ValueType::Bool(_) => todo!(),
            ValueType::Combo(v) => match v {
                ComboProperty::LeftPad(v) => v,
                ComboProperty::RightPad(v) => v,
                _ => todo!(),
            },
        }
    }

    pub(crate) fn to_trigger(self) -> Trigger {
        match self {
            ValueType::U8(_) => todo!(),
            ValueType::U16(_) => todo!(),
            ValueType::I16(_) => todo!(),
            ValueType::Pad(_) => todo!(),
            ValueType::Symbol(_) => todo!(),
            ValueType::Bool(_) => todo!(),
            ValueType::Combo(v) => match v {
                ComboProperty::LT(v) => v,
                ComboProperty::RT(v) => v,
                _ => todo!(),
            },
        }
    }
}

impl PartialEq for ValueType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::U8(l), Self::U8(r)) => l == r,
            (Self::U16(l), Self::U16(r)) => l == r,
            (Self::Pad(l), Self::Pad(r)) => l == r,
            (Self::Symbol(l), Self::Symbol(r)) => l == r,
            (Self::Bool(l), Self::Bool(r)) => l == r,
            (Self::Combo(l), Self::Combo(r)) => l == r,
            _ => false,
        }
    }
}

impl Eq for ValueType {}
