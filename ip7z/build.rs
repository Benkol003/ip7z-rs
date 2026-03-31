fn main() {
    assert!(
        cfg!(feature = "static") ^ cfg!(feature = "dynamic"),
        "exactly one of 'static' and 'dynamic' features can be enabled"
    );

        match std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap().as_str() {
            "windows" => {
                println!("cargo:rustc-link-lib=dylib=oleaut32"); 
            },
            "unix"  => {},
            t => panic!("unsupported target platform {}",t)
        }
    }