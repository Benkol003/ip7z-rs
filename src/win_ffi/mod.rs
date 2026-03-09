//// needed types for FFI with 7zip, adapted from winsafe. 
/// winsafe crate cant be used directly due to extensive FFI to win32 libs, breaking linux compatibility.
mod hresult;
mod bstr;
mod propvariant;
mod filetime;

pub use bstr::*;
pub use hresult::*;
pub use propvariant::*;
pub use filetime::*;