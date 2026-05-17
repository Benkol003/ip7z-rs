//adapted from winsafe's implementation of BSTR.

use widestring::WideStr;

use crate::{
    ffi::wchar,
    win_ffi::{HRESULT, HrResult},
};
use std::alloc::Layout;

#[repr(transparent)]
pub struct BSTR(*mut wchar);

impl Drop for BSTR {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { SysFreeString(self.0) };
            self.0 = std::ptr::null_mut();
        }else {
            //should only be reached from BSTR::default()
        }
    }
}

impl Default for BSTR {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}

impl Clone for BSTR {
    fn clone(&self) -> Self {
        let ptr = unsafe { SysAllocStringLen(self.as_ptr(), self.len()) };
        if ptr.is_null() {
            panic!("BSTR::clone - SysAllocStringLen OOM");
        }
        Self(ptr)
    }
}

impl std::fmt::Display for BSTR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ws = WideStr::from_slice(self.as_slice());
        std::fmt::Display::fmt(&ws.display(), f)
    }
}
impl std::fmt::Debug for BSTR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BSTR: \"{}\"", self.to_string().escape_default())
    }
}

impl TryFrom<&str> for BSTR {
    type Error = HRESULT;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value.as_ref())
    }
}

impl From<&BSTR> for String {
    fn from(value: &BSTR) -> String {
        WideStr::from_slice(value.as_slice()).to_string_lossy()
    }
}

impl BSTR {
    fn real_ptr(&self) -> *const wchar {
        unsafe { self.0.byte_offset(- (size_of::<u32>() as isize)) }
    }
    //will not impl real_ptr_mut as always need to realloc the pointer to change the size

    /// bytes: number of bytes in the data string not including the null terminator (i.e. length from SysLenString())
    #[cfg(not(windows))]
    fn layout(bytes: u32) -> HrResult<Layout> {
        match Layout::from_size_align(bytes as usize + size_of::<wchar>() + size_of::<u32>(), size_of::<wchar>()) {
            Ok(l) => Ok(l),
            Err(_) => Err(HRESULT::E_OUTOFMEMORY),
        }
    }

    pub fn from_str(s: &str) -> HrResult<Self> {
        let wchars: Vec<wchar> = s.chars().map(|c| c as wchar).collect();
        if wchars.len() > u32::MAX as usize {
            return Err(HRESULT::E_INVALIDARG);
        }

        let ptr = unsafe { SysAllocStringLen(wchars.as_ptr(), wchars.len() as u32) };
        if ptr.is_null() {
            return Err(HRESULT::E_OUTOFMEMORY);
        }
        Ok(Self(ptr))
    }


    /// number of wchars in the BSTR, not including the null terminator.
    #[must_use]
    pub fn len(&self) -> u32 {
        unsafe {
            match self.0.is_null() {
                true => 0,
                false => (*self.real_ptr() as u32) / size_of::<wchar>() as u32,
            }
        }
    }

    /// Creates a new `BSTR` by wrapping a pointer.
    ///
    /// # Safety
    ///
    /// Be sure the pointer has the correct type and isn't owned by anyone else,
    /// otherwise you may cause memory access violations.
    #[must_use]
    pub const unsafe fn from_ptr(p: *mut wchar) -> Self {
        Self(p)
    }

    /// Returns the underlying
    /// [`LPWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
    /// pointer to the null-terminated wide string.
    #[must_use]
    pub const fn as_ptr(&self) -> *const wchar {
        self.0
    }

    // /// Returns a pointer to the underlying
    // /// [`LPWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
    // /// pointer to the null-terminated wide string.
    #[must_use]
    pub const fn as_mut_ptr(&mut self) -> *mut wchar {
        self.0
    }

    #[must_use]
    pub fn as_slice<'a>(&'a self) -> &'a[wchar] {
        unsafe {
            let len = self.len();
            if len == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(self.0, len as usize)
            }
        }
    }

    /// Ejects the underlying
    /// [`LPWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
    /// pointer leaving a null pointer in its place, so that
    /// [`SysFreeString`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysfreestring)
    /// won't be called.
    ///
    /// Be sure to free the pointer, otherwise, as the name of this method
    /// implies, you will cause a memory leak.
    #[must_use]
    pub const fn leak(&mut self) -> *mut wchar {
        std::mem::replace(&mut self.0, std::ptr::null_mut())
    }

    #[cfg(not(windows))]
    fn alloc(wchars: u32) -> HrResult<*mut wchar> {
        unsafe {
            let bytes: u32 = wchars * (size_of::<wchar>() as u32);
            let ptr = std::alloc::alloc(Self::layout(bytes)?);
            *(ptr as *mut u32) = bytes;
            Ok(ptr.byte_offset(size_of::<u32>() as isize) as *mut wchar)
        }
    }
}

#[cfg(not(windows))]
unsafe fn SysAllocStringLen(psz: *const wchar, len: u32) -> *mut wchar {
    match BSTR::alloc(len) {
        Err(_) => {
            return std::ptr::null_mut();
        }
        Ok(ptr) => unsafe {
            let src_slice = std::slice::from_raw_parts(psz, len as usize);
            let dest_slice = std::slice::from_raw_parts_mut(ptr, len as usize);
            dest_slice.copy_from_slice(src_slice);
            *ptr.add(len as usize) = 0;
            ptr
        },
    }
}

#[cfg(not(windows))]
unsafe fn SysFreeString(psz: *mut wchar) {
    unsafe {
        let real_ptr = psz.byte_offset(-(size_of::<u32>() as isize));
        std::alloc::dealloc(real_ptr as *mut u8, BSTR::layout(*real_ptr as u32).unwrap());
    }
}

//it looks like SysAllocString/SysFreeString dont use malloc/free, maybe using CoTaskMemAlloc
//for 7zip on windows we must then use SysFreeString for BSTR's passed across the ffi
#[cfg(windows)]
#[link(name = "OleAut32", kind = "dylib")]
unsafe extern "C" {
    fn SysAllocStringLen(psz: *const wchar, len: u32) -> *mut wchar;
    fn SysFreeString(psz: *const wchar);
}

#[test]
fn bstr_round_trip() {
    let str = "hello!";
    let bstr = BSTR::from_str(str).unwrap();
    assert!(bstr.len() == 6);
    assert!(bstr.len() as usize == str.len());
    let string2 = bstr.to_string();
    let str2 = string2.as_str();
    assert!(str == str2);
    drop(bstr);

    let empty = BSTR::default();
    assert!(empty.len()==0);
    drop(empty);
}
