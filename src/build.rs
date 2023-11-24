use std::iter;

use crate::block::core::Core;
use crate::block::end::End;
use crate::block::info::Info;
use crate::block::name::Name;
use crate::{Bess, Block, Data, Error, Footer, Result};

impl Bess {
    // Create a builder for `Bess`.
    #[must_use]
    pub fn builder() -> Builder {
        Builder::default()
    }
}

/// Builder for `Bess`.
#[derive(Debug, Default)]
pub struct Builder {
    name: Option<Name>,
    info: Option<Info>,
    core: Needed<Core>,
    xtra: Vec<Block>,
}

impl Builder {
    /// Builds a new `Bess`.
    pub fn build(self, ctx: impl Into<Vec<u8>>) -> Result<Bess> {
        // Prepare context buffer
        let ctx = ctx.into();
        // Extract parts
        let Self {
            name,
            info,
            core,
            xtra,
        } = self;
        // Check needed fields
        let core = core.get()?;
        // Convert to blocks
        let name = name.map(Block::from);
        let info = info.map(Block::from);
        let core = core.map(Block::from);
        // Chain blocks together
        let blx =
            // First, add ordered blocks
            [name, info, core]
            .into_iter()
            .flatten()
            // Next, chain extra blocks
            .chain(xtra)
            // Then, finish with end block
            .chain(iter::once(End.into()))
            .collect();
        // Calculate footer
        let end = Footer::new(u32::try_from(ctx.len()).map_err(|_| Error::TooLarge)?);
        // Build and return
        Ok(Bess { ctx, blx, end })
    }

    pub fn name(mut self, name: impl ToString) -> Self {
        self.name = Some(Name::new(name));
        self
    }

    pub fn info(mut self, info: Info) -> Self {
        self.info = Some(info);
        self
    }

    pub fn core(mut self, core: Core) -> Self {
        self.core = Needed(core.into());
        self
    }

    pub fn block<T: Data + 'static>(mut self, body: T) -> Self {
        self.xtra.push(Block::from(body));
        self
    }
}

/// Wrapper around a required field.
#[derive(Debug)]
struct Needed<T: Data>(Option<T>);

impl<T: Data> Needed<T> {
    /// Converts to the required field if it has been provided.
    ///
    /// # Note
    ///
    /// If the required field is present, this function guarantees the returned
    /// `Ok` variant contains `Some(T)`.
    fn get(self) -> Result<Option<T>> {
        self.0.ok_or(Error::Required(T::ident())).map(Some)
    }
}

impl<T: Data> Default for Needed<T> {
    fn default() -> Self {
        Self(Option::default())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::block::core::{Execution, Locations, Model, Pointer, Registers, Version};

    #[rustfmt::skip]
    pub const BYTES: &[u8] = &[
        // Bess: Contents
        // -- snip --
        // Bess: Blocks
        // Bess: Block: NAME
        b'N', b'A', b'M', b'E', // head.ident
        0x04, 0x00, 0x00, 0x00, // head.len
        b'b', b'e', b's', b's', // body.name
        // Bess: Block: INFO
        b'I', b'N', b'F', b'O', // head.ident
        0x12, 0x00, 0x00, 0x00, // head.len
        b'B', b'E', b'S', b'S', // body.info.title
        b' ', b'T', b'e', b's', // ...
        b't', b'i', b'n', b'g', // ...
        b' ', b'R', b'o', b'm', // ...
        0xcd, 0xab,             // body.info.gchk
        // Bess: Block: CORE
        b'C', b'O', b'R', b'E', // head.ident
        0xd0, 0x00, 0x00, 0x00, // head.len
        0x01, 0x00,             // body.core.version.major
        0x01, 0x00,             // body.core.version.minor
        b'D', b' ', b' ', b' ', // body.core.model
        0x00, 0x01,             // body.core.reg.pc
        0xb0, 0x01,             // body.core.reg.af
        0x13, 0x00,             // body.core.reg.bc
        0xd8, 0x00,             // body.core.reg.de
        0x4d, 0x01,             // body.core.reg.hl
        0xfe, 0xff,             // body.core.reg.sp
        0x01,                   // body.core.reg.ime
        0xe0,                   // body.core.reg.ie
        0x00,                   // body.core.reg.exe
                                // body.core.reg.mmio
        0xff, 0x00, 0x7e, 0xff, // ... 0xff00
        0xcf, 0x00, 0x00, 0xf8, // ... 0xff04
        0xff, 0xff, 0xff, 0xff, // ... 0xff08
        0xff, 0xff, 0xff, 0xe1, // ... 0xff0c
        0x00, 0x80, 0xf3, 0xc1, // ... 0xff10
        0x87, 0xff, 0x00, 0x00, // ... 0xff14
        0x00, 0x00, 0x00, 0x00, // ... 0xff18
        0x00, 0x00, 0x00, 0xff, // ... 0xff1c
        0x00, 0x00, 0x00, 0x00, // ... 0xff20
        0x77, 0xf3, 0x80, 0xff, // ... 0xff24
        0xff, 0xff, 0xff, 0xff, // ... 0xff28
        0xff, 0xff, 0xff, 0xff, // ... 0xff2c
        0x00, 0x00, 0x00, 0x00, // ... 0xff30
        0x00, 0x00, 0x00, 0x00, // ... 0xff34
        0x00, 0x00, 0x00, 0x00, // ... 0xff38
        0x00, 0x00, 0x00, 0x00, // ... 0xff3c
        0x91, 0x01, 0x00, 0x00, // ... 0xff40
        0x99, 0x00, 0x00, 0xfc, // ... 0xff44
        0x00, 0x00, 0x00, 0x00, // ... 0xff48
        0xff, 0xff, 0xff, 0xff, // ... 0xff4c
        0xfe, 0xff, 0xff, 0xff, // ... 0xff50
        0xff, 0xff, 0xff, 0xff, // ... 0xff54
        0xff, 0xff, 0xff, 0xff, // ... 0xff58
        0xff, 0xff, 0xff, 0xff, // ... 0xff5c
        0xff, 0xff, 0xff, 0xff, // ... 0xff60
        0xff, 0xff, 0xff, 0xff, // ... 0xff64
        0xff, 0xff, 0xff, 0xff, // ... 0xff68
        0xff, 0xff, 0xff, 0xff, // ... 0xff6c
        0xff, 0xff, 0xff, 0xff, // ... 0xff70
        0xff, 0xff, 0xff, 0xff, // ... 0xff74
        0xff, 0xff, 0xff, 0xff, // ... 0xff78
        0xff, 0xff, 0xff, 0xff, // ... 0xff7c
        0x00, 0x20, 0x00, 0x00, // body.core.mem.wram.len
        0x00, 0xa0, 0x00, 0x00, // body.core.mem.wram.ptr
        0x00, 0x20, 0x00, 0x00, // body.core.mem.vram.len
        0x00, 0x80, 0x00, 0x00, // body.core.mem.vram.ptr
        0x00, 0x20, 0x00, 0x00, // body.core.mem.eram.len
        0x00, 0xc0, 0x00, 0x00, // body.core.mem.eram.ptr
        0xa0, 0x00, 0x00, 0x00, // body.core.mem.oam.len
        0x00, 0xfe, 0x00, 0x00, // body.core.mem.oam.ptr
        0x7f, 0x00, 0x00, 0x00, // body.core.mem.hram.len
        0x80, 0xff, 0x00, 0x00, // body.core.mem.hram.ptr
        0x00, 0x00, 0x00, 0x00, // body.core.mem.bgp.len
        0x00, 0x00, 0x00, 0x00, // body.core.mem.bgp.ptr
        0x00, 0x00, 0x00, 0x00, // body.core.mem.obj.len
        0x00, 0x00, 0x00, 0x00, // body.core.mem.obj.ptr
        // Bess: Block: END
        b'E', b'N', b'D', b' ', // head.ident
        0x00, 0x00, 0x00, 0x00, // head.len
        // Bess: Footer
        0x00, 0x00, 0x00, 0x00, // end.start
        b'B', b'E', b'S', b'S', // end.magic
    ];

    pub fn setup() -> Bess {
        let core = Core {
            version: Version { major: 1, minor: 1 },
            model: Model(*b"D   "),
            reg: Registers {
                pc: 0x0100,
                af: 0x01b0,
                bc: 0x0013,
                de: 0x00d8,
                hl: 0x014d,
                sp: 0xfffe,
                ime: true,
                ie: 0xe0,
                exe: Execution::Running,
                mmio: [
                    0xff, 0x00, 0x7e, 0xff, 0xcf, 0x00, 0x00, 0xf8, // 0xff00
                    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xe1, // 0xff08
                    0x00, 0x80, 0xf3, 0xc1, 0x87, 0xff, 0x00, 0x00, // 0xff10
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, // 0xff18
                    0x00, 0x00, 0x00, 0x00, 0x77, 0xf3, 0x80, 0xff, // 0xff20
                    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 0xff28
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0xff30
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0xff38
                    0x91, 0x01, 0x00, 0x00, 0x99, 0x00, 0x00, 0xfc, // 0xff40
                    0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, // 0xff48
                    0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 0xff50
                    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 0xff58
                    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 0xff60
                    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 0xff68
                    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 0xff70
                    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 0xff78
                ],
            },
            #[rustfmt::skip]
            mem: Locations {
                wram: Pointer { len: 0x2000, ptr: 0xa000, },
                vram: Pointer { len: 0x2000, ptr: 0x8000, },
                eram: Pointer { len: 0x2000, ptr: 0xc000, },
                oam:  Pointer { len: 0x00a0, ptr: 0xfe00, },
                hram: Pointer { len: 0x007f, ptr: 0xff80, },
                bgp:  Pointer { len: 0, ptr: 0 },
                obj:  Pointer { len: 0, ptr: 0 },
            },
        };
        Bess::builder()
            .name("bess")
            .info(Info::new(*b"BESS Testing Rom", 0xabcd))
            .core(core)
            .build([])
            .unwrap()
    }

    #[test]
    fn builder_works() {
        setup();
    }
}
