use super::*;

/// Encode an object into a vector
pub fn serialize<T: Encodable + ?Sized>(data: &T) -> Result<Vec<u8>, io::Error> {
    let mut encoder = Vec::new();
    let len = data.consensus_encode(&mut encoder)?;
    debug_assert_eq!(len, encoder.len());
    Ok(encoder)
}

macro_rules! define_le_to_array {
    ($name: ident, $type: ty, $byte_len: expr) => {
        #[inline]
        pub fn $name(val: $type) -> [u8; $byte_len] {
            debug_assert_eq!(::core::mem::size_of::<$type>(), $byte_len); // size_of isn't a constfn in 1.22
            let mut res = [0; $byte_len];
            for i in 0..$byte_len {
                res[i] = ((val >> i * 8) & 0xff) as u8;
            }
            res
        }
    };
}

define_le_to_array!(u16_to_array_le, u16, 2);
define_le_to_array!(u32_to_array_le, u32, 4);
define_le_to_array!(u64_to_array_le, u64, 8);

#[inline]
pub fn i16_to_array_le(val: i16) -> [u8; 2] {
    u16_to_array_le(val as u16)
}

#[inline]
pub fn i32_to_array_le(val: i32) -> [u8; 4] {
    u32_to_array_le(val as u32)
}

#[inline]
pub fn i64_to_array_le(val: i64) -> [u8; 8] {
    u64_to_array_le(val as u64)
}

/// Extensions of `Write` to encode data as per Bitcoin consensus
pub trait WriteExt {
    /// Output a 64-bit uint
    fn emit_u64(&mut self, v: u64) -> Result<(), io::Error>;
    /// Output a 32-bit uint
    fn emit_u32(&mut self, v: u32) -> Result<(), io::Error>;
    /// Output a 16-bit uint
    fn emit_u16(&mut self, v: u16) -> Result<(), io::Error>;
    /// Output a 8-bit uint
    fn emit_u8(&mut self, v: u8) -> Result<(), io::Error>;

    /// Output a 64-bit int
    fn emit_i64(&mut self, v: i64) -> Result<(), io::Error>;
    /// Output a 32-bit int
    fn emit_i32(&mut self, v: i32) -> Result<(), io::Error>;
    /// Output a 16-bit int
    fn emit_i16(&mut self, v: i16) -> Result<(), io::Error>;
    /// Output a 8-bit int
    fn emit_i8(&mut self, v: i8) -> Result<(), io::Error>;

    /// Output a boolean
    fn emit_bool(&mut self, v: bool) -> Result<(), io::Error>;

    /// Output a byte slice
    fn emit_slice(&mut self, v: &[u8]) -> Result<(), io::Error>;
}

macro_rules! encoder_fn {
    ($name:ident, $val_type:ty, $writefn:ident) => {
        #[inline]
        fn $name(&mut self, v: $val_type) -> Result<(), io::Error> {
            self.write_all(&$writefn(v))
        }
    };
}

impl<W: io::Write> WriteExt for W {
    encoder_fn!(emit_u64, u64, u64_to_array_le);
    encoder_fn!(emit_u32, u32, u32_to_array_le);
    encoder_fn!(emit_u16, u16, u16_to_array_le);
    encoder_fn!(emit_i64, i64, i64_to_array_le);
    encoder_fn!(emit_i32, i32, i32_to_array_le);
    encoder_fn!(emit_i16, i16, i16_to_array_le);

    #[inline]
    fn emit_i8(&mut self, v: i8) -> Result<(), io::Error> {
        self.write_all(&[v as u8])
    }
    #[inline]
    fn emit_u8(&mut self, v: u8) -> Result<(), io::Error> {
        self.write_all(&[v])
    }
    #[inline]
    fn emit_bool(&mut self, v: bool) -> Result<(), io::Error> {
        self.write_all(&[v as u8])
    }
    #[inline]
    fn emit_slice(&mut self, v: &[u8]) -> Result<(), io::Error> {
        self.write_all(v)
    }
}

/// Data which can be encoded in a consensus-consistent way
pub trait Encodable {
    /// Encode an object with a well-defined format.
    /// Returns the number of bytes written on success.
    ///
    /// The only errors returned are errors propagated from the writer.
    fn consensus_encode<W: io::Write>(&self, writer: W) -> Result<usize, io::Error>;
}

/// A variable-length unsigned integer
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct VarInt(pub u64);

// Primitive types
macro_rules! impl_int_encodable {
    ($ty:ident, $meth_enc:ident) => {
        impl Encodable for $ty {
            #[inline]
            fn consensus_encode<S: WriteExt>(&self, mut s: S) -> Result<usize, io::Error> {
                s.$meth_enc(*self)?;
                Ok(core::mem::size_of::<$ty>())
            }
        }
    };
}

impl_int_encodable!(u8, emit_u8);
impl_int_encodable!(u16, emit_u16);
impl_int_encodable!(u32, emit_u32);
impl_int_encodable!(u64, emit_u64);
impl_int_encodable!(i8, emit_i8);
impl_int_encodable!(i16, emit_i16);
impl_int_encodable!(i32, emit_i32);
impl_int_encodable!(i64, emit_i64);

impl VarInt {
    /// Gets the length of this VarInt when encoded.
    /// Returns 1 for 0..=0xFC, 3 for 0xFD..=(2^16-1), 5 for 0x10000..=(2^32-1),
    /// and 9 otherwise.
    #[allow(dead_code)]
    #[inline]
    pub fn len(&self) -> usize {
        match self.0 {
            0..=0xFC => 1,
            0xFD..=0xFFFF => 3,
            0x10000..=0xFFFFFFFF => 5,
            _ => 9,
        }
    }
}

impl Encodable for VarInt {
    #[inline]
    fn consensus_encode<S: io::Write>(&self, mut s: S) -> Result<usize, io::Error> {
        match self.0 {
            0..=0xFC => {
                (self.0 as u8).consensus_encode(s)?;
                Ok(1)
            }
            0xFD..=0xFFFF => {
                s.emit_u8(0xFD)?;
                (self.0 as u16).consensus_encode(s)?;
                Ok(3)
            }
            0x10000..=0xFFFFFFFF => {
                s.emit_u8(0xFE)?;
                (self.0 as u32).consensus_encode(s)?;
                Ok(5)
            }
            _ => {
                s.emit_u8(0xFF)?;
                (self.0 as u64).consensus_encode(s)?;
                Ok(9)
            }
        }
    }
}

#[allow(dead_code)]
fn consensus_encode_with_size<S: io::Write>(data: &[u8], mut s: S) -> Result<usize, io::Error> {
    let vi_len = VarInt(data.len() as u64).consensus_encode(&mut s)?;
    s.emit_slice(data)?;
    Ok(vi_len + data.len())
}

#[cfg(test)]
mod tests {
    use hashes::hex::ToHex;

    use super::*;
    #[test]
    fn test_ser_compact_size_tests() {
        let r1 = serialize(&VarInt(34_u64)).unwrap();
        let r2 = serialize(&VarInt(253_u64)).unwrap();
        let r3 = serialize(&VarInt(254_u64)).unwrap();
        let r4 = serialize(&VarInt(255_u64)).unwrap();
        let r5 = serialize(&VarInt(55555_u64)).unwrap();
        let r6 = serialize(&VarInt(666666_u64)).unwrap();
        let r7 = serialize(&VarInt(999999999_u64)).unwrap();
        let r8 = serialize(&VarInt(10000000000000_u64)).unwrap();

        assert_eq!(r1.to_hex(), "22");
        assert_eq!(r2.to_hex(), "fdfd00");
        assert_eq!(r3.to_hex(), "fdfe00");
        assert_eq!(r4.to_hex(), "fdff00");
        assert_eq!(r5.to_hex(), "fd03d9");
        assert_eq!(r6.to_hex(), "fe2a2c0a00");
        assert_eq!(r7.to_hex(), "feffc99a3b");
        assert_eq!(r8.to_hex(), "ff00a0724e18090000");
    }
}
