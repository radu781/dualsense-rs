use std::ops::Range;

pub(crate) struct Offset {
    pub(crate) bytes: Range<usize>,
    pub(crate) bits: Range<usize>,
}

impl Offset {
    /// Create an instance with whole bytes
    pub(crate) fn bytes(bytes: Range<usize>) -> Self {
        Offset { bytes, bits: 0..8 }
    }

    /// Create an instance with a partial byte
    pub(crate) fn bits(byte: usize, bits: Range<usize>) -> Self {
        Offset {
            bytes: byte..byte + 1,
            bits,
        }
    }

    /// Create an instance with a single bit
    pub(crate) fn bit(byte: usize, bit: usize) -> Self {
        Offset {
            bytes: byte..byte + 1,
            bits: bit..bit + 1,
        }
    }
}
