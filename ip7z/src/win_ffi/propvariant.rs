use std::{error::Error, mem::ManuallyDrop};
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

impl From<VT_BOOL> for bool {
    fn from(value: VT_BOOL) -> Self {
        match value {
            VT_BOOL::FALSE => false,
            VT_BOOL::TRUE => true
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
	pub cVal: i8,
	pub bVal: u8,
	pub iVal: i16,
	pub uiVal: u16, //also VARIANT_BOOL
	pub lVal: i32,
	pub ulVal: u32,
	pub hVal: i64,
	pub uhVal: u64,
	pub fltVal: f32,
	pub dblVal: f64,
    pub boolVal: VT_BOOL,
    pub scode: HRESULT,
    pub filetime: FILETIME,
    pub bstrVal: ManuallyDrop<BSTR>,
    pub ptr: *mut std::ffi::c_void, // for all pointer fields
    
    //unused
    //pub(crate) punkVal: ManuallyDrop<IUnknown>, //com::IUnknown stores vtable as a pointer internally
}

#[derive(Debug)]
pub struct PROPVARIANTConversionError {
    expected: VARTYPE, 
    got: VARTYPE,
}

impl std::fmt::Display for PROPVARIANTConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"unexpected PROPVARIANT type: expected {:?}, got {:?}",self.expected,self.got)
    }
}

impl Error for PROPVARIANTConversionError {}

impl PROPVARIANTConversionError {
    pub const fn new(expected: VARTYPE, got: VARTYPE) -> Self {
        PROPVARIANTConversionError { expected: expected, got: got }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Default,Clone, Copy)]
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

impl PROPVARIANT {
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

trait VariantType<T> {
    fn new(v: T) -> Self;
}

impl VariantType<i8> for PROPVARIANT {
    fn new(v: i8) -> Self {
        PROPVARIANT::from(VARTYPE::VT_I1, PROPVARIANT_union { cVal: v})
    }
}

impl TryFrom<PROPVARIANT> for i8 {
    type Error = PROPVARIANTConversionError;
    fn try_from(value: PROPVARIANT) -> Result<Self, Self::Error> {
        unsafe {
            if value.vt != VARTYPE::VT_I1 {
                return Err(PROPVARIANTConversionError::new(VARTYPE::VT_I1, value.vt));
            }
            Ok(value.data.cVal)
        }
    }
}

impl VariantType<i16> for PROPVARIANT {
    fn new(v: i16) -> Self {
        PROPVARIANT::from(VARTYPE::VT_I2, PROPVARIANT_union { iVal: v})
    }
}

impl TryFrom<PROPVARIANT> for i16 {
    type Error = PROPVARIANTConversionError;
    fn try_from(value: PROPVARIANT) -> Result<Self, Self::Error> {
        unsafe {
            if value.vt != VARTYPE::VT_I2 {
                return Err(PROPVARIANTConversionError::new(VARTYPE::VT_I2, value.vt));
            }
            Ok(value.data.iVal)
        }
    }
}

impl VariantType<i32> for PROPVARIANT {
    fn new(v: i32) -> Self {
        PROPVARIANT::from(VARTYPE::VT_I4, PROPVARIANT_union { lVal: v})
    }
}

impl TryFrom<PROPVARIANT> for i32 {
    type Error = PROPVARIANTConversionError;
    fn try_from(value: PROPVARIANT) -> Result<Self, Self::Error> {
        unsafe {
            if value.vt != VARTYPE::VT_I4 {
                return Err(PROPVARIANTConversionError::new(VARTYPE::VT_I4, value.vt));
            }
            Ok(value.data.lVal)
        }
    }
}

impl VariantType<i64> for PROPVARIANT {
    fn new(v: i64) -> Self {
        PROPVARIANT::from(VARTYPE::VT_I8, PROPVARIANT_union { hVal: v})
    }
}

impl TryFrom<PROPVARIANT> for i64 {
    type Error = PROPVARIANTConversionError;
    fn try_from(value: PROPVARIANT) -> Result<Self, Self::Error> {
        unsafe {
            if value.vt != VARTYPE::VT_I8 {
                return Err(PROPVARIANTConversionError::new(VARTYPE::VT_I8, value.vt));
            }
            Ok(value.data.hVal)
        }
    }
}

impl VariantType<u8> for PROPVARIANT {
    fn new(v: u8) -> Self {
        PROPVARIANT::from(VARTYPE::VT_UI1, PROPVARIANT_union { bVal: v})
    }
}

impl TryFrom<PROPVARIANT> for u8 {
    type Error = PROPVARIANTConversionError;
    fn try_from(value: PROPVARIANT) -> Result<Self, Self::Error> {
        unsafe {
            if value.vt != VARTYPE::VT_UI1 {
                return Err(PROPVARIANTConversionError::new(VARTYPE::VT_UI1, value.vt));
            }
            Ok(value.data.bVal)
        }
    }
}

impl VariantType<u16> for PROPVARIANT {
    fn new(v: u16) -> Self {
        PROPVARIANT::from(VARTYPE::VT_UI2, PROPVARIANT_union { uiVal: v})
    }
}

impl TryFrom<PROPVARIANT> for u16 {
    type Error = PROPVARIANTConversionError;
    fn try_from(value: PROPVARIANT) -> Result<Self, Self::Error> {
        unsafe {
            if value.vt != VARTYPE::VT_UI2 {
                return Err(PROPVARIANTConversionError::new(VARTYPE::VT_UI2, value.vt));
            }
            Ok(value.data.uiVal)
        }
    }
}

impl VariantType<u32> for PROPVARIANT {
    fn new(v: u32) -> Self {
        PROPVARIANT::from(VARTYPE::VT_UI4, PROPVARIANT_union { ulVal: v})
    }
}

impl TryFrom<PROPVARIANT> for u32 {
    type Error = PROPVARIANTConversionError;
    fn try_from(value: PROPVARIANT) -> Result<Self, Self::Error> {
        unsafe {
            if value.vt != VARTYPE::VT_UI4 {
                return Err(PROPVARIANTConversionError::new(VARTYPE::VT_UI4, value.vt));
            }
            Ok(value.data.ulVal)
        }
    }
}

impl VariantType<u64> for PROPVARIANT {
    fn new(v: u64) -> Self {
        PROPVARIANT::from(VARTYPE::VT_UI8, PROPVARIANT_union { uhVal: v})
    }
}

impl TryFrom<PROPVARIANT> for u64 {
    type Error = PROPVARIANTConversionError;

    fn try_from(value: PROPVARIANT) -> Result<Self, Self::Error> {
        unsafe {
            if value.vt != VARTYPE::VT_UI8 {
                return Err(PROPVARIANTConversionError::new(VARTYPE::VT_UI8, value.vt));
            }
            Ok(value.data.uhVal)
        }
    }
}

impl VariantType<BSTR> for PROPVARIANT {
    fn new(v: BSTR) -> Self {
        PROPVARIANT::from(VARTYPE::VT_BSTR, PROPVARIANT_union { bstrVal: ManuallyDrop::new(v)})
    }
}

impl TryFrom<PROPVARIANT> for BSTR {
    type Error = PROPVARIANTConversionError;

    fn try_from(mut value: PROPVARIANT) -> Result<Self, Self::Error> {
        unsafe {
            if value.vt != VARTYPE::VT_BSTR {
                return Err(PROPVARIANTConversionError::new(VARTYPE::VT_BSTR, value.vt));
            }
            let bstr = ManuallyDrop::take(&mut value.data.bstrVal);
            value.vt = VARTYPE::VT_EMPTY;
            Ok(bstr)
        }
    }
}

impl TryFrom<PROPVARIANT> for String {
    type Error = PROPVARIANTConversionError;
    fn try_from(value: PROPVARIANT) -> Result<Self, Self::Error> {
        Ok(BSTR::try_from(value)?.to_string())
    }
}

impl VariantType<bool> for PROPVARIANT {
    fn new(v: bool) -> Self {
        PROPVARIANT::from(VARTYPE::VT_BOOL, PROPVARIANT_union { boolVal: VT_BOOL::from(v)})
    }
}

impl TryFrom<PROPVARIANT> for bool {
    type Error = PROPVARIANTConversionError;

    fn try_from(value: PROPVARIANT) -> Result<Self, Self::Error> {
        unsafe {
            if value.vt != VARTYPE::VT_BOOL {
                return Err(PROPVARIANTConversionError::new(VARTYPE::VT_BOOL, value.vt));
            }
            Ok(value.data.boolVal.into())
        }
    }
}

impl VariantType<FILETIME> for PROPVARIANT {
    fn new(v: FILETIME) -> Self {
        PROPVARIANT::from(VARTYPE::VT_FILETIME, PROPVARIANT_union { filetime: v})
    }
}

impl TryFrom<PROPVARIANT> for FILETIME {
    type Error = PROPVARIANTConversionError;

    fn try_from(value: PROPVARIANT) -> Result<Self, Self::Error> {
        unsafe {
            if value.vt != VARTYPE::VT_FILETIME {
                return Err(PROPVARIANTConversionError::new(VARTYPE::VT_FILETIME, value.vt));
            }
            Ok(value.data.filetime)
        }
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
            VARTYPE::VT_BSTR => { unsafe { 
                ManuallyDrop::drop(&mut self.data.bstrVal)
            }}
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
