
/// currently we copy the 7z source tree into out-dir, as makefiles will not work
/// if we set out dir to a path containing spaces, and will also fail to build 'all' target if we set a custom output dir
/// TODO if we just include source files here manually

fn main() {
    assert!(
        cfg!(feature = "static") ^ cfg!(feature = "dynamic"),
        "only one of 'static' and 'dynamic' features can be enabled"
    );

    #[cfg(all(feature = "static"))] {
        let z7_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("7zip");
        let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
        copy_folder(&z7_dir,&out_dir);
        
        match std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap().as_str() {
            "windows" => build_7z_msvc(z7_dir, out_dir),
            "unix"  => build_7z_unix(z7_dir, out_dir),
            t => panic!("unsupported target platform {}",t)
        }

    }
}

#[cfg(feature = "static")]
const Z7_BUNDLE: &str = "CPP/7zip/Bundles/Format7zF";  

fn copy_folder(src: impl AsRef<std::path::Path>,dest: impl AsRef<std::path::Path>) {
    std::fs::create_dir_all(&dest).unwrap();
    for entry in std::fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            copy_folder(entry.path(), dest.as_ref().join(entry.file_name()));
        } else {
            std::fs::copy(entry.path(),dest.as_ref().join(entry.file_name())).unwrap();
        }
    }
}

#[cfg(feature = "static")]
fn build_7z_unix(z7_dir: impl AsRef<std::path::Path>, out_dir: impl AsRef<std::path::Path>) {

    //TODO setting MY_ARCH / -march / -mtune

    let bundle_dir = out_dir.as_ref().join(Z7_BUNDLE);
    let build_dir = bundle_dir.join("_o");

    copy_folder(z7_dir,&out_dir);

    let cc = cc::Build::new().cpp(false).get_compiler();
    let cxx = cc::Build::new().cpp(true).get_compiler();
    let mut ar = cc::Build::new().get_archiver();

    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    //see 7zip/DOC/readme.txt, 7zip_gcc.mak
    let asm_args: &[&str] = match arch.as_str() {
        "x86_64" => &["IS_X64=1", "USE_ASM=1"],
        "x86" => &["IS_X86=1","USE_ASM=1"],
        "aarch64" => &["USE_ASM=1"], 
        _ => &["USE_ASM=0"],//7zip_gcc.mak doesnt seem to build Asm/arm/, atm there is only a asm crc routine anyway
    };

    //TODO mingw builds are currently broken 
    let mingw_arg: &[String] = match std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") && cxx.is_like_gnu() {
        true => {
            let cc_path = cc.path().to_str().unwrap();
            let windres = if let Some(idx) = cc_path.find("-gcc") {
                format!("RC={}-windres", &cc_path[..idx])
            } else {
                "RC=windres".to_string()
            };
            &["IS_MINGW=1".into(),windres,"CFLAGS=-loleaut32".into()]
        },
        false => &[]
    };
    

    let status = std::process::Command::new("make")
    .current_dir(&bundle_dir)
    .env("CC", cc.path())
    .env("CXX",cxx.path())
    .env("AR", ar.get_program())
    .arg(format!("MY_ASM=\"{}\"",uasm::UASM_PATH)) //TODO gate behind asm feature
    .args(asm_args)
    .arg("-f").arg("makefile.gcc")
    .arg("-j")
    .args(mingw_arg)
    .status().unwrap();
    if !status.success() {
        panic!("make failed with {}",status);
    }

    let objs: Vec<_> = std::fs::read_dir(&build_dir).unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path().file_name().unwrap().to_owned())
        .filter(|p| std::path::Path::new(p).extension().map(|e| e == "o").unwrap_or(false))
        .collect();

    let status = ar.current_dir(&build_dir).arg("rcs").arg("lib7z.a").args(objs).status().unwrap();
    if !status.success() {
        panic!("ar failed with {}",status);
    }


    println!("cargo:rustc-link-search={}",build_dir.display());
    println!("cargo:rustc-link-lib=static:+whole-archive=7z");
    println!("cargo:rustc-link-lib=stdc++"); //this should go to CPPFLAGS instead
}


#[cfg(feature = "static")]
fn build_7z_msvc(z7_dir: impl AsRef<std::path::Path>, out_dir: impl AsRef<std::path::Path>) {
    let tool = cc::Build::new().try_get_compiler().expect("failed to find compiler");
    let cl_path = tool.path();
    let nmake_path = cl_path.parent().unwrap().join("nmake.exe");
    let libtool_path = cl_path.parent().unwrap().join("lib.exe");

    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    println!("cargo:warning=target arch: {}",arch);
    let cl_arch = match arch.as_str() {
        "x86_64" => "x64",
        "x86" => "x86",
        "aarch64" => "arm64",
        "arm" => "arm",
        _ => panic!("unsupported architecture: {}", arch),
    };

    let bundle_dir = out_dir.as_ref().join(Z7_BUNDLE);
    let build_dir = bundle_dir.join(cl_arch);

    copy_folder(z7_dir,&out_dir);

    let status = std::process::Command::new(&nmake_path)
        .current_dir(&bundle_dir)
        .envs(tool.env().to_vec())
        .arg(format!("PLATFORM={}", cl_arch))
        .status().unwrap();
    if !status.success() {
        panic!("make failed with {}",status);
    }

    let objs: Vec<_> = std::fs::read_dir(&build_dir).unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path().file_name().unwrap().to_owned())
        .filter(|p| std::path::Path::new(p).extension().map(|e| e == "obj").unwrap_or(false))
        .collect();

    //TODO you need to link in objs in asm folder aswell
    let status = std::process::Command::new(&libtool_path)
        .current_dir(&build_dir) //using full path in /OUT may exceed path limit
        .arg("/OUT:7z_static.lib")
        .arg(format!("/MACHINE:{}",cl_arch))
        .args(objs)
        .status().unwrap();
    if !status.success() {
        panic!("cl failed with {}",status);
    }

    println!("cargo:rustc-link-search={}",build_dir.display());
    println!("cargo:rustc-link-lib=static:+whole-archive=7z_static");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=dylib=advapi32"); 
    println!("cargo:rustc-link-lib=dylib=oleaut32"); 
}