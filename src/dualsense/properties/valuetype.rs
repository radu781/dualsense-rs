use super::{dpad::DPad, symbols::Symbols};

#[derive(Clone, Copy, Debug)]
pub(crate) enum ValueType {
    U8(u8),
    U16(u16),
    Pad(DPad),
    Symbol(Symbols),
    Bool(bool),
}

impl ValueType {
    pub(crate) fn to_u8(self) -> u8 {
        match self {
            ValueType::U8(v) => v,
            ValueType::U16(_) => todo!(),
            ValueType::Pad(_) => todo!(),
            ValueType::Symbol(_) => todo!(),
            ValueType::Bool(_) => todo!(),
        }
    }

    pub(crate) fn to_u16(self) -> u16 {
        match self {
            ValueType::U8(_) => todo!(),
            ValueType::U16(v) => v,
            ValueType::Pad(_) => todo!(),
            ValueType::Symbol(_) => todo!(),
            ValueType::Bool(_) => todo!(),
        }
    }

    pub(crate) fn to_dpad(self) -> DPad {
        match self {
            ValueType::U8(_) => todo!(),
            ValueType::U16(_) => todo!(),
            ValueType::Pad(v) => v,
            ValueType::Symbol(_) => todo!(),
            ValueType::Bool(_) => todo!(),
        }
    }

    pub(crate) fn to_symbol(self) -> Symbols {
        match self {
            ValueType::U8(_) => todo!(),
            ValueType::U16(_) => todo!(),
            ValueType::Pad(_) => todo!(),
            ValueType::Symbol(v) => v,
            ValueType::Bool(_) => todo!(),
        }
    }

    pub(crate) fn to_bool(self) -> bool {
        match self {
            ValueType::U8(_) => todo!(),
            ValueType::U16(_) => todo!(),
            ValueType::Pad(_) => todo!(),
            ValueType::Symbol(_) => todo!(),
            ValueType::Bool(v) => v,
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
            _ => false,
        }
    }
}

impl Eq for ValueType {}
