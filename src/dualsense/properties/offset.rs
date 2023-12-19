use std::ops::Range;

pub(crate) struct Offset {
    pub(crate) byte: Range<usize>,
    pub(crate) bit: Range<usize>,
}

impl Offset {
    pub(crate) fn new(byte: Range<usize>, bit: Range<usize>) -> Self {
        Offset { byte, bit }
    }
}
