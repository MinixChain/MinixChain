use super::*;
use core::result;
use hex::FromHexError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MastError {
    /// Indicates whether the MAST build error
    MastBuildError,
    /// Mast generate merkle proof error
    MastGenProofError,
    /// Mast generate address error
    MastGenAddrError,
    /// Invalid constructed mast
    /// Example: When partial merkle tree contains no scripts
    InvalidMast(String),

    /// Bech32m encoding error
    EncodeToBech32Error(String),
    /// Format error of hex
    FromHexError(String),
    // Mainly used to handle io errors of encode
    IoError(String),
    /// Error which may occur while processing keypairs.
    KeyPairError(String),
}

impl From<io::Error> for MastError {
    fn from(err: io::Error) -> Self {
        MastError::IoError(err.to_string())
    }
}

impl From<FromHexError> for MastError {
    fn from(e: FromHexError) -> Self {
        match e {
            FromHexError::InvalidHexCharacter { c, index } => {
                MastError::FromHexError(format!("InvalidHexCharacter {}, {}", c, index))
            }
            FromHexError::OddLength => MastError::FromHexError("OddLength".to_owned()),
            FromHexError::InvalidStringLength => {
                MastError::FromHexError("InvalidStringLength".to_owned())
            }
        }
    }
}

impl From<hashes::hex::Error> for MastError {
    fn from(e: hashes::hex::Error) -> Self {
        match e {
            hashes::hex::Error::InvalidChar(c) => {
                MastError::FromHexError(format!("InvalidChar {}", c))
            }
            hashes::hex::Error::OddLengthString(c) => {
                MastError::FromHexError(format!("OddLengthString {}", c))
            }
            hashes::hex::Error::InvalidLength(a, b) => {
                MastError::FromHexError(format!("InvalidLength {},{}", a, b))
            }
        }
    }
}

impl From<schnorrkel::SignatureError> for MastError {
    fn from(e: schnorrkel::SignatureError) -> Self {
        MastError::KeyPairError(format!("SignatureError({:?})", e))
    }
}

pub type Result<T> = result::Result<T, MastError>;
