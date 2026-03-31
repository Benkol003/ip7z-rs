use num_enum::TryFromPrimitive;
use strum_macros::EnumIter;



#[derive(EnumIter,TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum Codecs {

    //branch coders
    BCJ         = 0x3030103,
    BCJ2        = 0x303011B,

    PPC         = 0x3030205,
    IA64        = 0x3030401,
    ARM         = 0x3030501,
    ARMT        = 0x3030701,
    ARM64       = 0xA,
    RISCV       = 0xB,
    SPARC       = 0x3030805,

    BZip2       = 0x40202,
    Copy        = 0,
    Delta       = 0x3,
    Deflate     = 0x40108,
    Deflate64   = 0x40109,

    LZMA        = 0x30101,
    LZMA2       = 0x21,

    PPMD        = 0x30401,

    //encoders not supported
    Rar1        = 0x40301,
    Rar2        = 0x40302,
    Rar3        = 0x40303,
    Rar5        = 0x40305,

    Swap2       = 0x20302,
    Swap4       = 0x20304,

    //cryto folder
    Z7AES       = 0x6F10701, //displayed as 7zAES for kName
    AES256CBC   = 0x6F00181,
}

impl Codecs {

    /// supports_decoder is not supplied as will always be true
    pub const fn supports_encoder(self) -> bool {
        match self {
            Self::Rar1 | Self::Rar2 | Self::Rar3 | Self::Rar5 => false,
            _ => true,
        }
    }
}

//TODO these also have GUIDs via kEncoder/kDecoder??