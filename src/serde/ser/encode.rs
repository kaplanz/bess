use std::io::Write;

use crate::{Error, Result};

pub trait Encode {
    fn encode(&self, output: impl Write) -> Result<usize>;
}

impl Encode for bool {
    fn encode(&self, output: impl Write) -> Result<usize> {
        u8::from(*self).encode(output).map_err(Error::from)
    }
}

macro_rules! add_impl {
    ($($t:ty)*) => ($(
        impl Encode for $t {
            fn encode(&self, mut output: impl Write) -> Result<usize> {
                output.write(&self.to_le_bytes()).map_err(Error::from)
            }
        }
    )*)
}

add_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

impl Encode for &str {
    fn encode(&self, mut output: impl Write) -> Result<usize> {
        output.write(self.as_bytes()).map_err(Error::from)
    }
}

impl Encode for &[u8] {
    fn encode(&self, mut output: impl Write) -> Result<usize> {
        output.write(self).map_err(Error::from)
    }
}
