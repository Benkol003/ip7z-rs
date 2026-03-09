use crate::{IProgress::IProgress, IStream::IInStream, ffi::{PROPID,Z7IGroups, wchar}, win_ffi::{BSTR, VARTYPE}};
use std::{cell::RefCell, ffi::{c_ulong, c_void, c_int}, io::{Read,Write}};
use com::{ClassAllocation, sys::GUID};
use com::interfaces::IUnknown;
use crate::win_ffi::{PROPVARIANT, FILETIME, HRESULT};

/*
see the following for depreciations - from guid.txt:
09 IFolder.h :: FOLDER_MANAGER_INTERFACE
  00 - 04 // old IFolderManager
  05 IFolderManager

*/

com::interfaces! {
    #[uuid(Z7IGroups::IFolder.iface_iid(0x0))]
    pub unsafe interface IFolderFolder: IUnknown {
        fn LoadItems(&self) -> HRESULT;
        fn GetNumberOfItems(&self, num_items: *mut u32) -> HRESULT;
        fn GetProperty(&self, item_index: u32, prop_id: PROPID, value: *mut PROPVARIANT) -> HRESULT;
        fn BindToFolder(&self,index: u32, name: *const wchar, result_folder: *mut *mut IFolderFolder) -> HRESULT;
        fn BindToParentFolder(&self, result_folder: *mut *mut IFolderFolder) -> HRESULT;
        fn GetNumberOfProperties(&self, num_props: *mut u32) -> HRESULT;
        fn GetPropertyInfo(&self, index: u32, name: *mut BSTR, prop_id: *mut PROPID, var_type: *mut VARTYPE) -> HRESULT;
        fn GetFolderProperty(&self, prop_id: PROPID, value: *mut PROPVARIANT) -> HRESULT;
    }

    #[uuid(Z7IGroups::IFolder.iface_iid(0x4))]
    pub unsafe interface IFolderWasChanged: IUnknown {
        fn WasChanged(&self, was_changed: *mut i32) -> HRESULT; //TODO bool
    }

    #[uuid(Z7IGroups::IFolder.iface_iid(0x7))]
    pub unsafe interface IFolderGetSystemIconIndex: IUnknown {
        pub fn GetSystemIconIndex(&self, index: u32, icon_index: *mut i32) -> HRESULT;
    }

    #[uuid(Z7IGroups::IFolder.iface_iid(0x8))]
    pub unsafe interface IFolderGetItemFullSize: IUnknown {
        fn GetItemFullSize(&self, index: u32, value: *mut PROPVARIANT, progress: *mut IProgress) -> HRESULT;
    }

    #[uuid(Z7IGroups::IFolder.iface_iid(0x9))]
    pub unsafe interface IFolderClone: IUnknown {
        fn Clone(&self, result_folder: *mut *mut IFolderFolder) -> HRESULT;
    }
    
    #[uuid(Z7IGroups::IFolder.iface_iid(0xA))]
    pub unsafe interface IFolderSetFlatMode: IUnknown {
        fn SetFlatMode(&self, flat_mode: i32) -> HRESULT;
    }

    #[uuid(Z7IGroups::IFolder.iface_iid(0x0B))]
    pub unsafe interface IFolderOperationsExtractCallback: IProgress {
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

    #[uuid(Z7IGroups::IFolder.iface_iid(0xE))]
    pub unsafe interface IFolderProperties: IUnknown {
        fn GetNumberOfFolderProperties(&self, num_properties: *mut u32) -> HRESULT;
        fn GetFolderPropertyInfo(&self, index: u32, name: *mut BSTR, prop_id: *mut PROPID, var_type: *mut VARTYPE) -> HRESULT;
    }

    #[uuid(Z7IGroups::IFolder.iface_iid(0x10))]
    pub unsafe interface IFolderArcProps: IUnknown {
        fn GetArcNumLevels(&self, num_levels: *mut u32) -> HRESULT;
        fn GetArcProp(&self, level: u32, prop_id: PROPID, value: *mut PROPVARIANT) -> HRESULT;
        fn GetArcNumProps(&self, level: u32, num_props: *mut u32) -> HRESULT;
        fn GetArcPropInfo(&self, level: u32, index: u32, name: *mut BSTR, prop_id: *mut PROPID, var_type: *mut VARTYPE) -> HRESULT;
        fn GetArcProp2(&self, level: u32, num_props: *mut u32) -> HRESULT;
        fn GetArcPropInfo2(&self, level: u32, index: u32, name: *mut BSTR, prop_id: *mut PROPID, var_type: *mut VARTYPE) -> HRESULT;
    }

    //genuinely why tf does this exist...
    #[uuid(Z7IGroups::IFolder.iface_iid(0x11))]
    pub unsafe interface IGetFolderArcProps: IUnknown {
        fn GetFolderArcProps(&self, object: *mut *mut IGetFolderArcProps) -> HRESULT;
    }

    #[uuid(Z7IGroups::IFolder.iface_iid(0x13))]
    pub unsafe interface IFolderOperations: IUnknown {
        fn CreateFolder(&self, name: *const wchar, progress: *mut IProgress) -> HRESULT;
        fn CreateFile(&self, name: *const wchar, progress: *mut IProgress) -> HRESULT;
        fn Rename(&self, index: u32, new_name: *const wchar, progress: *mut IProgress) -> HRESULT;
        fn Delete(&self, indicies: *const u32, num_items: u32, progress: *mut IProgress) -> HRESULT;
        fn CopyTo(&self, 
            move_mode: i32, 
            indicies: *const u32, 
            num_items: u32, 
            include_alt_streams: i32, 
            replace_alt_stream_chars_mode: i32, 
            path: *const wchar, 
            callback: *mut IFolderOperationsExtractCallback) -> HRESULT;
        fn CopyFrom(&self, move_mode: i32, from_folder_path: *const wchar, items_paths: *const *const wchar, num_items: u32, progress: *mut IProgress) -> HRESULT;
        fn SetProperty(&self, index: u32, prop_id: PROPID, value: *const PROPVARIANT, progress: *mut IProgress) -> HRESULT;
        fn CopyFromFile(&self, index: u32, full_file_path: *const wchar, progress: *mut IProgress) -> HRESULT;
    }

    #[uuid(Z7IGroups::IFolder.iface_iid(0x15))]
    pub unsafe interface IFolderCompare: IUnknown {
        fn CompareItems(&self, index1: u32, index2: u32, prop_id: PROPID, prop_is_raw: i32) -> HRESULT;
    }

    #[uuid(Z7IGroups::IFolder.iface_iid(0x16))]
    pub unsafe interface IFolderGetItemName: IUnknown {
        fn GetItemName(&self, index: u32, name: *mut *const wchar, len: *mut c_int) -> HRESULT;
        fn GetItemPrefix(&self, index: u32, name: *mut *const wchar, len: *mut c_int) -> u64;
    }

    #[uuid(Z7IGroups::IFolder.iface_iid(0x17))]
    pub unsafe interface IFolderAltStreams: IUnknown {
        fn BindToAltStreamsIndexed(&self, index: u32, result_folder: *mut *mut IFolderFolder) -> HRESULT;
        fn BindToAltStreamsNamed(&self, name: *const wchar, result_folder: *mut *mut IFolderFolder) -> HRESULT;
        fn AreAltStreamsSupported(&self, index: u32, is_supported: *mut i32) -> HRESULT; //TODO bool
    }

    #[uuid(Z7IGroups::IFolderManager.iface_iid(0x5))]
    pub unsafe interface IFolderManager: IUnknown {
        fn OpenFolderFile(&self, 
            in_stream: *mut IInStream, 
            file_path: *const wchar, 
            arc_format: *const wchar, 
            result_folder: *mut *mut IFolderFolder,
            progress: *mut IProgress
        ) -> HRESULT;
        fn GetExtensions(&self, extensions: *mut BSTR) -> HRESULT;
        fn GetIconPath(&self, ext: *const wchar, icon_path: *mut BSTR, icon_index: *mut i32) -> HRESULT;
    }
}