//! Core state information.

#[cfg(feature = "serde")]
use serde_with::{As, Bytes};

use super::{Data, Ident};

/// `CORE` block.
///
/// Contains both core state information as well as basic information about the
/// BESS version used. This is a **required** block.
///
/// This block must be the first block, unless the `NAME` or `INFO` blocks
/// exist, then it must come directly after them.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct Core {
    /// BESS major and minor version numbers.
    pub version: Version,
    /// A four-character ASCII model identifier.
    pub model: Model,
    /// Saved values of registers.
    pub reg: Registers,
    /// Large buffer content pointers.
    pub mem: Locations,
}

impl Core {
    /// Identifier for this block.
    const IDENT: Ident = Ident::new(*b"CORE");
    /// Constant length of this block.
    const LEN: u32 = 0xd0;
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Data for Core {
    fn ident() -> Ident {
        Self::IDENT
    }

    fn len(&self) -> u32 {
        Self::LEN
    }
}

/// BESS version.
///
/// Both major and minor versions should be 1. Implementations are expected to
/// reject incompatible majors, but still attempt to read newer minor versions.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct Version {
    /// Major BESS version as a 16-bit integer.
    pub major: u16,
    /// Minor BESS version as a 16-bit integer.
    pub minor: u16,
}

/// Model identifier.
///
/// Four-character string to identify Game Boy models.
///
/// 1. The first letter represents mutually-incompatible families of models and
///    is required. The allowed values are `'G'` for the original Game Boy
///    family, `'S'` for the Super Game Boy family, and `'C'` for the Game Boy
///    Color and Advance family.
/// 2. The second letter represents a specific model within the family, and is
///    optional (If an implementation does not distinguish between specific
///    models in a family, a space character may be used). The allowed values
///    for family G are `'D'` for DMG and `'M'` for MGB; the allowed values for
///    family S are `'N'` for NTSC, `'P'` for PAL, and `'2'` for SGB2; and the allowed
///    values for family C are `'C'` for CGB, and `'A'` for the various GBA line
///    models.
/// 3. The third letter represents a specific CPU revision within a model, and
///    is optional (If an implementation does not distinguish between revisions,
///    a space character may be used). The allowed values for model GD (DMG) are
///    `'0'` and `'A'`, through `'C'`; the allowed values for model CC (CGB) are `'0'`
///    and `'A'`, through `'E'`; the allowed values for model CA (AGB, AGS, GBP) are
///    `'0'`, `'A'` and `'B'`; and for every other model this value must be a space
///    character.
/// 4. The last character is used for padding and must be a space character.
///
/// # Examples
///
/// |  Model   | Description
/// |----------|-------------
/// | `"GD  "` | A DMG of an unspecified revision.
/// | `"CCE "` | A CGB using CPU revision E.
/// | `"S   "` | Some model of the SGB family.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct Model(pub [u8; 4]);

/// Register values.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct Registers {
    /// The value of the PC register.
    pub pc: u16,
    /// The value of the AF register.
    pub af: u16,
    /// The value of the BC register.
    pub bc: u16,
    /// The value of the DE register.
    pub de: u16,
    /// The value of the HL register.
    pub hl: u16,
    /// The value of the SP register.
    pub sp: u16,
    /// The value of IME (0 or 1).
    pub ime: bool,
    /// The value of the IE register.
    pub ie: u8,
    /// Execution state (0 = running; 1 = halted; 2 = stopped).
    pub exe: Execution,
    /// The values of every memory-mapped register (128 bytes).
    #[cfg_attr(feature = "serde", serde(with = "As::<Bytes>"))]
    pub mmio: Mmio,
}

/// Execution state.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
#[rustfmt::skip]
pub enum Execution {
    Running = 0,
    Halted  = 1,
    Stopped = 2,
}

/// Memory-mapped registers.
///
/// The values of memory-mapped registers should be written 'as-is' to memory as
/// if the actual ROM wrote them, with the following exceptions and notes:
///
/// - Unused registers have Don't-Care values which should be ignored.
/// - Unused register bits have Don't-Care values which should be ignored.
/// - If the model is CGB or newer, the value of KEY0 (FF4C) must be valid as it
///   determines DMG mode.
///   - Bit 2 determines DMG mode. A value of `0x04` usually denotes DMG mode,
///     while a value of `0x80` usually denotes CGB mode.
/// - Object priority is derived from KEY0 (FF4C) instead of OPRI (FF6C) because
///   OPRI can be modified after booting, but only the value of OPRI during boot
///   ROM execution takes effect.
/// - If a register doesn't exist on the emulated model (For example, KEY0
///   (FF4C) on a DMG), its value should be ignored.
/// - BANK (FF50) should be 0 if the boot ROM is still mapped, and 1 otherwise,
///   and must be valid.
/// - Implementations should not start a serial transfer when writing the value
///   of SB.
/// - Similarly, no value of `NRx4` should trigger a sound pulse on save state
///   load.
/// - And similarly again, implementations should not trigger DMA transfers when
///   writing the values of DMA or HDMA5.
/// - The value store for DIV will be used to set the internal divisor to
///   `DIV << 8`.
/// - Implementation should apply care when ordering the write operations (For
///   example, writes to NR52 must come before writes to the other APU
///   registers).
pub type Mmio = [u8; 0x80];

/// Locations of large buffers.
///
/// The contents of large buffers are stored outside of BESS structure so data
/// from an implementation's native save state format can be reused. The offsets
/// are absolute offsets from the save state file's start. Background and object
/// palette sizes must be 0 for models prior to Game Boy Color.
///
/// An implementation needs handle size mismatches gracefully. For example, if
/// too large MBC RAM size is specified, the superfluous data should be ignored.
/// On the other hand, if a too small VRAM size is specified (For example, if
/// it's a save state from an emulator emulating a CGB in DMG mode, and it
/// didn't save the second CGB VRAM bank), the implementation is expected to set
/// that extra bank to all zeros.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct Locations {
    /// WRAM pointer.
    pub wram: Pointer,
    /// VRAM pointer.
    pub vram: Pointer,
    /// ERAM pointer.
    pub eram: Pointer,
    /// OAM pointer.
    pub oam: Pointer,
    /// HRAM pointer.
    pub hram: Pointer,
    /// Background palettes pointer.
    pub bgp: Pointer,
    /// Object palettes pointer.
    pub obj: Pointer,
}

/// Wide-pointers to buffers.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct Pointer {
    /// Size of the buffer.
    pub len: u32,
    /// Absolute offset.
    pub ptr: u32,
}
