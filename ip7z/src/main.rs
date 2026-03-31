use std::cell::Cell;
use std::error::Error;
use std::path::PathBuf;

use com::Interface;
use ip7z::IArchive::{ArchiveOpenCallback, IArchiveOpenCallback, IInArchive, HandlerPropID, OpenStatus};
use ip7z::ICoder::ICompressCodecsInfo;
use ip7z::IStream::{FileInStream, IInStream};
use ip7z::ffi::{PROPID, Z7, Z7Formats};
use ip7z::propid;
use ip7z::win_ffi::{BSTR, HRESULT, HrResult, PROPVARIANT, VARTYPE};

#[test]
#[cfg_attr(miri, ignore)]
fn archive_fname() -> Result<(), Box<dyn Error>>{

    let bstr = BSTR::try_from("hello")?;
    println!("bstr len: {}",bstr.len());
    println!("{}",bstr);
    drop(bstr);

    let z7 = Z7::new()?;

    unsafe {
    let r = z7.CreateInterface::<IInArchive>(Z7Formats::Z7.handler_clsid());
    let in_archive = match r {
        Ok(a) => a,
        Err(e) => {
            println!("failed to create InArchive: {}",e.code());
            return Ok(());
        }
    };

    let mut value: PROPVARIANT = PROPVARIANT::default();
    in_archive.GetProperty(0, HandlerPropID::kClassID as PROPID, &mut value).ok()?;

    let fname = PathBuf::from("./tmp/@ace.7z");
    let in_fstream = FileInStream::new(&fname).unwrap();

    let mut nprop: u32 = u32::MAX;
    in_archive.GetNumberOfProperties(&mut nprop);
    for i in 0..nprop {
        let mut name = BSTR::default();
        let mut prop_id = PROPID::default();
        let mut var_type = VARTYPE::default();
        in_archive.GetPropertyInfo(i, &mut name, &mut prop_id, &mut var_type).ok()?;
    }

    let open_cbk = ArchiveOpenCallback::allocate(Cell::new(OpenStatus::default()));
    let max_check_start_pos = 0;
    in_archive.Open(
        in_fstream.query_interface::<IInStream>().ok_or(HRESULT::E_NOINTERFACE)?, 
        &max_check_start_pos, 
        open_cbk.query_interface::<IArchiveOpenCallback>().ok_or(HRESULT::E_NOINTERFACE)?
        ).ok()?;

    let mut nitems: u32 = 0;
    in_archive.GetNumberOfItems(&mut nitems).ok()?;
    assert!(nitems > 0);
    
    in_archive.into_iter().for_each(|i| {
        let i = i.unwrap();
        println!("{}",i.path.display());
    });

    println!("exiting main...");
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>>{
    //archive_fname()?;
    Ok(())
}
