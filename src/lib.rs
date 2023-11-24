//! Best Effort Save State (BESS).
//!
//! # Motivation
//!
//! BESS is a save state format specification designed to allow different
//! emulators, as well as majorly different versions of the same emulator, to
//! import save states from other BESS-compliant save states. BESS works by
//! appending additional, implementation-agnostic information about the
//! emulation state. This allows a single save state file to be read as both a
//! fully-featured, implementation specific save state which includes detailed
//! timing information; and as a portable "best effort" save state that
//! represents a state accurately enough to be restored in casual use-cases.[^1]
//!
//! # Specification
//!
//! See the official specification over at [SameBoy][bess].
//!
//! <!-- Footnotes -->
//! [^1]: Several parts of this documentation were shamelessly copied directly
//!       from the [source][bess]. Used pursuant to the rights granted by the
//!       MIT License.
//!
//! <!-- Reference-style links -->
//! [bess]: https://github.com/LIJI32/SameBoy/blob/master/BESS.md

#![warn(clippy::pedantic)]

pub mod block;

mod build;
mod error;
#[cfg(feature = "serde")]
mod serde;

pub use self::block::{Block, Data};
pub use self::error::{Error, Result};
#[cfg(feature = "serde")]
pub use self::serde::{de, ser};

/// BESS document.
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[derive(Debug)]
pub struct Bess {
    /// Context for BESS file.
    ctx: Vec<u8>,
    /// Blocks specifying contents.
    blx: Vec<Block>,
    /// Footer for BESS file.
    end: Footer,
}

/// Footer appended to declare BESS files.
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[derive(Debug)]
struct Footer {
    start: u32,
    magic: u32,
}

impl Footer {
    /// Constructs a new `Footer`.
    pub fn new(start: u32) -> Self {
        Self {
            start,
            magic: MAGIC,
        }
    }
}

/// Magic number for BESS files.
///
/// This is simply the ASCII string "BESS" in hexadecimal.
const MAGIC: u32 = u32::from_le_bytes(*b"BESS");
