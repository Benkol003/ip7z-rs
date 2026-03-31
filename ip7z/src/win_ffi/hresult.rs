use std::fmt;

pub type HrResult<T> = Result<T,HRESULT>;

/// 7-zip uses the following subset of HRESULT error codes.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct HRESULT(i32);

macro_rules! derive_codes {
    ($struct_name: ident, $($name:ident = $val:expr ,)*) => {
        impl $struct_name {
            $(
                pub const $name: Self = Self::from_u32($val);
            )*
        }

        impl fmt::Display for $struct_name {

            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let name_str: &str = match *self {
                    $(
                        Self::$name => stringify!($name),
                    )*
                    _ => "(Unknown HRESULT)",
                };
                write!(f,"{}: 0x{}",name_str,self.0 as u32)
            }
        }
    }
}

derive_codes!(HRESULT,
    S_OK                        = 0x0,
    S_FALSE                     = 0x1,
    E_NOTIMPL                   = 0x80004001,
    E_NOINTERFACE               = 0x80004002,
    E_ABORT                     = 0x80004004,
    E_FAIL                      = 0x80004005,
    STG_E_INVALIDFUNCTION       = 0x80030001,
    CLASS_E_CLASSNOTAVAILABLE   = 0x80040111,
    E_OUTOFMEMORY               = 0x8007000E,
    E_INVALIDARG                = 0x80070057,
    TYPE_E_MISMATCH             = 0x80028CA0,

);

impl HRESULT {
    pub const fn from_i32(value: i32) -> HRESULT {
            Self(value)
    }

    pub const fn from_u32(value: u32) -> HRESULT {
            Self::from_i32(value as i32)
    }
}

impl From<HRESULT> for i32 {
    fn from(value: HRESULT) -> Self {
        value.0
    }
}

impl From<i32> for HRESULT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl std::error::Error for HRESULT {}

impl HRESULT {

    pub fn code(self) -> i32 {
        i32::from(self)
    }

    pub fn succeeded(self) -> bool {
        self.0 >=0
    }

    pub fn failed(self) -> bool {
        self.0 < 0
    }

    pub fn ok(self) -> HrResult<()>{
        match self.succeeded() {
            true => {
                HrResult::Ok(())
            }
            false => {
                HrResult::Err(self)
            }
        }
    }
}