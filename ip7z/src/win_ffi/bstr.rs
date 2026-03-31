//adapted from winsafe's implementation of BSTR.

use std::{alloc::Layout,str::EncodeUtf16};
use widestring::U16Str;
use crate::{ffi::wchar, win_ffi::{HRESULT,HrResult}};

/// A
/// [string data type](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/automat/bstr)
/// used with COM automation.
///
/// Automatically calls
/// [`SysFreeString`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysfreestring)
/// when the object goes out of scope.
#[repr(transparent)]
pub struct BSTR(*mut u16);

impl Drop for BSTR {
	fn drop(&mut self) {
		if !self.0.is_null() {
			self.free();
		}
	}
}

impl Default for BSTR {
	fn default() -> Self {
		Self(std::ptr::null_mut())
	}
}

impl std::fmt::Display for BSTR {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let slice = U16Str::from_slice(self.as_slice());
		std::fmt::Display::fmt(&slice.display(),f)
	}
}
impl std::fmt::Debug for BSTR {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "BSTR: \"{}\"", self)
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
        String::from_utf16_lossy(unsafe {
            std::slice::from_raw_parts(value.as_ptr(), value.len() as usize)
        })
    }
}

impl BSTR {

	fn real_ptr_mut(&mut self) -> *mut u16 {
		unsafe { self.0.byte_offset(-4) }
	}

	fn real_ptr(&self) -> *const u16 {
		unsafe { self.0.byte_offset(-4) }
	}

	/// bytes: number of bytes in the data string not including the null terminator (i.e. length from SysLenString())
	#[cfg(not(windows))]
	fn layout(bytes: u32) -> HrResult<Layout> {
		match Layout::from_size_align(bytes as usize + 2 + 4, 4) {
			Ok(l) => Ok(l),
			Err(_) => Err(HRESULT::E_OUTOFMEMORY)
		}
	}


	#[cfg(not(windows))]
	unsafe fn copy_from( se: EncodeUtf16, wchars: u32, ptr: *mut u16) {
		unsafe {
			let bstr_sz = ptr as *mut u32;
			*bstr_sz = (wchars * 2) as u32;
			let mut bstr = ptr.byte_offset(4) as *mut u16;
			se.for_each(|wc| {
				*bstr = wc;
				bstr = bstr.byte_add(2);
			});
			*bstr = 0; //null terminator
		}
	}

	#[cfg(not(windows))]
	fn free(&mut self) {
		unsafe {
			let ptr = self.real_ptr_mut();
			std::alloc::dealloc(ptr as *mut u8, Self::layout(*ptr as u32).unwrap());
			self.0 = std::ptr::null_mut();
		}
	}

	#[cfg(windows)]
	fn free(&mut self) {
		unsafe {
			SysFreeString(self.0);
			self.0 = std::ptr::null_mut();
		}
	}

	#[cfg(not(windows))]
	pub fn from_str(s: &str) -> HrResult<Self> {
		unsafe {
			let wchars = s.encode_utf16().count();
			let ptr = std::alloc::alloc(Self::layout((wchars*2) as u32)?) as *mut u16;
			if ptr.is_null() { 
				Err(HRESULT::E_OUTOFMEMORY) 
			} else {
				Self::copy_from(s.encode_utf16(), wchars as u32, ptr);
				Ok(Self(ptr.byte_offset(4))) 
			}
		}
	}

	#[cfg(windows)]
	pub fn from_str(s: &str) -> HrResult<Self> {
		let wchars: Vec<wchar> = s.encode_utf16().collect();
		if wchars.len() > u32::MAX as usize {
			return Err(HRESULT::E_INVALIDARG);
		}

		let ptr = unsafe {
			SysAllocStringLen(wchars.as_ptr(),wchars.len() as u32)
		};
		if ptr.is_null() {
			return Err(HRESULT::E_OUTOFMEMORY);
		}
		Ok(Self(ptr))
	}

	/// [`SysStringLen`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysstringlen)
	/// function.
	#[must_use]
	pub fn len(&self) -> u32 {
		unsafe {
			match self.0.is_null() {
				true => 0,
				false => {
					(*self.real_ptr() as u32) /2
				}
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
	pub const unsafe fn from_ptr(p: *mut u16) -> Self {
		Self(p)
	}

	/// Returns the underlying
	/// [`LPWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
	/// pointer to the null-terminated wide string.
	#[must_use]
	pub const fn as_ptr(&self) -> *mut u16 {
		self.0
	}

	/// Returns a pointer to the underlying
	/// [`LPWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
	/// pointer to the null-terminated wide string.
	#[must_use]
	pub const fn as_mut_ptr(&mut self) -> *mut *mut u16 {
		&mut self.0
	}

	/// Returns the underlying
	/// [`LPWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
	/// memory block as a null-terminated `u16` slice.
	#[must_use]
	pub fn as_slice(&self) -> &[u16] {
		unsafe { 
			let len = self.len();
			if len==0 {
				&[]
			} else {
				std::slice::from_raw_parts(self.0, self.len() as usize + 1)
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
	pub const fn leak(&mut self) -> *mut u16 {
		std::mem::replace(&mut self.0, std::ptr::null_mut())
	}
}

//it looks like SysAllocString/SysFreeString dont use malloc/free, maybe using CoTaskMemAlloc 
//for 7zip on windows we must then use SysFreeString for BSTR's passed across the ffi 
#[cfg(windows)]
#[link(name="OleAut32",kind="dylib")]
unsafe extern "C" {
	fn SysAllocStringLen(psz: *const wchar, len: u32) -> *mut u16;
	fn SysFreeString(psz: *const wchar);
}

#[test]
fn test_from_str() {
	let str =  "hello!";
	let bstr = BSTR::from_str(str).unwrap();
	assert!(bstr.len() as usize == str.len());
	drop(bstr);
}
