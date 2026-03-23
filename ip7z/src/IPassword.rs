use com::sys::HRESULT;
use com::interfaces::IUnknown;
use crate::ffi::Z7IGroups;
use crate::win_ffi::BSTR;

com::interfaces! {

    #[uuid(Z7IGroups::IPassword.iface_iid(0x10))]
    pub unsafe interface ICryptoGetTextPassword: IUnknown {
        pub fn CryptoGetTextPassword(&self, password: *mut BSTR) -> HRESULT;
    }

    #[uuid(Z7IGroups::IPassword.iface_iid(0x11))]
    pub unsafe interface ICryptoGetTextPassword2: IUnknown {
        pub fn CryptoGetTextPassword2(&self, password_is_defined: *mut i32, password: *mut BSTR) -> HRESULT;
    }
}