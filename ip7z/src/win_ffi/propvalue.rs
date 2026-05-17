use crate::win_ffi::{FILETIME, HRESULT, PROPVARIANT, PROPVARIANTConversionError, VARTYPE};
use filetime::FileTime;

//rust wrapper to cleanly convert and validate PROPID + PROPVARIANT results from ffi
//see VariantType trait on PROPVARIANT for the reverse operation

//VT_INT / VT_UINT isnt used by 7zip in practice but is here for completeness.

pub enum PropValue {
    Empty,
    I16(i16),
    I32(i32),
    String(String),
    Error(HRESULT),
    Bool(bool),
    I8(i8),
    U8(u8),
    U16(u16),
    U32(u32),
    I64(i64),
    U64(u64),
    Int(std::ffi::c_int),
    UInt(std::ffi::c_uint),
    FileTime(FileTime),
}

impl PropValue {
    pub fn new<T>(vt: impl Into<VARTYPE>, pv: T) -> Result<PropValue, PROPVARIANTConversionError>
    where
        T: AsRef<PROPVARIANT>,
    {
        let vt = vt.into();
        if vt != pv.as_ref().vt {
            return Err(PROPVARIANTConversionError::new(pv.as_ref().vt, vt));
        }

        Ok(match vt {
            VARTYPE::VT_EMPTY => PropValue::Empty,
            VARTYPE::VT_I2 => PropValue::I16(i16::try_from(pv.as_ref())?),
            VARTYPE::VT_I4 => PropValue::I32(i32::try_from(pv.as_ref())?),
            VARTYPE::VT_BSTR => PropValue::String(String::try_from(pv.as_ref())?),
            VARTYPE::VT_ERROR => PropValue::Error(HRESULT::try_from(pv.as_ref())?),
            VARTYPE::VT_BOOL => PropValue::Bool(bool::try_from(pv.as_ref())?),
            VARTYPE::VT_I1 => PropValue::I8(i8::try_from(pv.as_ref())?),
            VARTYPE::VT_UI1 => PropValue::U8(u8::try_from(pv.as_ref())?),
            VARTYPE::VT_UI2 => PropValue::U16(u16::try_from(pv.as_ref())?),
            VARTYPE::VT_UI4 => PropValue::U32(u32::try_from(pv.as_ref())?),
            VARTYPE::VT_I8 => PropValue::I64(i64::try_from(pv.as_ref())?),
            VARTYPE::VT_UI8 => PropValue::U64(u64::try_from(pv.as_ref())?),
            VARTYPE::VT_INT => PropValue::Int(std::ffi::c_int::try_from(pv.as_ref())?),
            VARTYPE::VT_UINT => PropValue::UInt(std::ffi::c_uint::try_from(pv.as_ref())?),
            VARTYPE::VT_FILETIME => {
                const EPOCH_DIFF: u64 = 11_644_473_600;
                let win_ft = FILETIME::try_from(pv.as_ref())?.to_u64();
                const SEC_100_NS: u64 = 10_000_000;
                let unix_secs = (win_ft / SEC_100_NS).saturating_sub(EPOCH_DIFF) as i64;
                let nanosecs = ((win_ft % SEC_100_NS) * 100) as u32;
                PropValue::FileTime(FileTime::from_unix_time(unix_secs as i64, nanosecs))
            }
        })
    }
}
