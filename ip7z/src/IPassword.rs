use crate::{ffi::Z7IGroups, win_ffi::HRESULT};
use crate::win_ffi::BSTR;

use windows_core::{IUnknown, interface};

#[interface(Z7IGroups::IPassword.iface_iid(0x10))]
pub unsafe trait ICryptoGetTextPassword: IUnknown {

}

#[interface(Z7IGroups::IPassword.iface_iid(0x11))]
pub unsafe trait ICryptoGetTextPassword2: IUnknown {
    fn CryptoGetTextPassword2(&self, password_is_defined: *mut i32, password: *mut BSTR) -> HRESULT;
}