struct LZMAHandler {

}

#[repr[u32]]
enum kProps {
    kpidSize,
    kpidPackSize,
    kpidMethod
}
#[repr[u32]]
enum kArcProps {
    kpidNumStreams,
    kpidMethod
}

impl LZMAHandler {
    pub fn new() -> Self {
        
        LZMAHandler {

        }
    }
}