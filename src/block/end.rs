//! Mark the end of BESS data.

use super::{Data, Ident};

/// `END` block.
///
/// Contains no data itself, but marks the end of BESS data. This is a
/// **required** block.
///
/// It must be the last block.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default)]
pub struct End;

impl End {
    /// Identifier for this block.
    const IDENT: Ident = Ident::new(*b"END ");
    /// Constant length of this block.
    const LEN: u32 = 0;
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Data for End {
    fn ident() -> Ident {
        Self::IDENT
    }

    fn len(&self) -> u32 {
        Self::LEN
    }
}
