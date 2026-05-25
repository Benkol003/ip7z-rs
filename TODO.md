be careful of impl functions that use out pointers, use std::ptr::write to assign to them, otherwise will try to call drop on uninitialised memory, maybe we should use MaybeUninit here?

7zip ffi interface pointers dont take ownership and call release. use windows_core::Interface_Ref for this (which is transparent over e c_void pointer to the underlying vtable) to avoid memory leaks.

Z7PropId's - map each property to a type and have a generalised get

be able to mark interfaces or functions deprecated  
do our class impls of interfaces need to be thread safe? 
support for p7zip - extra codecs? or has p7zip been deprecated 

check PROPVARIANT drop (PropVariantClear)

worth looking at https://github.com/rikyoz/bit7z for structuring and tests

mingw and bsd builds fail for various reasons

using HRESULT/HRRESULT for anything other than ffi isnt helpful e.g. given TYPE_E_MISMATCH, what type was given?


tracing:
add debug for PROPVARIANT union fields
add proper names for all instrument macros, otherwise for fn's in an impl is just the fn name, e.g.:
```
impl IArchiveExtractCallback for ArchiveExtractCallback {
#[instrument(skip(self),name = "IArchiveExtractCallback::GetStream")]
```
