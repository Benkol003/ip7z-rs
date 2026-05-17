use std::cell::{Cell, RefCell};
use std::error::Error;
use std::path::{Path, PathBuf};

use com::Interface;
use ip7z::IArchive::{ArchiveExtractCallback, ArchiveOpenCallback, AskMode, HandlerPropID, IArchiveExtractCallback, IArchiveOpenCallback, IInArchive, OpenStatus};
use ip7z::ICoder::ICompressCodecsInfo;
use ip7z::IProgress::{IProgress, Progress};
use ip7z::IStream::{FileInStream, FileOutStream, IInStream};
use ip7z::ffi::{PROPID, Z7, Z7Formats};
use ip7z::propid;
use ip7z::win_ffi::{BSTR, HRESULT, HrResult, PROPVARIANT, VARTYPE};

#[test]
#[cfg_attr(miri, ignore)]
fn archive_fname() -> Result<(), Box<dyn Error>> {
    _archive_fname()
}

fn _archive_fname() -> Result<(), Box<dyn Error>>{
    tracing_subscriber::fmt()
    .with_writer(tracing_subscriber::fmt::TestWriter::new())
    .with_max_level(tracing::Level::TRACE)
    .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
    .init();

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

    //TODO GetArchiveProperty instead?
    //in_archive.GetProperty(0, HandlerPropID::kClassID, &mut value).ok()?;

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

    //TODO IArchive:new called a lot
    in_archive.clone().into_iter().for_each(|i| {
        let i = i.unwrap();
        //println!("{}",i.path.display());
    });

    println!("extract...");


    //extract all
    let to_extract: Vec<u32> = (0..nitems).collect();
    let progress = Progress::new();
    let ca_extract_callback = ArchiveExtractCallback::allocate(in_archive.clone(), progress,RefCell::new(None),PathBuf::from("./tmp"));
    let extract_callback = ca_extract_callback.query_interface::<IArchiveExtractCallback>().ok_or(HRESULT::E_NOINTERFACE)?; //TODO converting option -> HRESULT
    //we get STG_E_INVALIDFUNCTION...
    in_archive.Extract(to_extract.as_ptr(), to_extract.len() as u32, AskMode::Extract, extract_callback).ok()?; //TODO Error: HRESULT(-2147467259) not 0x8...

    println!("exiting main...");
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>>{
    _archive_fname()?;
    Ok(())
}
