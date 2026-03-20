fn main() {
    assert!(
        cfg!(feature = "static") ^ cfg!(feature = "dynamic"),
        "only one of 'static' and 'dynamic' features can be enabled"
    );

    #[cfg(windows)]{
        println!("cargo:rustc-link-lib=dylib=advapi32");  //needed for SysAllocString / SysFreeString
    }
}