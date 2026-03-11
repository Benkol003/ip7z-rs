use core::panic;

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    match target_os.as_str() {
        "windows" => {
            println!("cargo:rustc-link-lib=dylib=7z");
            println!("cargo:rustc-link-lib=dylib=advapi32");
        }
        "linux" => {
            let candidates = ["/usr/lib/7zip/", "/usr/lib/p7zip/"];
            let path = candidates.iter().find(|p| std::path::Path::new(p).join("7z.so").exists())
                .expect("could not find 7z.so");
            println!("cargo:rustc-link-search={path}");
            println!("cargo:rustc-link-lib=dylib:+verbatim=7z.so");
            println!("cargo:rustc-link-arg=-Wl,-rpath,{path}");
        }
        _ => {
            panic!("unknown target os {}",target_os);
        }
    }
}