use crate::ffi::{PROPID,Z7IGroups};
use std::cell::Cell;
use std::ffi::c_void;
use crate::win_ffi::{HRESULT, PROPVARIANT};
use windows_core::{GUID, IUnknown, InterfaceRef, implement, interface};

use crate::IStream::*;

//TODO tett usage of NCoderPropID, NMethodPropID

//for coder props?
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum CoderPropID {
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


//for ICompressCodecsInfo::GetProperty??
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum MethodPropID {
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

//pub enum NModuleInterfaceType

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum ModulePropID {
    kInterfaceType,   // VT_UI4
    kVersion          // VT_UI4
}

#[interface(Z7IGroups::ICoder.iface_iid(0x4))]
pub unsafe trait ICompressProgressInfo: IUnknown {
    pub fn SetRatioInfo(&self, in_size: *const u64, out_size: *const u64) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x5))]
pub unsafe trait ICompressCoder: IUnknown {
    pub fn Code(&self,
        in_stream: InterfaceRef<ISequentialInStream>,
        out_stream: InterfaceRef<ISequentialOutStream>,
        in_size: *const u64,
        out_size: *const u64,
        progress: InterfaceRef<ICompressProgressInfo>
    ) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x18))]
pub unsafe trait ICompressCoder2: IUnknown {
    pub fn Code(&self,
        in_streams: *const ISequentialInStream,
        in_sizes: *const*const u64,
        num_in_streams: u32,
        out_streams: *const ISequentialOutStream,
        out_sizes: *const*const u64,
        num_out_streams: u32
    ) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x1F))]
pub unsafe trait ICompressSetCoderPropertiesOpt: IUnknown {
    pub fn SetCoderPropertiesOpt(&self, prop_ids: *const CoderPropID, props: *const PROPVARIANT, num_props: u32) -> HRESULT;
}

//difference between this and CoderPropertiesOpt?
#[interface(Z7IGroups::ICoder.iface_iid(0x20))]
pub unsafe trait ICompressSetCoderProperties: IUnknown {
    pub fn SetCoderProperties(&self, prop_ids: *const CoderPropID, props: *const PROPVARIANT, num_props: u32) -> HRESULT;
}

//TODO what is the array...
#[interface(Z7IGroups::ICoder.iface_iid(0x22))]
pub unsafe trait ICompressSetDecoderProperties2: IUnknown {
    pub fn SetDecoderProperties2(&self, data: *const u8, size: u32) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x23))]
pub unsafe trait ICompressWriteCoderProperties: IUnknown {
    pub fn WriteCoderProperties(&self, out_stream: InterfaceRef<ISequentialOutStream>) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x24))]
pub unsafe trait ICompressGetInStreamProcessedSize: IUnknown {
    pub fn GetInStreamProcessedSize(&self, value: *mut u64) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x25))]
pub unsafe trait ICompressSetCoderMt: IUnknown {
    pub fn SetNumberOfThreads(&self, num_threads: u32) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x26))]
pub unsafe trait ICompressSetFinishMode: IUnknown {
    pub fn SetFinishMode(&self, mode: u32) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x27))]
pub unsafe trait ICompressGetInStreamProcessedSize2: IUnknown {
    pub fn GetInStreamProcessedSize2(&self, stream_index: u32, value: *mut u64) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x28))]
pub unsafe trait ICompressSetMemLimit: IUnknown {
    pub fn SetMemLimit(&self, mem_usage: u64) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x29))]
pub unsafe trait ICompressReadUnusedFromInBuf: IUnknown {
    pub fn ReadUnusedFromInBuf(&self, data: *mut u8, size: u32, processed_size: *mut u32) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x30))]
pub unsafe trait ICompressGetSubStreamSize: IUnknown {
    pub fn GetSubStreamSize(&self, sub_stream: u64, value: *mut u64) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x31))]
pub unsafe trait ICompressSetInStream: IUnknown {
    pub fn SetOutStream(&self, in_stream: InterfaceRef<ISequentialInStream>) -> HRESULT;
    pub fn ReleaseInStream(&self) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x32))]
pub unsafe trait ICompressSetOutStream: IUnknown {
    pub fn SetOutStream(&self, out_stream: InterfaceRef<ISequentialOutStream>) -> HRESULT;
    pub fn ReleaseOutStream(&self) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x34))]
pub unsafe trait ICompressSetOutStreamSize: IUnknown {
    pub fn SetOutStreamSize(&self, out_size: *const u64) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x35))]
pub unsafe trait ICompressSetBufSize: IUnknown {
    pub fn SetInBufSize(&self, stream_index: u32, size: u32) -> HRESULT;
    pub fn SetOutBufSize(&self, stream_index: u32, size: u32) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x36))]
pub unsafe trait ICompressInitEncoder: IUnknown {
    pub fn InitEncoder(&self) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x37))]
pub unsafe trait ICompressSetInStream2: IUnknown {
    pub fn SetInStream2(&self, stream_index: u32, in_stream: InterfaceRef<ISequentialInStream>) -> HRESULT;
    pub fn ReleaseInStream2(&self, stream_index: u32) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x40))]
pub unsafe trait ICompressFilter: IUnknown {
    pub fn Init(&self) -> HRESULT;
    pub fn Filter(&self, data: *mut u8, size: u32) -> u32;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x60))]
pub unsafe trait ICompressCodecsInfo: IUnknown {
    pub fn GetNumMethods(&self, num_methods: *mut u32) -> HRESULT;
    pub fn GetProperty(&self, index: u32, prop_id: MethodPropID, value: *mut PROPVARIANT) -> HRESULT;
    pub fn CreateDecoder(&self, index: u32, iid: *const GUID, coder: *mut*mut c_void) -> HRESULT;
    pub fn CreateEncoder(&self, index: u32, iid: *const GUID, coder: *mut*mut c_void) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x61))]
pub unsafe trait ISetCompressCodecsInfo: IUnknown {
    pub fn SetCompressCodecsInfo(&self, compress_codecs_info: InterfaceRef<ICompressCodecsInfo>) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x80))]
pub unsafe trait ICryptoProperties: IUnknown {
    pub fn SetKey(&self, data: *const u8, size: u32) -> HRESULT;
    pub fn SetInitVector(&self, data: *const u8, size: u32) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x8C))]
pub unsafe trait ICryptoResetInitVector: IUnknown {
    pub fn ResetInitVector(&self) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0x90))]
pub unsafe trait ICryptoSetPassword: IUnknown {
    pub fn CryptoSetPassword(&self, data: *const u8, size: u32) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0xA0))]
pub unsafe trait ICryptoSetCRC: IUnknown {
    pub fn CryptoSetCRC(&self, crc: u32) -> HRESULT;
}

#[interface(Z7IGroups::ICoder.iface_iid(0xC0))]
pub unsafe trait IHasher: IUnknown {
    pub fn Init(&self);
    pub fn Update(&self, data: *const u8, size: u32);
    pub fn Final(&self, digest: *mut u8);
    pub fn GetDigestSize(&self) -> u32;
}

#[interface(Z7IGroups::ICoder.iface_iid(0xC1))]
pub unsafe trait IHashers: IUnknown {
    pub fn GetNumHashers(&self) -> u32;
    pub fn GetHasherProp(&self, index: u32, prop_id: PROPID, value: *mut PROPVARIANT) -> HRESULT;
    pub fn CreateHasher(&self, index: u32, hasher: *mut IHasher) -> HRESULT;
}

#[derive(Default)]
pub struct RatioInfo {
    pub in_size: u64,
    pub out_size: u64
}

#[implement(ICompressProgressInfo)]
pub struct CompressProgressInfo {
    pub ratio_info: Cell<RatioInfo>
}

impl ICompressProgressInfo_Impl for CompressProgressInfo_Impl {
    unsafe fn SetRatioInfo(&self, in_size: *const u64, out_size: *const u64) -> HRESULT {
        unsafe {
            self.ratio_info.set(RatioInfo { in_size:*in_size, out_size: *out_size });
        }
        HRESULT::S_OK
    }
}
