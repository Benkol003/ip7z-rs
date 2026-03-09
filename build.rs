fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    
    match target_os.as_str() {
        "windows" => {
            println!("cargo:rustc-link-lib=dylib=7z");
            println!("cargo:rustc-link-lib=dylib=advapi32");
        }
        "linux" => {
            println!("cargo:rustc-link-search=/usr/lib/7zip");
            println!("cargo:rustc-link-lib=dylib:+verbatim=7z.so");
            println!("cargo:rustc-link-arg=/usr/lib/7zip/7z.so");
            println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/7zip");
        }
        _ => {}
    }
}