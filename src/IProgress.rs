use crate::ffi::Z7IGroups;
use com::sys::HRESULT;
use com::interfaces::IUnknown;

com::interfaces! {
    #[uuid(Z7IGroups::IProgress.iface_iid(0x5))]
    pub unsafe interface IProgress: IUnknown {
        pub fn SetTotal(&self, total: u64) -> HRESULT;
        pub fn SetCompleted(&self, complete_value: *const u64) -> HRESULT;
    }
}