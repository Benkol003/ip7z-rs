use num_enum::FromPrimitive;
use strum_macros::Display;

pub type HrResult<T> = Result<T,HRESULT>;

/// 7-zip uses the following subset of HRESULT error codes.
#[allow(non_camel_case_types)]

#[derive(Clone, Copy, Debug)]
#[derive(Display)]
#[derive(FromPrimitive)]
#[repr(u32)]
pub enum HRESULT {
    S_OK                        = 0x0,
    S_FALSE                     = 0x1,
    E_NOTIMPL                   = 0x80004001,
    E_NOINTERFACE               = 0x80004002,
    E_ABORT                     = 0x80004004,
    #[num_enum(default)]
    E_FAIL                      = 0x80004005,
    STG_E_INVALIDFUNCTION       = 0x80030001,
    CLASS_E_CLASSNOTAVAILABLE   = 0x80040111,
    E_OUTOFMEMORY               = 0x8007000E,
    E_INVALIDARG                = 0x80070057,
    TYPE_E_MISMATCH             = 0x80028CA0,

}

impl From<HRESULT> for i32 {
    fn from(value: HRESULT) -> Self {
        value as i32
    }
}

impl std::error::Error for HRESULT {}

impl HRESULT {

    pub fn code(self) -> i32 {
        i32::from(self)
    }

    pub fn succeeded(self) -> bool {
        return (self as i32) >=0;
    }

    pub fn failed(self) -> bool {
        return (self as i32) < 0;
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