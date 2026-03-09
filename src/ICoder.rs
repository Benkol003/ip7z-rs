use crate::ffi::{PROPID,Z7IGroups,wchar};
use std::cell::{Cell, RefCell};
use std::default;
use std::ffi::{c_ulong, c_void};
use bitflags::bitflags;
use com::AbiTransferable;
use com::sys::GUID;
use com::interfaces::IUnknown;
use crate::win_ffi::{BSTR, FILETIME, HRESULT, PROPVARIANT, VARTYPE};

use crate::IStream::*;

//TODO tett usage of NCoderPropID, NMethodPropID

//for coder props?
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum NCoderPropID {
    kDefaultProp = 0,
    kDictionarySize,    // VT_UI4
    kUsedMemorySize,    // VT_UI4
    kOrder,             // VT_UI4
    kBlockSize,         // VT_UI4 or VT_UI8
    kPosStateBits,      // VT_UI4
    kLitContextBits,    // VT_UI4
    kLitPosBits,        // VT_UI4
    kNumFastBytes,      // VT_UI4
    kMatchFinder,       // VT_BSTR
    kMatchFinderCycles, // VT_UI4
    kNumPasses,         // VT_UI4
    kAlgorithm,         // VT_UI4
    kNumThreads,        // VT_UI4
    kEndMarker,         // VT_BOOL
    kLevel,             // VT_UI4
    kReduceSize,        // VT_UI8 : it's estimated size of largest data stream that will be compressed
                        //   encoder can use this value to reduce dictionary size and allocate data buffers

    kExpectedDataSize,  // VT_UI8 : for ICompressSetCoderPropertiesOpt :
                        //   it's estimated size of current data stream
                        //   real data size can differ from that size
                        //   encoder can use this value to optimize encoder initialization

    kBlockSize2,        // VT_UI4 or VT_UI8
    kCheckSize,         // VT_UI4 : size of digest in bytes
    kFilter,            // VT_BSTR
    kMemUse,            // VT_UI8
    kAffinity,          // VT_UI8
    kBranchOffset,      // VT_UI4
    kHashBits,          // VT_UI4
    kNumThreadGroups,   // VT_UI4
    kThreadGroup,       // VT_UI4
    kAffinityInGroup,   // VT_UI8
}

unsafe impl AbiTransferable for NCoderPropID {
    type Abi = u32;
    fn get_abi(&self) -> Self::Abi {
        self.clone() as u32
    }
}


//for ICompressCodecsInfo::GetProperty??
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum NMethodPropID {
    kID,
    kName,
    kDecoder,
    kEncoder,
    kPackStreams,
    kUnpackStreams,
    kDescription,
    kDecoderIsAssigned,
    kEncoderIsAssigned,
    kDigestSize,
    kIsFilter 
}

//TODO make this derivable
unsafe impl AbiTransferable for NMethodPropID {
    type Abi = u32;
    fn get_abi(&self) -> Self::Abi {
        self.clone() as u32
    }
}

//pub enum NModuleInterfaceType

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum NModulePropID {
    kInterfaceType,   // VT_UI4
    kVersion          // VT_UI4    
}

com::interfaces! {
    #[uuid(Z7IGroups::ICoder.iface_iid(0x4))]
    pub unsafe interface ICompressProgressInfo: IUnknown {
        pub fn SetRatioInfo(&self, in_size: *const u64, out_size: *const u64) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x5))]
    pub unsafe interface ICompressCoder: IUnknown {
        pub fn Code(&self, 
            in_stream: *mut ISequentialInStream, 
            out_stream: *mut ISequentialOutStream,
            in_size: *const u64,
            out_size: *const u64,
            progress: *mut ICompressProgressInfo
        ) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x18))]
    pub unsafe interface ICompressCoder2: IUnknown {
        pub fn Code(&self, 
            in_streams: *const*mut ISequentialInStream,
            in_sizes: *const*const u64,
            num_in_streams: u32,
            out_streams: *const*mut ISequentialOutStream,
            out_sizes: *const*const u64,
            num_out_streams: u32
        ) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x1F))]
    pub unsafe interface ICompressSetCoderPropertiesOpt: IUnknown {
        pub fn SetCoderPropertiesOpt(&self, prop_ids: *const NCoderPropID, props: *const PROPVARIANT, num_props: u32) -> HRESULT;
    }

    //difference between this and CoderPropertiesOpt?
    #[uuid(Z7IGroups::ICoder.iface_iid(0x20))]
    pub unsafe interface ICompressSetCoderProperties: IUnknown {
        pub fn SetCoderProperties(&self, prop_ids: *const NCoderPropID, props: *const PROPVARIANT, num_props: u32) -> HRESULT;
    }

    //TODO what is the array...
    #[uuid(Z7IGroups::ICoder.iface_iid(0x22))]
    pub unsafe interface ICompressSetDecoderProperties2: IUnknown {
        pub fn SetDecoderProperties2(&self, data: *const u8, size: u32) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x23))]
    pub unsafe interface ICompressWriteCoderProperties: IUnknown {
        pub fn WriteCoderProperties(&self, out_stream: *mut ISequentialOutStream) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x24))]
    pub unsafe interface ICompressGetInStreamProcessedSize: IUnknown {
        pub fn GetInStreamProcessedSize(&self, value: *mut u64) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x25))]
    pub unsafe interface ICompressSetCoderMt: IUnknown {
        pub fn SetNumberOfThreads(&self, num_threads: u32) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x26))]
    pub unsafe interface ICompressSetFinishMode: IUnknown {
        pub fn SetFinishMode(&self, mode: u32) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x27))]
    pub unsafe interface ICompressGetInStreamProcessedSize2: IUnknown {
        pub fn GetInStreamProcessedSize2(&self, stream_index: u32, value: *mut u64) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x28))]
    pub unsafe interface ICompressSetMemLimit: IUnknown {
        pub fn SetMemLimit(&self, mem_usage: u64) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x29))]
    pub unsafe interface ICompressReadUnusedFromInBuf: IUnknown {
        pub fn ReadUnusedFromInBuf(&self, data: *mut c_void, size: u32, processed_size: *mut u32) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x30))]
    pub unsafe interface ICompressGetSubStreamSize: IUnknown {
        pub fn GetSubStreamSize(&self, sub_stream: u64, value: *mut u64) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x31))]
    pub unsafe interface ICompressSetInStream: IUnknown {
        pub fn SetOutStream(&self, in_stream: *mut ISequentialInStream) -> HRESULT;
        pub fn ReleaseInStream(&self) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x32))]
    pub unsafe interface ICompressSetOutStream: IUnknown {
        pub fn SetOutStream(&self, out_stream: *mut ISequentialOutStream) -> HRESULT;
        pub fn ReleaseOutStream(&self) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x34))]
    pub unsafe interface ICompressSetOutStreamSize: IUnknown {
        pub fn SetOutStreamSize(&self, out_size: *const u64) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x35))]
    pub unsafe interface ICompressSetBufSize: IUnknown {
        pub fn SetInBufSize(&self, stream_index: u32, size: u32) -> HRESULT;
        pub fn SetOutBufSize(&self, stream_index: u32, size: u32) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x36))]
    pub unsafe interface ICompressInitEncoder: IUnknown {
        pub fn InitEncoder(&self) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x37))]
    pub unsafe interface ICompressSetInStream2: IUnknown {
        pub fn SetInStream2(&self, stream_index: u32, in_stream: *mut ISequentialInStream) -> HRESULT;
        pub fn ReleaseInStream2(&self, stream_index: u32) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x40))]
    pub unsafe interface ICompressFilter: IUnknown {
        pub fn Init(&self) -> HRESULT;
        pub fn Filter(&self, data: *mut u8, size: u32) -> u32;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x60))]
    pub unsafe interface ICompressCodecsInfo: IUnknown {
        pub fn GetNumMethods(&self, num_methods: *mut u32) -> HRESULT;
        pub fn GetProperty(&self, index: u32, prop_id: NMethodPropID, value: *mut PROPVARIANT) -> HRESULT;
        pub fn CreateDecoder(&self, index: u32, iid: *const GUID, coder: *mut*mut c_void) -> HRESULT;
        pub fn CreateEncoder(&self, index: u32, iid: *const GUID, coder: *mut*mut c_void) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x61))]
    pub unsafe interface ISetCompressCodecsInfo: IUnknown {
        pub fn SetCompressCodecsInfo(&self, compress_codecs_info: *mut ICompressCodecsInfo) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x80))]
    pub unsafe interface ICryptoProperties: IUnknown {
        pub fn SetKey(&self, data: *const u8, size: u32) -> HRESULT;
        pub fn SetInitVector(&self, data: *const u8, size: u32) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x8C))]
    pub unsafe interface ICryptoResetInitVector: IUnknown {
        pub fn ResetInitVector(&self) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0x90))]
    pub unsafe interface ICryptoSetPassword: IUnknown {
        pub fn CryptoSetPassword(&self, data: *const u8, size: u32) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0xA0))]
    pub unsafe interface ICryptoSetCRC: IUnknown {
        pub fn CryptoSetCRC(&self, crc: u32) -> HRESULT;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0xC0))]
    pub unsafe interface IHasher: IUnknown {
        pub fn Init(&self);
        pub fn Update(&self, data: *const c_void, size: u32);
        pub fn Final(&self, digest: *mut u8);
        pub fn GetDigestSize(&self) -> u32;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0xC1))]
    pub unsafe interface IHashers: IUnknown {
        pub fn GetNumHashers(&self) -> u32;
        pub fn GetHasherProp(&self, index: u32, prop_id: PROPID, value: *mut PROPVARIANT) -> HRESULT;
        pub fn CreateHasher(&self, index: u32, hasher: *mut*mut IHasher) -> HRESULT;
    }
}

#[derive(Default)]
pub struct RatioInfo {
    pub in_size: u64,
    pub out_size: u64
}

com::class! {
    pub class CompressProgressInfo: ICompressProgressInfo {
        ratio_info: Cell<RatioInfo>
    }

    impl ICompressProgressInfo for CompressProgressInfo {
        pub fn SetRatioInfo(&self, in_size: *const u64, out_size: *const u64) -> HRESULT {
            unsafe {
                self.ratio_info.set(RatioInfo { in_size:*in_size, out_size: *out_size });
            }
            HRESULT::S_OK
        }
    }

}