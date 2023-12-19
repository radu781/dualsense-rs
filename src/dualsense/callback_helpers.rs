use crate::properties::{dpad::DPad, symbols::Symbols};

enum Callback {
    FromU8(Box<dyn Fn(u8)>),
    FromU16(Box<dyn Fn(u16)>),
    FromDPad(Box<dyn Fn(DPad)>),
    FromSymbols(Box<dyn Fn(Symbols)>),
}

trait ToCallbackU8 {
    fn to_callback(self) -> Callback;
}

trait ToCallbackU16 {
    fn to_callback(self) -> Callback;
}

trait ToCallbackDPad {
    fn to_callback(self) -> Callback;
}

trait ToCallbackSymbols {
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

impl<F> ToCallbackDPad for F
where
    F: Fn(DPad) + 'static,
{
    fn to_callback(self) -> Callback {
        Callback::FromDPad(Box::new(self))
    }
}

impl<F> ToCallbackSymbols for F
where
    F: Fn(Symbols) + 'static,
{
    fn to_callback(self) -> Callback {
        Callback::FromSymbols(Box::new(self))
    }
}
