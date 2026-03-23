use std::cell::Cell;
use com::interfaces::IUnknown;
use crate::{ffi::Z7IGroups, win_ffi::HRESULT};

#[derive(Clone, Copy)]
pub struct ProgressStatus {
    completed: u64,
    total: u64
}

com::interfaces! {
    #[uuid(Z7IGroups::IProgress.iface_iid(0x5))]
    pub unsafe interface IProgress: IUnknown {
        pub fn SetTotal(&self, total: u64) -> HRESULT;
        pub fn SetCompleted(&self, complete_value: *const u64) -> HRESULT;
    }
}

com::class! {
    #[no_class_factory]
    pub class Progress: IProgress {
        status: Cell<ProgressStatus>
    }

    impl IProgress for Progress {
        pub fn SetTotal(&self, total: u64) -> HRESULT {
            let mut s = self.status.get();
            s.total = total;
            self.status.set(s);
            HRESULT::S_OK
        }

        pub fn SetCompleted(&self, complete_value: *const u64) -> HRESULT {
            let mut s = self.status.get();
            unsafe { s.completed = *complete_value; }
            self.status.set(s);
            HRESULT::S_OK
        }
    }
}