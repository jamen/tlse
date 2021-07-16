use std::os::raw::{c_uchar, c_ulong};

use crate::{CArray, CBaseClassNonCopyable, CCharString};

#[derive(Debug)]
#[repr(C)]
pub struct CBankFile {
    pub inherited_base_class_non_copyable: CBaseClassNonCopyable,
    pub size: c_ulong,
    pub symbols: CArray<CCharString>,
    pub checksums: CArray<c_ulong>,
    pub runtime_data: CArray<self::CRuntimeData>,
    pub update_data: CArray<*mut CBankFileEntryUpdateData>,
    pub packed_data_offset: CPackedUIntArray,
}

#[derive(Debug)]
#[repr(C)]
pub struct CPackedUIntArray {
    pub packed_ints: *mut c_ulong,
    pub size: c_ulong,
    pub bits: c_uchar,
    pub bias: c_ulong,
}

#[derive(Debug)]
#[repr(C)]
pub struct CBankFileAsync {
    pub inherited_bank_file: CBankFile,
}

#[derive(Debug)]
#[repr(C)]
pub struct CBankFileEntryUpdateData {
    // pub state_block_crc: c_ulong,
// pub info_size: c_ulong,
// pub info_buffer: *mut c_char,
// pub filenames: CSmallVector<CCharString, 8>,
// pub requires_update: bool,
// pub exists: bool,
// pub state_block: CCountedPointer<CBankStateBlock>,
}

// #[repr(C)]
// #[derive(Debug)]
// pub struct CBankStateBlock {}

#[derive(Debug)]
#[repr(C)]
pub struct CRuntimeData {
    pub data_offset: c_ulong,
    pub data_size: c_ulong,
    pub data_type: c_uchar,
    pub valid: bool,
}
