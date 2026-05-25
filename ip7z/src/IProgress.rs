use std::cell::Cell;
use crate::{ffi::Z7IGroups, win_ffi::HRESULT};

use windows_core::{interface, IUnknown, implement};

#[derive(Clone, Copy, Default)]
pub struct ProgressStatus {
    completed: u64,
    total: u64
}

#[interface(Z7IGroups::IProgress.iface_iid(0x5))]
pub unsafe trait IProgress: IUnknown {
    pub fn SetTotal(&self, total: u64) -> HRESULT;
    pub fn SetCompleted(&self, complete_value: *const u64) -> HRESULT;
}


#[derive(Default)]
#[implement(IProgress)]
pub struct Progress {
    status: Cell<ProgressStatus>
}

impl IProgress_Impl for Progress_Impl {
    unsafe fn SetTotal(&self, total: u64) -> HRESULT {
        let mut s = self.status.get();
        s.total = total;
        self.status.set(s);
        HRESULT::S_OK
    }

    unsafe fn SetCompleted(&self, complete_value: *const u64) -> HRESULT {
        let mut s = self.status.get();
        unsafe { s.completed = *complete_value; }
        self.status.set(s);
        HRESULT::S_OK
    }
}