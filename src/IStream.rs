use std::{cell::RefCell, fs::File, io::{Read, Seek, SeekFrom, Write}};
use com::ClassAllocation;
use com::interfaces::IUnknown;
use strum_macros::FromRepr;
use crate::{ffi::{PROPID,Z7IGroups}, win_ffi::HrResult};
use crate::win_ffi::{PROPVARIANT, FILETIME, HRESULT};

#[repr(C)]
struct StreamFileProps {
    size: u64,
    vo_iid: u64,
    file_id_low: u64,
    file_id_high: u64,
    num_links: u32,
    attributes: u32,
    created_time: FILETIME,
    access_time: FILETIME,
    modified_time: FILETIME
}

#[allow(non_camel_case_types)]
#[derive(FromRepr,Clone, Copy)]
#[repr(u32)]
pub enum STREAM_SEEK {
  SET = 0,
  CUR = 1,
  END = 2
}

impl TryFrom<u32> for STREAM_SEEK {
    type Error = HRESULT;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_repr(value).ok_or(HRESULT::E_INVALIDARG)
    }
}

impl STREAM_SEEK {
    pub fn with_offset(self, offset: i64) -> HrResult<SeekFrom> {
        match self {
            STREAM_SEEK::SET => {
                if offset < 0 {
                    Err(HRESULT::E_INVALIDARG)
                } else {
                    Ok(SeekFrom::Start(offset as u64))
                }
            },
            STREAM_SEEK::END => Ok(SeekFrom::End(offset)),
            STREAM_SEEK::CUR => Ok(SeekFrom::Current(offset))
        }
    }
}

com::interfaces! {
    #[uuid(Z7IGroups::IStream.iface_iid(0x1))]
    pub unsafe interface ISequentialInStream: IUnknown {
        pub fn Read(&self,data: *mut u8,size: u32, processed_size: *mut u32) -> HRESULT;
    }

    #[uuid(Z7IGroups::IStream.iface_iid(0x2))]
    pub unsafe interface ISequentialOutStream: IUnknown {
        pub fn Write(&self, data: *const u8,size: u32, processed_size: *mut u32) -> HRESULT;
    }

    #[uuid(Z7IGroups::IStream.iface_iid(0x3))]
    pub unsafe interface IInStream: ISequentialInStream {
        pub fn Seek(&self, offset: i64, seek_origin: u32, new_position: *mut u64) -> HRESULT;
    }

    #[uuid(Z7IGroups::IStream.iface_iid(0x4))]
    pub unsafe interface IOutStream: ISequentialOutStream {
        pub fn Seek(&self, offset: i64, seek_origin: u32, new_position: *mut u64) -> HRESULT;
        pub fn SetSize(&self, new_size: u64) -> HRESULT;
    }

    #[uuid(Z7IGroups::IStream.iface_iid(0x6))]
    pub unsafe interface IStreamGetSize: IUnknown {
        pub fn GetSize(&self, size: *mut u64) -> HRESULT;
    }

    #[uuid(Z7IGroups::IStream.iface_iid(0x7))]
    pub unsafe interface IOutStreamFinish: IUnknown {
        pub fn OutStreamFinish(&self) -> HRESULT;
    }

    // #[uuid(Z7IGroups::IStream.iface_guid(0x8))] //old version
    // pub unsafe interface IStreamGetProps: IUnknown {
    //     pub fn GetProps(&self,size: *mut u64, created_time: *mut FILETIME,access_time: *mut FILETIME, modified_time: *mut FILETIME, attributes: *mut u32) -> HRESULT;
    // }

    #[uuid(Z7IGroups::IStream.iface_iid(0x9))]
    pub unsafe interface IStreamGetProps: IUnknown {
        pub fn GetProps2(&self, props: *mut StreamFileProps) -> HRESULT;
    }
    
    #[uuid(Z7IGroups::IStream.iface_iid(0xa))]
    pub unsafe interface IStreamGetProp: IUnknown {
        pub fn GetProperty(&self, prop_id: PROPID, value: *mut PROPVARIANT) -> HRESULT;
        pub fn ReloadProps(&self) -> HRESULT;
    }

     #[uuid(Z7IGroups::IStream.iface_iid(0x10))]
     pub unsafe interface IStreamSetRestriction: IUnknown {
        pub fn SetRestriction(&self, begin: u64, end: u64) -> HRESULT;
     }
}

fn Seek(file: &RefCell<File>,offset: i64, seek_origin: u32, new_position: *mut u64) -> HRESULT {
    let mut file = file.borrow_mut();
    let seek = match STREAM_SEEK::try_from(seek_origin) {
        Err(e) => return e,
        Ok(v) => v
    };
    let seek_offset = match seek.with_offset(offset) {
        Err(e) => return e,
        Ok(v) => v
    };
    match file.seek(seek_offset) {
        Err(_e) => return HRESULT::E_FAIL,
        Ok(v) => {
            if !new_position.is_null() {
                unsafe { *new_position=v; }
            }
        }
    }
    HRESULT::S_OK
}


com::class!{

    #[no_class_factory]
    pub class FileInStream: IInStream(ISequentialInStream), ISequentialInStream {
        file: RefCell<std::fs::File>
    }

    impl ISequentialInStream for FileInStream {
        pub fn Read(&self,data: *mut u8,size: u32, processed_size: *mut u32) -> HRESULT {
            let mut file = self.file.borrow_mut();
            let buf: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(data,size as usize) };
            let r = file.read(buf);
            match r {
                Ok(s) => {
                    unsafe {*processed_size = s as u32};
                    HRESULT::S_OK
                }
                Err(_) => HRESULT::E_FAIL
            }
        }
    }

    impl IInStream for FileInStream {
        fn Seek(&self, offset: i64, seek_origin: u32, new_position: *mut u64) -> HRESULT {
            Seek(&self.file,offset,seek_origin,new_position)
        }
    }
}

impl FileInStream {
    pub fn new(path: &std::path::Path) -> std::io::Result<ClassAllocation<Self>> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .open(path)?;
        Ok(Self::allocate(RefCell::new(file)))
    }
}

com::class! {
    #[no_class_factory]
    pub class FileOutStream: IOutStream(ISequentialOutStream), ISequentialOutStream {
        file: RefCell<std::fs::File>
    }

    impl IOutStream for FileOutStream {
        fn Seek(&self, offset: i64, seek_origin: u32, new_position: *mut u64) -> HRESULT {
            Seek(&self.file,offset,seek_origin,new_position)
        }
        fn SetSize(&self, new_size: u64) -> HRESULT {
            match self.file.borrow_mut().set_len(new_size) {
                Err(_) => HRESULT::E_FAIL,
                Ok(()) => HRESULT::S_OK
            }
        }
    }

    impl ISequentialOutStream for FileOutStream {
        pub fn Write(&self, data: *const u8,size: u32, processed_size: *mut u32) -> HRESULT {
            let mut file = self.file.borrow_mut();
            let buf: &[u8] = unsafe { std::slice::from_raw_parts(data,size as usize) };
            let r = file.write(buf);
            match r {
                Ok(s) => {
                    if !processed_size.is_null() {
                        unsafe {*processed_size = s as u32};
                    }
                    HRESULT::S_OK
                }
                Err(_) => HRESULT::E_FAIL
            }
        }
    }
}