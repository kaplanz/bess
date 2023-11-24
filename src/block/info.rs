//! Information about the ROM this save state originates from.

use super::{Data, Ident};

type Title = [u8; 16];

/// `INFO` block.
///
/// Contains information about the ROM this save state originates from. This is
/// an *optional* block.
///
/// When used, this block should come before `CORE` but after `NAME`.
#[allow(unused)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct Info {
    /// Title provided by the ROM header.
    ///
    /// Bytes 0x134-0x143 from the ROM.
    title: Title,
    /// Global checksum provided by the ROM header.
    ///
    /// Bytes 0x14E-0x14F from the ROM.
    gchk: u16,
}

impl Info {
    /// Identifier for this block.
    const IDENT: Ident = Ident::new(*b"INFO");
    /// Constant length of this block.
    const LEN: u32 = 0x12;
}

impl Info {
    /// Constructs a new `Name`.
    pub fn new(title: impl Into<Title>, gchk: u16) -> Self {
        Self {
            title: title.into(),
            gchk,
        }
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Data for Info {
    fn ident() -> Ident {
        Self::IDENT
    }

    fn len(&self) -> u32 {
        Self::LEN
    }
}
