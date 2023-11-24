//! Name of the emulator that created this save state.

use super::{Data, Ident};

/// `NAME` block.
///
/// Contains the name and version of the originating emulator in ASCII. This is
/// an *optional* block.
///
/// While optional, it is highly recommended to be included in every
/// implementation – it allows the user to know which emulator and version is
/// compatible with the native save state format contained in this file.
///
/// When used, this block should come first.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct Name(String);

impl Name {
    /// Identifier for this block.
    const IDENT: Ident = Ident::new(*b"NAME");
}

impl Name {
    /// Constructs a new `Name`.
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(name: impl ToString) -> Self {
        Self(name.to_string())
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Data for Name {
    fn ident() -> Ident {
        Self::IDENT
    }

    fn len(&self) -> u32 {
        u32::try_from(self.0.len()).unwrap()
    }
}
