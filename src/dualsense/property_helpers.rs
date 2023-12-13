use std::ops::Range;

pub(crate) struct Offset {
    pub(crate) byte: Range<usize>,
    pub(crate) bit: Range<usize>,
}

impl Offset {
    fn new(byte: Range<usize>, bit: Range<usize>) -> Self {
        Offset { byte, bit }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub(crate) enum Property {
    LeftPadX,
    LeftPadY,
    RightPadX,
    RightPadY,
    L2,
    R2,
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
        }
    }
}

enum Callback {
    FromU8(Box<dyn Fn(u8)>),
    FromU16(Box<dyn Fn(u16)>),
}

trait ToCallbackU8 {
    fn to_callback(self) -> Callback;
}

trait ToCallbackU16 {
    fn to_callback(self) -> Callback;
}

impl<F> ToCallbackU8 for F
where
    F: Fn(u8) + 'static,
{
    fn to_callback(self) -> Callback {
        Callback::FromU8(Box::new(self))
    }
}

impl<F> ToCallbackU16 for F
where
    F: Fn(u16) + 'static,
{
    fn to_callback(self) -> Callback {
        Callback::FromU16(Box::new(self))
    }
}

#[derive(Clone, Copy)]
pub(crate) enum ValueType {
    U8(u8),
    U16(u16),
}

impl ValueType {
    pub(crate) fn to_u8(self) -> u8 {
        match self {
            ValueType::U8(v) => v,
            ValueType::U16(_) => todo!(),
        }
    }

    pub(crate) fn from_slice(&self, data: &[u8]) -> Self {
        match self {
            ValueType::U8(_) => ValueType::U8(*data.first().unwrap()),
            ValueType::U16(_) => todo!(),
        }
    }
}

impl PartialEq for ValueType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::U8(l), Self::U8(r)) => l == r,
            (Self::U16(l), Self::U16(r)) => l == r,
            _ => false,
        }
    }
}

impl Eq for ValueType {}
