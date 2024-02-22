use std::ops::Range;

/// Specify the byte and bit offset for properties as singular values or intervals
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

    /// Create an instance with a single byte
    pub(crate) fn byte(byte: usize) -> Self {
        Offset {
            bytes: byte..byte + 1,
            bits: 0..8,
        }
    }

    /// Create an instance with a single bit
    pub(crate) fn bit(byte: usize, bit: usize) -> Self {
        Offset {
            bytes: byte..byte + 1,
            bits: bit..bit + 1,
        }
    }
    
    pub(crate) fn is_whole_byte(&self) -> bool {
        self.bits == (0..8)
    }
    
    pub(crate) fn is_single_byte(&self) -> bool {
        self.bytes.clone().count() == 1
    }
}
