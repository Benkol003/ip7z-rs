use std::cell::Cell;

use crate::ffi::{PROPID,Z7IGroups,wchar};
use crate::win_ffi::{PROPVARIANT,FILETIME,BSTR,VARTYPE,HRESULT};

use bitflags::bitflags;
use com::interfaces::IUnknown;
use crate::IProgress::*;
use crate::IStream::*;

com::interfaces! {
    #[uuid(Z7IGroups::IArchive.iface_iid(0x10))]
    pub unsafe interface IArchiveOpenCallback: IUnknown {
        pub fn SetTotal(&self, files: *const u64, bytes: *const u64) -> HRESULT;
        pub fn SetCompleted(&self, files: *const u64, bytes: *const u64) -> HRESULT;
    }

    #[uuid(Z7IGroups::IArchive.iface_iid(0x20))]
    pub unsafe interface IArchiveExtractCallback: IProgress {
        pub fn GetStream(&self, index: u32, out_stream: *mut*mut ISequentialOutStream, ask_extract_mode: i32) -> HRESULT;
        pub fn PrepareOperation(&self, ask_extract_mode: i32) -> HRESULT;
        pub fn SetOperationResult(&self, op_res: i32) -> HRESULT;
    }

    #[uuid(Z7IGroups::IArchive.iface_iid(0x22))]
    pub unsafe interface IArchiveExtractCallbackMessage: IUnknown {
        pub fn ReportExtractResult(&self, index_type: u32, index: u32, op_res: i32) -> HRESULT;
    }

    #[uuid(Z7IGroups::IArchive.iface_iid(0x30))]
    pub unsafe interface IArchiveOpenVolumeCallback: IUnknown {
        pub fn GetProperty(&self,prop_id: PROPID, value: *mut PROPVARIANT) -> HRESULT;
        pub fn GetStream(&self, name: *const wchar, stream: *mut*mut IInStream) -> HRESULT;
    } 

    #[uuid(Z7IGroups::IArchive.iface_iid(0x40))]
    pub unsafe interface IInArchiveGetStream: IUnknown {
        pub fn GetStream(&self, index: u32, stream: *mut*mut IInStream) -> HRESULT;
    }

    #[uuid(Z7IGroups::IArchive.iface_iid(0x50))]
    pub unsafe interface IArchiveOpenSetSubArchiveName: IUnknown {
        pub fn SetSubArchiveName(&self, name: *const wchar) -> HRESULT;
    }

    #[uuid(Z7IGroups::IArchive.iface_iid(0x60))]
    pub unsafe interface IInArchive: IUnknown {
        pub fn Open(&self, stream: *mut IInStream, max_check_start_pos: *const u64, open_callback: *mut IArchiveOpenCallback) -> HRESULT;
        pub fn Close(&self) -> HRESULT;
        pub fn GetNumberOfItems(&self, num_items: *mut u32) -> HRESULT;
        pub fn GetProperty(&self, index: u32, propid: PROPID, value: *mut PROPVARIANT) -> HRESULT;
        pub fn Extract(&self, indicies: *const u32, num_items: u32, test_mode: i32, extract_callback: *mut IArchiveExtractCallback) -> HRESULT;
        pub fn GetArchiveProperty(&self, prop_id: PROPID, value: *mut PROPVARIANT) -> HRESULT;
        pub fn GetNumberOfProperties(&self, num_props: *mut u32) -> HRESULT;
        pub fn GetPropertyInfo(&self, index: u32, name: *mut BSTR, prop_id: *mut PROPID, var_type: *mut VARTYPE) -> HRESULT;
        pub fn GetNumberOfArchiveProperties(&self, num_props: *mut u32) -> HRESULT;
        pub fn GetArchivePropertyInfo(&self, index: u32, name: *mut BSTR, prop_id: *mut PROPID, var_type: *mut VARTYPE) -> HRESULT;
    }

    // #[uuid(Z7IGroups::IArchive.iface_guid(0x70))]
    // #[uuid(Z7IGroups::IArchive.iface_guid(0x71))]
    
    #[uuid(Z7IGroups::IArchive.iface_iid(0x61))]
    pub unsafe interface IArchiveOpenSeq: IUnknown {
        pub fn OpenSeq(&self, stream: *mut ISequentialInStream) -> HRESULT;
    }

    #[uuid(Z7IGroups::IArchive.iface_iid(0x80))]

    //is this deprecated for v2?
    pub unsafe interface IArchiveUpdateCallback: IProgress {
        pub fn GetUpdateItemInfo(&self,index: u32, new_data: *mut i32, new_props: *mut i32, index_in_archive: *mut u32) -> HRESULT;
        pub fn GetProperty(&self, index: u32, prop_id: PROPID,value: *mut PROPVARIANT) -> HRESULT;
        pub fn GetStream(&self, index: u32, in_stream: *mut*mut ISequentialInStream) -> HRESULT;
        pub fn SetOperationResult(&self, op_res: i32) -> HRESULT;
    }
    
    #[uuid(Z7IGroups::IArchive.iface_iid(0x82))]
    pub unsafe interface IArchiveUpdateCallback2: IProgress {
        pub fn GetVolumeSize(&self, index: u32, size: *mut u64) -> HRESULT;
        pub fn GetVolumeStream(&self, index: u32, volume_stream: *mut*mut ISequentialOutStream) -> HRESULT;
    }

    #[uuid(Z7IGroups::IArchive.iface_iid(0x83))]
    pub unsafe interface IArchiveUpdateCallbackFile: IArchiveUpdateCallback {
        pub fn GetStream2(&self, index: u32, in_stream: *mut*mut ISequentialInStream, notify_op: NUpdateNotifyOp) -> HRESULT;
        pub fn ReportOperation(&self, index_type: u32, index: u32, notify_op: NUpdateNotifyOp) -> HRESULT;
    }

    #[uuid(Z7IGroups::IArchive.iface_iid(0x84))]
    pub unsafe interface IArchiveGetDiskProperty: IUnknown {
        pub fn GetDiskProperty(&self, index: u32, prop_id: PROPID, value: *mut PROPVARIANT) -> HRESULT;
    }

    #[uuid(Z7IGroups::IArchive.iface_iid(0xA0))]
    pub unsafe interface IOutArchive: IUnknown {
        pub fn UpdateItems(&self, out_stream: *mut ISequentialOutStream, num_items: u32, update_callback: *mut IArchiveUpdateCallback) -> HRESULT;
        pub fn GetFileTimeType(&self, time_type: *mut NFileTimeType) -> HRESULT;
    }

    #[uuid(Z7IGroups::IArchive.iface_iid(0x03))]
    pub unsafe interface ISetProperties: IUnknown {
        pub fn SetProperties(&self, names: *const*const wchar, values: *const PROPVARIANT, num_props: u32) -> HRESULT;
    }

    #[uuid(Z7IGroups::IArchive.iface_iid(0x04))]
    pub unsafe interface IArchiveKeepModeForNextOpen: IUnknown {
        pub fn KeepModeForNextOpen(&self) -> HRESULT; 
    }

    #[uuid(Z7IGroups::IArchive.iface_iid(0x05))]
    pub unsafe interface IArchiveAllowTail: IUnknown {
        pub fn AllowTail(&self, allow_tail: i32) -> HRESULT;
    }

    #[uuid(Z7IGroups::IArchive.iface_iid(0x09))]
    pub unsafe interface IArchiveRequestMemoryUseCallback: IUnknown {
        pub fn RequestMemoryUse(&self, 
            flags: u32, 
            index_type: u32, 
            index: u32, 
            path: *const wchar,
            required_size: u64,
            allowed_size: *mut u64,
            answer_flags: *mut u32
        ) -> HRESULT;
    }

}


//TODO:
//cant use bitflags struct as not AbiTransferable
//allow use of typedef? or macro for reusing function defs

#[allow(non_snake_case)]
#[repr(u32)]
#[derive(Clone,Copy)]
pub enum NFileTimeType
{
KNotDefined = u32::MAX,
KWindows = 0,
KUnix,
KDOS,
K1ns
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum NArcInfoTimeFlags {
    kTime_Prec_Mask_bit_index = 0,
    kTime_Prec_Mask_num_bits = 26,

    kTime_Prec_Default_bit_index = 27,
    kTime_Prec_Default_num_bits = 5,
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum NHandlerPropID {
      kName = 0,        // VT_BSTR
      kClassID,         // binary GUID in VT_BSTR
      kExtension,       // VT_BSTR
      kAddExtension,    // VT_BSTR
      kUpdate,          // VT_BOOL
      kKeepName,        // VT_BOOL
      kSignature,       // binary in VT_BSTR
      kMultiSignature,  // binary in VT_BSTR
      kSignatureOffset, // VT_UI4
      kAltStreams,      // VT_BOOL
      kNtSecure,        // VT_BOOL
      kFlags,           // VT_UI4
      kTimeFlags        // VT_UI4
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum NAskMode {
    kExtract = 0,
    kTest,
    kSkip,
    kReadExternal   
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum NOperationResult {
    kOK = 0,
    kUnsupportedMethod,
    kDataError,
    kCRCError,
    kUnavailable,
    kUnexpectedEnd,
    kDataAfterEnd,
    kIsNotArc,
    kHeadersError,
    kWrongPassword
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum NEventIndexType {
    kNoIndex = 0,
    kInArcIndex,
    kBlockIndex,
    kOutArcIndex
}


#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum NUpdateNotifyOp {
    kAdd = 0,
    kUpdate,
    kAnalyze,
    kReplicate,
    kRepack,
    kSkip,
    kDelete,
    kHeader,
    kHashRead,
    kInFileChanged  
}

//TODO derive macro
unsafe impl com::AbiTransferable for NUpdateNotifyOp {
    type Abi = NUpdateNotifyOp;
    fn get_abi(&self) -> Self::Abi { unsafe { std::mem::transmute_copy(self) } }
}

//NUpdate::NOperationResult: kOK only

bitflags!{

    pub struct NPropDataType: u32 {
        const kMask_ZeroEnd   = 1 << 4;
        // const UInt32 kMask_BigEndian = 1 << 5;
        const kMask_Utf       = 1 << 6;
        const kMask_Utf8  = Self::kMask_Utf.bits() | 0;
        const kMask_Utf16 = Self::kMask_Utf.bits() | 1;
        // const UInt32 kMask_Utf32 = kMask_Utf | 2;

        const kNotDefined = 0;
        const kRaw = 1;

        const kUtf8z  = Self::kMask_Utf8.bits()  | Self::kMask_ZeroEnd.bits();
        const kUtf16z = Self::kMask_Utf16.bits() | Self::kMask_ZeroEnd.bits();    
    }

    //for IArchiveRequestMemoryUseCallback
    pub struct NRequestMemoryUseFlags: u32 {
        const k_AllowedSize_WasForced    = 1 << 0;  // (*allowedSize) was forced by -mmemx or -smemx
        const k_DefaultLimit_Exceeded    = 1 << 1;  // default limit of archive format was exceeded
        const k_MLimit_Exceeded          = 1 << 2;  // -mmemx value was exceeded
        const k_SLimit_Exceeded          = 1 << 3;  // -smemx value was exceeded
        
        const k_NoErrorMessage           = 1 << 10; // do not show error message, and show only request
        const k_IsReport                 = 1 << 11; // only report is required, without user request
        
        const k_SkipArc_IsExpected       = 1 << 12; // NRequestMemoryAnswerFlags::k_SkipArc flag answer is expected
        const k_Report_SkipArc           = 1 << 13; // report about SkipArc operation
    }

    //for IArchiveRequestMemoryUseCallback
    pub struct NRequestMemoryAnswerFlags: u32 {
        const k_Allow          = 1 << 0;  // allow further archive extraction
        const k_Stop           = 1 << 1;  // for exit (and return_code == E_ABORT is used)
        const k_SkipArc        = 1 << 2;  // skip current archive extraction

        const k_Limit_Exceeded  = 1 << 10;  // limit was exceeded
    }
}

#[derive(Default,Clone, Copy)]
pub struct OpenStatus {
    completed_bytes: u64,
    completed_files: u64,
    total_bytes: u64,
    total_files: u64
}

com::class! {
    pub class ArchiveOpenCallback: IArchiveOpenCallback {
        status: Cell<OpenStatus>
    }

    impl IArchiveOpenCallback for ArchiveOpenCallback {
        pub fn SetTotal(&self, files: *const u64, bytes: *const u64) -> HRESULT {
            unsafe {
                let mut status = self.status.get();
                if !bytes.is_null() {
                    status.total_bytes = *bytes;
                }
                if !files.is_null() {
                    status.total_files = *files;
                }
                self.status.set(status);
                HRESULT::S_OK
            }
        }
        pub fn SetCompleted(&self, files: *const u64, bytes: *const u64) -> HRESULT {
            unsafe {
                let mut status = self.status.get();
                if !bytes.is_null() {
                    status.completed_bytes = *bytes;
                }
                if !files.is_null() {
                    status.completed_files = *files;
                }
                self.status.set(status);
                HRESULT::S_OK
            }
        }
    }
}