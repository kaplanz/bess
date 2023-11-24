use std::io::Read;
use std::mem;

use serde::Deserialize;

use super::Deserializer;
use crate::block::Header;
use crate::{Bess, Data, Error, Footer, Result};

pub trait Decode: Sized {
    fn decode(input: impl Read) -> Result<Self>;
}

impl Decode for Bess {
    fn decode(mut input: impl Read) -> Result<Self> {
        // Read the entire buffer
        let mut buf = Vec::new();
        input.read_to_end(&mut buf)?;
        // Decode the footer
        let end = {
            // Extract footer bytes
            let len = buf.len();
            let ftx = len - mem::size_of::<Footer>();
            let buf = buf.get(ftx..).ok_or(Error::TooShort)?;
            // Deserialize from bytes
            let mut de = Deserializer::from_bytes(buf);
            Footer::deserialize(&mut de)?
        };
        // Decode the context
        let ctx = buf
            .get(..end.start as usize)
            .ok_or(Error::TooShort)?
            .to_vec();
        // Decode the blocks
        let blx = {
            let vec = Vec::new();
            // Extract blocks bytes
            let len = buf.len();
            let ftx = len - mem::size_of::<Footer>();
            let buf = buf.get(end.start as usize..ftx).ok_or(Error::TooShort)?;
            // Deserialize from bytes
            let mut de = Deserializer::from_bytes(buf);
            while !de.input.is_empty() {
                // Read the head
                let head = Header::deserialize(&mut de)?;
                let blk = todo!();
                vec.push(blk);
            }
            vec
        };

        Ok(Bess { ctx, blx, end })
    }
}
