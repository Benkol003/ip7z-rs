use std::{error::Error, ffi::c_void, path::PathBuf};

use com::Interface;
use com::sys::GUID;
use com::interfaces::IUnknown;

use crate::win_ffi::{PROPVARIANT,HRESULT, HrResult};

pub type PROPID = u32;

//7zip typedefs stuff e.g. ULONG to u32 on linux so it matches window's definitions for unsigned long and not 64-bit

#[allow(non_camel_case_types)]
pub type wchar = u16;

/*
given e.g. in c++ ISequentialInStream * const *inStreams:
the rust equivalent is *mut*const ISequentialInStreams,
however use *const*const ISequentialInStreams so we dont need to use mut in calling code,
and also there isnt a use case for this pointer to change
TODO do this across all pointer FFI defs
*/


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
            $(pub $name: fn( $( $arg: $arg_ty ),* ) -> $ret, )*
        }

        impl $struct_name {
            $( 
                $(#[$attr])*
                pub unsafe fn $name(&self, $( $arg: $arg_ty ),* ) -> $ret {
                    (self.$name)( $( $arg ),* )
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
        lib_static!($tt)
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



pub const fn handler_clsid(id: u8) -> GUID {
    //{23170F69-40C1-278A-1000-00 01 10 xx 00 00}
    GUID {data1: 0x23170F69, data2: 0x40C1, data3: 0x278A, data4: [0x10,0x00,0x00,0x01,0x10,id,0x00,0x00]}
}

impl Z7 {
    pub fn CreateInterface<I: Interface>(&self, clsid: GUID) -> HrResult<I> {
        unsafe {
            let mut object: *mut c_void = std::ptr::null_mut();
            let r: HRESULT = (self.CreateObject)(&clsid, &I::IID, &mut object);
            if r.succeeded() {
                Ok(std::ptr::read(&object as *const*mut c_void as *mut I)) //do i need read_unaligned?
            } else {
                Err(r)
            }
        }
    }

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

    #[cfg(all(feature = "static"))]
    pub fn new() -> Result<Self,Box<dyn Error>> {
        Self {
            CreateEncoder : CreateEncoder,
            CreateDecoder : CreateDecoder,
            CreateObject: CreateObject,
            GetHandlerProperty: GethandlerProperty,
            GetHandlerProperty2: GetHandlerProperty2,
            SetLargePageMode: SetLargePageMode
        }
    }

}

//for {23170F69-40C1-278A-0000-00yy00xx0000}
#[repr(u8)]
pub enum Z7IGroups {
    IProgress       = 0,
    IFolderArchive  = 1,
    IStream         = 3,
    ICoder          = 4,
    IPassword       = 5,
    IArchive        = 6,
    IFolder         = 8,
    IFolderManager  = 9
}

impl Z7IGroups {
    pub const fn iface_iid(self,id_x: u8) -> GUID {
        GUID { data1: 0x23170F69, data2: 0x40C1, data3: 0x278A, data4: [0x00,0x00,0x00,self as u8,0x00,id_x,0x00,0x00] }
    }
}

//{23170F69-40C1-278A-1000-000110xx0000}

#[repr(u8)]
pub enum Z7Formats {
   Zip      = 0x01,  
   BZip2    = 0x02,  
   Rar      = 0x03,  
   Arj      = 0x04,  
   Z        = 0x05,  
   Lzh      = 0x06,  
   Z7       = 0x07,  
   Cab      = 0x08,  
   Nsis     = 0x09,  
   Lzma     = 0x0A,  
   Lzma86   = 0x0B,      
   Xz       = 0x0C,      
   Ppmd     = 0x0D,      
   Zstd     = 0x0E,      
   LVM      = 0xBF,      
   AVB      = 0xC0,      
   LP       = 0xC1,      
   Sparse   = 0xC2,          
   APFS     = 0xC3,      
   Vhdx     = 0xC4,      
   Base64   = 0xC5,          
   COFF     = 0xC6,      
   Ext      = 0xC7,      
   VMDK     = 0xC8,      
   VDI      = 0xC9,      
   Qcow     = 0xCA,      
   GPT      = 0xCB,          
   Rar5     = 0xCC,      
   IHex     = 0xCD,          
   Hxs      = 0xCE,          
   TE       = 0xCF,      
   UEFIc    = 0xD0,              
   UEFIs    = 0xD1,          
   SquashFS = 0xD2,              
   CramFS   = 0xD3,              
   APM      = 0xD4,          
   Mslz     = 0xD5,              
   Flv      = 0xD6,          
   Swf      = 0xD7,          
   Swfc     = 0xD8,      
   Ntfs     = 0xD9,          
   Fat      = 0xDA,          
   Mbr      = 0xDB,          
   Vhd      = 0xDC,          
   Pe       = 0xDD,          
   Elf      = 0xDE,          
   MachO    = 0xDF,              
   Udf      = 0xE0,          
   Xar      = 0xE1,          
   Mub      = 0xE2,              
   Hfs      = 0xE3,          
   Dmg      = 0xE4,          
   Compound = 0xE5,              
   Wim      = 0xE6,          
   Iso      = 0xE7,                
   Chm      = 0xE9,          
   Split    = 0xEA,              
   Rpm      = 0xEB,              
   Deb      = 0xEC,          
   Cpio     = 0xED,          
   Tar      = 0xEE,          
   GZip     = 0xEF,          
}

impl Z7Formats {
    const fn format_clsid(group: u16, id: u8) -> GUID {
        //{23170F69-40C1-278A-1000-00 01 10 xx 00 00}
        GUID {data1: 0x23170F69, data2: 0x40C1, data3: group, data4: [0x10,0x00,0x00,0x01,0x10,id,0x00,0x00]}
    }
    
    pub const fn handler_clsid(self) -> GUID {
        Self::format_clsid(0x278A,self as u8)
    }
    pub const fn decoder_clsid(self) -> GUID {
        Self::format_clsid(0x2790,self as u8)
    }
    pub const fn encoder_clsid(self) -> GUID{
        Self::format_clsid(0x2791,self as u8)
    }
    pub const fn hasher_clsid(self) -> GUID {
        Self::format_clsid(0x2792,self as u8)
    }
}

com::interfaces!{

    
    /////////////// IHasher ///////////////
    #[uuid(Z7IGroups::ICoder.iface_iid(0xC0))]
    pub unsafe interface IHasher: IUnknown {
        fn Init(&self);
        fn Update(&self,data: *const c_void,size: u32);
        fn Final(&self, digest: u8);
        fn GetDigestSize(&self) -> u32;
    }

    #[uuid(Z7IGroups::ICoder.iface_iid(0xC1))]
    pub unsafe interface IHashers: IUnknown {
        fn GetNumHashers(&self) -> u32;
        fn GetHasherProp(&self,index: u32, propid: PROPID, value: *mut PROPVARIANT) -> HRESULT;
        fn CreateHasher(&self, index: u32, hasher: *mut IHasher) -> HRESULT;
    }
    ///////////////////////////////////////
}
