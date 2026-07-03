/// currently we copy the 7z source tree into out-dir, as makefiles will not work
/// if we set out dir to a path containing spaces, and will also fail to build 'all' target if we set a custom output dir
/// TODO if we just include source files here manually
/// 
/// TODO slow as hell in WSL

fn main() {
    assert!(
        cfg!(feature = "static") ^ cfg!(feature = "dynamic"),
        "only one of 'static' and 'dynamic' features can be enabled"
    );

    #[cfg(all(feature = "static"))] {
        let z7_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("7zip");
        let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
        copy_folder(&z7_dir,&out_dir);
        patch_z7(&out_dir);

        let target_family  = std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap();

        match std::env::var("CARGO_CFG_TARGET_ENV").unwrap().as_str() {
            "gnu" => {
                if target_family.as_str() == "unix" || target_family.as_str() == "windows" {
                    build_7z_unix(z7_dir, out_dir) //family is windows but env is gnu, is msys build
                }else {
                    panic!("unsupported target platform {} for env 'gnu'",target_family)
                }
            }
            "msvc" => {
                match target_family.as_str() {
                    "windows" => build_7z_msvc(z7_dir, out_dir),
                    t => panic!("unsupported target family {} for target env 'msvc'",t)
                }
            }
            t => panic!("unsupported target env {}",t)
        }

    }
}

#[cfg(feature = "static")]
const Z7_BUNDLE: &str = "CPP/7zip/Bundles/Format7zF";

fn copy_folder(src: impl AsRef<std::path::Path>,dest: impl AsRef<std::path::Path>) {
    let _ = std::fs::remove_dir_all(&dest);
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

fn link_win_libs() {
    println!("cargo:rustc-link-lib=dylib=user32");
    println!("cargo:rustc-link-lib=dylib=advapi32");
    println!("cargo:rustc-link-lib=dylib=uuid");
    println!("cargo:rustc-link-lib=dylib=oleaut32");
}

#[cfg(feature = "static")]
fn build_7z_unix(z7_dir: impl AsRef<std::path::Path>, out_dir: impl AsRef<std::path::Path>) {

    use std::path::PathBuf;
    use path_slash::PathExt as _;
    use path_slash::PathBufExt as _;
    use path_slash::CowExt as _;

    //TODO setting MY_ARCH / -march / -mtune
    //make sure to .replace("\\","/") on any paths, or will break in mingw

    let bundle_dir = out_dir.as_ref().join(Z7_BUNDLE);
    let bundle_dir = bundle_dir.to_slash().unwrap();
    let build_dir = out_dir.as_ref().join(Z7_BUNDLE).join("_o");
    let build_dir = build_dir.to_slash().unwrap();

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

    let is_mingw = std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") && cxx.is_like_gnu();

    //TODO mingw builds are currently broken
    let mingw_arg: &[String] = match is_mingw {
        true => {
            let cc_path = cc.path().to_str().unwrap();
            let windres = if let Some(idx) = cc_path.find("-gcc") {
                format!("RC={}-windres", &cc_path[..idx])
            } else {
                "RC=windres".to_string()
            };
            &["IS_MINGW=1".into(),windres]
        },
        false => &[]
    };

    let cc_path = cc.path().to_slash().unwrap();
    let cxx_path = cxx.path().to_slash().unwrap();
    let ar_path = PathBuf::from(ar.get_program());
    let ar_path = ar_path.to_slash().unwrap();
    let uasm_path = PathBuf::from(uasm::UASM_PATH);
    let uasm_path = uasm_path.to_slash().unwrap();

    println!("cargo:warning=CC:{}",cc.path().display());
    println!("cargo:warning=CXX:{}",cxx.path().display()); 
    println!("cargo:warning=AR:{}",ar.get_program().display());
    println!("cargo:warning=BUNDLE DIR:{}",&*bundle_dir);
    println!("cargo:warning=BUILD DIR:{}",&*build_dir);
    println!("cargo:warning=is mingw?{:?}",mingw_arg);
    println!("cargo:warning=uasm path: {}",&*uasm_path);

    let mut cmd = std::process::Command::new("make");
    cmd.current_dir(&*bundle_dir)

    //TODO why does this arg go missing if we put it at the end
    .arg("-f").arg("makefile.gcc")
    .arg("--output-sync=target")
    .arg("-j")
    .env("CC", &*cc_path)
    .env("CXX",&*cxx_path)
    .env("AR", &*ar_path)
    .arg(format!("MY_ASM=\"{}\"",uasm_path)) //TODO gate behind asm feature
    .args(asm_args)
    .args(mingw_arg);

    
    let status = cmd.status().unwrap();
    if !status.success() {
        panic!("make failed with {}",status);
    }

    let objs: Vec<_> = std::fs::read_dir(&*build_dir).unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path().file_name().unwrap().to_owned())
        .filter(|p| std::path::Path::new(p).extension().map(|e| e == "o").unwrap_or(false))
        .collect();

    let status = ar.current_dir(&*build_dir).arg("rcs").arg("lib7z.a").args(objs).status().unwrap();
    if !status.success() {
        panic!("ar failed with {}",status);
    }


    println!("cargo:rustc-link-search={}",build_dir);
    println!("cargo:rustc-link-lib=static:+whole-archive=7z");
    println!("cargo:rustc-link-lib=stdc++"); //this should go to CPPFLAGS instead
    if is_mingw {
        link_win_libs();
    }
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
    link_win_libs();
}

fn patch_z7(z7_dir: impl AsRef<std::path::Path>) {
    let patch_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("patches");

    //remove submodule .git file
    std::fs::remove_file(&z7_dir.as_ref().join(".git")).unwrap();
    let repo = git2::Repository::init(&z7_dir).unwrap();

    let mut config = repo.config().unwrap();
    config.set_str("core.autocrlf", "input").unwrap();

    repo.set_head("refs/heads/master").unwrap();
    let mut index = repo.index().unwrap();
    index.add_all(["."].iter(), git2::IndexAddOption::DEFAULT, None).unwrap();
    let tree_id = index.write_tree().unwrap();
    index.write().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    let sig = git2::Signature::now("git2", "git2").unwrap();
    let mut head = repo.find_commit(repo.commit(Some("refs/heads/master"),&sig,&sig,"git2: commit checkout",&tree,&[]).unwrap()).unwrap();

    let mut items: Vec<_> = std::fs::read_dir(patch_dir).unwrap().map(|e| e.unwrap()).collect();
    // //apply patch series in order
    items.sort_by_key(|e| e.path());
    for item in items {
        println!("cargo:info=applying patch {}",item.file_name().display());
        let mbox_patch = std::fs::read(item.path()).unwrap();

        //TODO if cant parse assume raw diff instead
        let message = mail_parser::MessageParser::new().parse(&mbox_patch).unwrap();
        let from = message.from().unwrap().clone().into_list();
        assert!(from.len()==1);
        let addr = from.get(0).unwrap();
        let date = message.date().unwrap();
        let tz_offset: i32 = (date.tz_hour as i32 * 60) + date.tz_minute as i32;
        let time = git2::Time::new(date.to_timestamp(),tz_offset);
        let sig = git2::Signature::new(&addr.name().unwrap().as_ref(),&addr.address().unwrap().as_ref(),&time).unwrap();

        //split upto diff contents
        let mut commit_msg = String::new();
        let mut diff_str = String::new();
        let mut found_diff: bool = false;
        for line in message.body_text(0).unwrap().lines() {
            if found_diff || line.starts_with("diff --git") {
                found_diff = true;
                diff_str.push_str(line);
                diff_str.push('\n');
            }else{
                commit_msg.push_str(line);
                commit_msg.push('\n');
            }
        }

        let diff = git2::Diff::from_buffer(diff_str.as_bytes()).unwrap();

        repo.apply(&diff, git2::ApplyLocation::WorkDir, None).unwrap();
        index.add_all(["."].iter(), git2::IndexAddOption::DEFAULT, None).unwrap();

        index.write().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();

        //let commit_msg = format!("git2",item.file_name().display());
        //TOOD use split out message
        //
        head = repo.find_commit(repo.commit(Some("HEAD"),&sig,&sig,&commit_msg,&tree,&[&head]).unwrap()).unwrap();
    }
    println!("cargo:info=out dir: {}",&z7_dir.as_ref().display());
}
