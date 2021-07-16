use std::os::raw::{c_uchar, c_ulong};

#[derive(Debug)]
#[repr(C)]
pub struct CRuntimeData {
    pub data_offset: c_ulong,
    pub data_size: c_ulong,
    pub data_type: c_uchar,
    pub valid: bool,
}
