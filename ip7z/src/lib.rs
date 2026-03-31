#![doc = include_str!("../../README.md")]

#![allow(non_snake_case)]

pub mod ffi;
pub mod IStream;
pub mod ICoder;
pub mod IProgress;
pub mod IArchive;
pub mod IPassword;
pub mod IFolder;
pub mod propid;

pub mod win_ffi;

pub mod codecs;

#[cfg(test)]
mod tests;

use thiserror::Error;

use crate::win_ffi::{HRESULT, PROPVARIANTConversionError};
#[derive(Error,Debug)]
pub enum FFIError {
    #[error(transparent)]
    HResultError(#[from] HRESULT),
    #[error(transparent)]
    PropVariantConversionError(#[from] PROPVARIANTConversionError)
} 

pub type FFIResult<T> = Result<T,FFIError>;