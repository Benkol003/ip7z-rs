use std::mem::ManuallyDrop;


use crate::{ffi::{PROPID, wchar}, win_ffi::{BSTR, FILETIME, HRESULT}};

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct STATPROPSTG {
    lpwstrName: *mut wchar,
    propid: PROPID,
    vt: VARTYPE
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(i16)]
pub enum VT_BOOL {
    TRUE = -1,
    FALSE = 0
}

impl From<bool> for VT_BOOL {
    fn from(value: bool) -> Self {
        match value {
            true => VT_BOOL::TRUE,
            false => VT_BOOL::FALSE
        }
    }
}

#[repr(C)]
pub struct PROPVARIANT {
	pub vt: VARTYPE,
	wReserved1: u16,
	wReserved2: u16,
	wReserved3: u16,
	pub data: PROPVARIANT_union,
}

#[repr(C)]
pub union PROPVARIANT_union {
	pub(crate) cVal: i8,
	pub(crate) bVal: u8,
	pub(crate) iVal: i16,
	pub(crate) uiVal: u16, //also VARIANT_BOOL
	pub(crate) lVal: i32,
	pub(crate) ulVal: u32,
	pub(crate) hVal: i64,
	pub(crate) uhVal: u64,
	pub(crate) fltVal: f32,
	pub(crate) dblVal: f64,
    pub(crate) boolVal: VT_BOOL,
    pub(crate) scode: HRESULT,
    pub(crate) filetime: FILETIME,
    pub(crate) bstrVal: ManuallyDrop<BSTR>,
    pub(crate) ptr: *mut std::ffi::c_void, // for all pointer fields
    
    //unused
    //pub(crate) punkVal: ManuallyDrop<IUnknown>, //com::IUnknown stores vtable as a pointer internally
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Default,Clone, Copy)]
#[derive(num_enum::TryFromPrimitive,strum_macros::Display)]
#[repr(u16)]
pub enum VARTYPE {
    #[default]
    VT_EMPTY = 0,
    //VT_NULL = 1,
    VT_I2 = 2,
    VT_I4 = 3,
    //VT_R4 = 4,
    //VT_R8 = 5,
    //VT_CY = 6,
    //VT_DATE = 7,
    VT_BSTR = 8,
    //VT_DISPATCH = 9,
    VT_ERROR = 10,
    VT_BOOL = 11,
    //VT_VARIANT = 12, //unused by 7-zip
    //VT_UNKNOWN = 13, 
    //VT_DECIMAL = 14,

    VT_I1 = 16,
    VT_UI1 = 17,
    VT_UI2 = 18,
    VT_UI4 = 19,
    VT_I8 = 20,
    VT_UI8 = 21,
    VT_INT = 22,
    VT_UINT = 23,
    //VT_VOID = 24,
    //VT_HRESULT = 25, //similar to VT_ERROR but may be >=0, whereas VT_SCODE is expected to be <0?
    VT_FILETIME = 64
}

// impl From<u16> for VARTYPE {
//     fn from(value: u16) -> Self {
        
//     }
// }

// impl TryFrom<u16> for VARTYPE {
//     type Error = u16;
//     fn try_from(value: u16) -> Result<Self, Self::Error> {
//         let found = VARTYPE::iter().find(|x| {
//             (*x as u16) == value
//         });
//         found.ok_or(value)
//     }
// }

impl PROPVARIANT {
    unsafe fn generic_new<T>(vt: VARTYPE, v: T) -> Self {
        const {
            assert!(size_of::<T>() <= size_of::<u64>())
        }
        let mut data: u64 = 0;
        unsafe {
            (&v as *const T).copy_to(&mut data as *mut u64 as *mut T, 1);
            Self {
                vt: vt,
                wReserved1: 0,
                wReserved2: 0,
                wReserved3: 0,
                data: PROPVARIANT_union { uhVal: data }
            }
        }
    }

    fn from(vt: VARTYPE, v: PROPVARIANT_union) -> Self {
        Self {
            vt: vt,
            wReserved1: 0,
            wReserved2: 0,
            wReserved3: 0,
            data: v
        }  
    }
}

//this or 
/*
impl VariantType<i8> for PROPVARIANT {
    fn new(v: i8) -> Self {
        unsafe { PROPVARIANT::generic_new(VARTYPE::VT_I1, v) }
    }
}
*/

trait VariantType<T> {
    fn new(v: T) -> Self;
}

impl VariantType<i8> for PROPVARIANT {
    fn new(v: i8) -> Self {
        PROPVARIANT::from(VARTYPE::VT_I1, PROPVARIANT_union { cVal: v})
    }
}

impl VariantType<i16> for PROPVARIANT {
    fn new(v: i16) -> Self {
        PROPVARIANT::from(VARTYPE::VT_I2, PROPVARIANT_union { iVal: v})
    }
}

impl VariantType<i32> for PROPVARIANT {
    fn new(v: i32) -> Self {
        PROPVARIANT::from(VARTYPE::VT_I4, PROPVARIANT_union { lVal: v})
    }
}

impl VariantType<i64> for PROPVARIANT {
    fn new(v: i64) -> Self {
        PROPVARIANT::from(VARTYPE::VT_I8, PROPVARIANT_union { hVal: v})
    }
}

impl VariantType<u8> for PROPVARIANT {
    fn new(v: u8) -> Self {
        PROPVARIANT::from(VARTYPE::VT_UI1, PROPVARIANT_union { bVal: v})
    }
}

impl VariantType<u16> for PROPVARIANT {
    fn new(v: u16) -> Self {
        PROPVARIANT::from(VARTYPE::VT_UI2, PROPVARIANT_union { uiVal: v})
    }
}

impl VariantType<u32> for PROPVARIANT {
    fn new(v: u32) -> Self {
        PROPVARIANT::from(VARTYPE::VT_UI4, PROPVARIANT_union { ulVal: v})
    }
}

impl VariantType<u64> for PROPVARIANT {
    fn new(v: u64) -> Self {
        PROPVARIANT::from(VARTYPE::VT_UI8, PROPVARIANT_union { uhVal: v})
    }
}

impl VariantType<BSTR> for PROPVARIANT {
    fn new(v: BSTR) -> Self {
        PROPVARIANT::from(VARTYPE::VT_BSTR, PROPVARIANT_union { bstrVal: ManuallyDrop::new(v)})
    }
}

impl VariantType<bool> for PROPVARIANT {
    fn new(v: bool) -> Self {
        PROPVARIANT::from(VARTYPE::VT_BOOL, PROPVARIANT_union { boolVal: VT_BOOL::from(v)})
    }
}

impl VariantType<FILETIME> for PROPVARIANT {
    fn new(v: FILETIME) -> Self {
        PROPVARIANT::from(VARTYPE::VT_FILETIME, PROPVARIANT_union { filetime: v})
    }
}

// impl VariantType<IUnknown> for PROPVARIANT {
//     fn new(v: IUnknown) -> Self {
//         PROPVARIANT::from(VARTYPE::VT_UNKNOWN, PROPVARIANT_union { punkVal: ManuallyDrop::new(v)})
//     }
// }

impl Drop for PROPVARIANT {
	fn drop(&mut self) {
		match self.vt {
            VARTYPE::VT_EMPTY => {},
            VARTYPE::VT_BSTR => { unsafe { ManuallyDrop::drop(&mut self.data.bstrVal) }}
            //VARTYPE::VT_UNKNOWN => {unsafe { ManuallyDrop::drop(&mut self.data.punkVal)}}
            _ => {}
		}
	}
}

impl Default for PROPVARIANT {
    fn default() -> Self {
        Self {
            vt: VARTYPE::VT_EMPTY,
            wReserved1: 0,
            wReserved2: 0,
            wReserved3: 0,
            data: PROPVARIANT_union { uhVal: 0 }
        }
    }
}
