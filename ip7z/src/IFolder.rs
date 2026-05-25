use crate::{IProgress::{IProgress, IProgress_Impl}, IStream::IInStream, ffi::{PROPID,Z7IGroups, wchar}, win_ffi::{BSTR, VARTYPE}};
use std::ffi::c_int;
use windows_core::{IUnknown, InterfaceRef, interface};
use crate::win_ffi::{PROPVARIANT, FILETIME, HRESULT};

/*
see the following for depreciations - from guid.txt:
09 IFolder.h :: FOLDER_MANAGER_INTERFACE
  00 - 04 // old IFolderManager
  05 IFolderManager

*/

#[interface(Z7IGroups::IFolder.iface_iid(0x0))]
pub unsafe trait IFolderFolder: IUnknown {
    fn LoadItems(&self) -> HRESULT;
    fn GetNumberOfItems(&self, num_items: *mut u32) -> HRESULT;
    fn GetProperty(&self, item_index: u32, prop_id: PROPID, value: *mut PROPVARIANT) -> HRESULT;
    fn BindToFolder(&self,index: u32, name: *const wchar, result_folder: *mut IFolderFolder) -> HRESULT;
    fn BindToParentFolder(&self, result_folder: *mut IFolderFolder) -> HRESULT;
    fn GetNumberOfProperties(&self, num_props: *mut u32) -> HRESULT;
    fn GetPropertyInfo(&self, index: u32, name: *mut BSTR, prop_id: *mut PROPID, var_type: *mut VARTYPE) -> HRESULT;
    fn GetFolderProperty(&self, prop_id: PROPID, value: *mut PROPVARIANT) -> HRESULT;
}

#[interface(Z7IGroups::IFolder.iface_iid(0x4))]
pub unsafe trait IFolderWasChanged: IUnknown {
    fn WasChanged(&self, was_changed: *mut i32) -> HRESULT; //TODO bool
}

#[interface(Z7IGroups::IFolder.iface_iid(0x7))]
pub unsafe trait IFolderGetSystemIconIndex: IUnknown {
    pub fn GetSystemIconIndex(&self, index: u32, icon_index: *mut i32) -> HRESULT;
}

#[interface(Z7IGroups::IFolder.iface_iid(0x8))]
pub unsafe trait IFolderGetItemFullSize: IUnknown {
    fn GetItemFullSize(&self, index: u32, value: *mut PROPVARIANT, progress: InterfaceRef<IProgress>) -> HRESULT;
}

#[interface(Z7IGroups::IFolder.iface_iid(0x9))]
pub unsafe trait IFolderClone: IUnknown {
    fn Clone(&self, result_folder: *mut IFolderFolder) -> HRESULT;
}

#[interface(Z7IGroups::IFolder.iface_iid(0xA))]
pub unsafe trait IFolderSetFlatMode: IUnknown {
    fn SetFlatMode(&self, flat_mode: i32) -> HRESULT;
}


#[interface(Z7IGroups::IFolder.iface_iid(0x0B))]
pub unsafe trait IFolderOperationsExtractCallback: IProgress {
    fn AskWrite(&self, 
        src_path: *const wchar,
        src_is_folder: i32,
        src_time: *const FILETIME,
        src_size: *const u64,
        dest_path_request: *const wchar,
        dest_path_result: *mut BSTR,
        write_answer: *mut i32
    ) -> HRESULT;
    fn ShowMessage(&self, message: *const wchar) -> HRESULT;
    fn SetCurrentFilePath(&self, file_path: *const wchar) -> HRESULT;
    fn SetNumFiles(&self, num_files: u64) -> HRESULT;
}

#[interface(Z7IGroups::IFolder.iface_iid(0xE))]
pub unsafe trait IFolderProperties: IUnknown {
    fn GetNumberOfFolderProperties(&self, num_properties: *mut u32) -> HRESULT;
    fn GetFolderPropertyInfo(&self, index: u32, name: *mut BSTR, prop_id: *mut PROPID, var_type: *mut VARTYPE) -> HRESULT;
}

#[interface(Z7IGroups::IFolder.iface_iid(0x10))]
pub unsafe trait IFolderArcProps: IUnknown {
    fn GetArcNumLevels(&self, num_levels: *mut u32) -> HRESULT;
    fn GetArcProp(&self, level: u32, prop_id: PROPID, value: *mut PROPVARIANT) -> HRESULT;
    fn GetArcNumProps(&self, level: u32, num_props: *mut u32) -> HRESULT;
    fn GetArcPropInfo(&self, level: u32, index: u32, name: *mut BSTR, prop_id: *mut PROPID, var_type: *mut VARTYPE) -> HRESULT;
    fn GetArcProp2(&self, level: u32, num_props: *mut u32) -> HRESULT;
    fn GetArcPropInfo2(&self, level: u32, index: u32, name: *mut BSTR, prop_id: *mut PROPID, var_type: *mut VARTYPE) -> HRESULT;
}

//genuinely why tf does this exist...
#[interface(Z7IGroups::IFolder.iface_iid(0x11))]
pub unsafe trait IGetFolderArcProps: IUnknown {
    fn GetFolderArcProps(&self, object: *mut IGetFolderArcProps) -> HRESULT;
}

#[interface(Z7IGroups::IFolder.iface_iid(0x13))]
pub unsafe trait IFolderOperations: IUnknown {
    fn CreateFolder(&self, name: *const wchar, progress: InterfaceRef<IProgress>) -> HRESULT;
    fn CreateFile(&self, name: *const wchar, progress: InterfaceRef<IProgress>) -> HRESULT;
    fn Rename(&self, index: u32, new_name: *const wchar, progress: InterfaceRef<IProgress>) -> HRESULT;
    fn Delete(&self, indicies: *const u32, num_items: u32, progress: InterfaceRef<IProgress>) -> HRESULT;
    fn CopyTo(&self, 
        move_mode: i32, 
        indicies: *const u32, 
        num_items: u32, 
        include_alt_streams: i32, 
        replace_alt_stream_chars_mode: i32, 
        path: *const wchar, 
        callback: InterfaceRef<IFolderOperationsExtractCallback>) -> HRESULT;
    fn CopyFrom(&self, move_mode: i32, from_folder_path: *const wchar, items_paths: *const *const wchar, num_items: u32, progress: InterfaceRef<IProgress>) -> HRESULT;
    fn SetProperty(&self, index: u32, prop_id: PROPID, value: *const PROPVARIANT, progress: InterfaceRef<IProgress>) -> HRESULT;
    fn CopyFromFile(&self, index: u32, full_file_path: *const wchar, progress: InterfaceRef<IProgress>) -> HRESULT;
}

#[interface(Z7IGroups::IFolder.iface_iid(0x15))]
pub unsafe trait IFolderCompare: IUnknown {
    fn CompareItems(&self, index1: u32, index2: u32, prop_id: PROPID, prop_is_raw: i32) -> HRESULT;
}

#[interface(Z7IGroups::IFolder.iface_iid(0x16))]
pub unsafe trait IFolderGetItemName: IUnknown {
    fn GetItemName(&self, index: u32, name: *mut *const wchar, len: *mut c_int) -> HRESULT;
    fn GetItemPrefix(&self, index: u32, name: *mut *const wchar, len: *mut c_int) -> u64;
}

#[interface(Z7IGroups::IFolder.iface_iid(0x17))]
pub unsafe trait IFolderAltStreams: IUnknown {
    fn BindToAltStreamsIndexed(&self, index: u32, result_folder: *mut IFolderFolder) -> HRESULT;
    fn BindToAltStreamsNamed(&self, name: *const wchar, result_folder: *mut IFolderFolder) -> HRESULT;
    fn AreAltStreamsSupported(&self, index: u32, is_supported: *mut i32) -> HRESULT; //TODO bool
}

#[interface(Z7IGroups::IFolderManager.iface_iid(0x5))]
pub unsafe trait IFolderManager: IUnknown {
    fn OpenFolderFile(&self, 
        in_stream: InterfaceRef<IInStream>, 
        file_path: *const wchar, 
        arc_format: *const wchar, 
        result_folder: *mut IFolderFolder,
        progress: InterfaceRef<IProgress>
    ) -> HRESULT;
    fn GetExtensions(&self, extensions: *mut BSTR) -> HRESULT;
    fn GetIconPath(&self, ext: *const wchar, icon_path: *mut BSTR, icon_index: *mut i32) -> HRESULT;
}