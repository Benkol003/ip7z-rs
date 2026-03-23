#![allow(non_snake_case)]

use std::{error::Error, ffi::c_void, path::PathBuf};

pub type HRESULT = u32;

pub type PROPID = u32;

//7zip typedefs stuff e.g. ULONG to u32 on linux so it matches window's definitions for unsigned long and not 64-bit

#[allow(non_camel_case_types)]
pub type wchar = u16;

/// placeholder PROPVARIANT type for FFI. actual PROPVARIANT impl is in ip7z crate.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct PROPVARIANT([u8; 24]);

#[repr(C)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}


#[cfg(feature = "dynamic")]
macro_rules! lib_dynamic {
    ($struct_name:ident, $( $(#[$attr:meta])* fn $name:ident ( $($arg:ident : $arg_ty:ty),* ) -> $ret:ty );* $(;)?) => {

        pub struct $struct_name {
            _lib: libloading::Library,
            $( pub $name: unsafe extern "C" fn($($arg_ty),*) -> $ret, )*
        }

        impl $struct_name {
            $(
                $(#[$attr])*
                pub unsafe fn $name(&self, $($arg: $arg_ty),*) -> $ret {
                    unsafe { (self.$name)($($arg),*) }
                }
            )*

            fn load(lib: libloading::Library) -> Result<Self,libloading::Error> {
                unsafe {
                    Ok(Self {
                        $(
                            $name: *lib.get(stringify!($name))?,
                        )*
                        _lib: lib
                    })
                }
            }
        }
    }
}

#[cfg(feature = "static")]
macro_rules! lib_static {
    ($struct_name:ident, $( $(#[$attr:meta])* fn $name:ident ( $($arg:ident : $arg_ty:ty),* ) -> $ret:ty );* $(;)?) => {
        pub struct $struct_name {
            $(pub $name: unsafe extern "C" fn( $( $arg: $arg_ty ),* ) -> $ret, )*
        }

        impl $struct_name {
            $( 
                $(#[$attr])*
                pub unsafe fn $name(&self, $( $arg: $arg_ty ),* ) -> $ret {
                    unsafe { (self.$name)( $( $arg ),* ) }
                }
            )*
        }

        unsafe extern "C" {
            $(
                fn $name( $( $arg: $arg_ty ),* ) -> $ret;
            )*
        } 
    }
}

#[cfg(feature = "static")]
macro_rules! lib {
    ($($tt:tt)*) => {
        lib_static!{$($tt)*}
    }
}

#[cfg(feature = "dynamic")]
macro_rules! lib {
    ($($tt:tt)*) => {
        lib_dynamic!{$($tt)*}
    }
}

lib!(
    Z7,
    fn CreateDecoder(index: u32, iid: *const GUID, out_object: *mut *mut c_void) -> HRESULT;
    fn CreateEncoder(index: i32, iid: *const GUID, out_object: *mut *mut c_void) -> HRESULT;
    fn CreateObject(clsid: *const GUID, iid: *const GUID, out_object: *mut *mut c_void) -> HRESULT;
    #[deprecated]
    fn GetHandlerProperty(propid: PROPID, value: *mut PROPVARIANT) -> HRESULT;
    fn GetHandlerProperty2(index: u32, propid: PROPID, value: *mut PROPVARIANT) -> HRESULT;
    fn SetLargePageMode() -> HRESULT;
);


impl Z7 {
    #[cfg(all(feature = "dynamic",target_os = "windows"))]
    fn find_7z() -> Result<PathBuf, Box<dyn Error>> {
        use winreg::{RegKey, enums::HKEY_LOCAL_MACHINE};
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let z7reg = hklm.open_subkey("SOFTWARE\\7-zip").unwrap();
        let path = PathBuf::from(z7reg.get_value::<String,_>("path")?).join("7z.dll");
        if !path.exists() {
            return Err(format!("7zip is installed but could not find 7z.dll at {}",path.display()).into());
        }
        Ok(path)
    }

    #[cfg(all(feature = "dynamic",any(target_os = "linux", target_os = "macos")))]
    fn find_7z() -> Result<PathBuf,Box<dyn Error>>{
        #[cfg(target_os = "linux")]
        let candidates = [
            "/usr/lib/7zip/", //arch: 7zip
            "/usr/libexec/7zip/", //fedora: 7zip
            "/usr/lib/p7zip/", //ubuntu/debian: p7zip-full
            ];

        #[cfg(target_os = "linux")]
        let errmsg ="could not find 7z.so, please install 7zip or p7zip via your package manager e.g.:\n\
            - sudo apt install p7zip-full\n\
            - sudo dnf install 7zip\n\
            - sudo pacman -S 7zip";

        #[cfg(target_os = "macos")]
        let candidates = ["/usr/local/lib/"]; //brew install sevenzip

        #[cfg(target_os = "macos")]
        let errmsg  = "could not find 7z.so, please install 7zip:\n\
        - brew install sevenzip";


        let path = candidates.iter()
            .map(|p| std::path::Path::new(p).join("7z.so"))
            .find(|p| p.exists())
            .ok_or(errmsg)?;

        Ok(PathBuf::from(path))
    }

    #[cfg(feature = "dynamic")]
    pub fn new() -> Result<Self,Box<dyn Error>> {
        unsafe {
            let lib = libloading::Library::new(Self::find_7z()?)?;
            let sself = Self::load(lib)?;
            sself.SetLargePageMode();
            Ok(sself)
        }
    }

    #[cfg(feature = "static")]
    pub fn new() -> Result<Self,Box<dyn Error>> {
        Ok(Self {
            CreateEncoder : CreateEncoder,
            CreateDecoder : CreateDecoder,
            CreateObject: CreateObject,
            GetHandlerProperty: GetHandlerProperty,
            GetHandlerProperty2: GetHandlerProperty2,
            SetLargePageMode: SetLargePageMode
        })
    }

}