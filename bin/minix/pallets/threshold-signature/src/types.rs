use schnorrkel::SignatureError;

use crate::mast::error::MastError;
use crate::{Config, Error};

impl<T: Config> From<MastError> for Error<T> {
    fn from(err: MastError) -> Self {
        match err {
            MastError::MastBuildError => Error::<T>::MastBuildError,
            MastError::InvalidMast(_) => Error::<T>::InvalidMast,
            MastError::MastGenProofError => Error::<T>::MastGenProofError,
            MastError::MastGenAddrError => Error::<T>::MastGenAddrError,
            MastError::FromHexError(_) => Error::<T>::InvalidEncoding,
            MastError::EncodeToBech32Error(_) => Error::<T>::InvalidEncoding,
            MastError::IoError(_) => Error::<T>::InvalidEncoding,
            MastError::KeyPairError(_) => Error::<T>::InvalidEncoding,
        }
    }
}

impl<T: Config> From<SignatureError> for Error<T> {
    fn from(_: SignatureError) -> Self {
        Error::<T>::InvalidSignature
    }
}
