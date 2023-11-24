//! BESS data blocks.

#![allow(clippy::len_without_is_empty)]

use std::fmt::{Debug, Display};

pub mod core;
pub mod end;
pub mod info;
pub mod name;

/// Block kind identifier.
///
/// Unique four-letter ASCII identifier.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct Ident([u8; 4]);

impl Ident {
    /// Constructs a new `Ident`.
    #[must_use]
    pub const fn new(value: [u8; 4]) -> Self {
        Self(value)
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::str::from_utf8(&self.0).unwrap_or("None"))
    }
}

/// Complete block structure.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct Block {
    head: Header,
    body: Box<dyn Data>,
}

impl Block {
    /// Gets the block's identifier.
    #[must_use]
    pub const fn ident(&self) -> &Ident {
        &self.head.ident
    }

    /// Gets the block's length.
    #[must_use]
    pub const fn len(&self) -> u32 {
        self.head.len
    }

    #[must_use]
    pub const fn body(&self) -> &dyn Data {
        &*self.body
    }
}

impl<T: Data + 'static> From<T> for Block {
    fn from(body: T) -> Self {
        Self {
            head: body.header(),
            body: Box::new(body),
        }
    }
}

/// Header preceding block data.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct Header {
    /// Unique identifier specifying the block type.
    ident: Ident,
    /// Length of the block, in bytes, excluding this header.
    len: u32,
}

/// Block body containing data.
#[cfg_attr(feature = "serde", typetag::serde)]
pub trait Data: Debug {
    /// Gets this block's identifier.
    fn ident() -> Ident
    where
        Self: Sized;

    /// Gets this block's identifier.
    fn len(&self) -> u32;

    /// Generates the header for this body.
    fn header(&self) -> Header
    where
        Self: Sized,
    {
        Header {
            ident: Self::ident(),
            len: self.len(),
        }
    }
}
