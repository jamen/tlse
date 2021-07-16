use std::os::raw::c_ulong;

#[derive(Debug)]
#[repr(C)]
pub struct CBasicString<T> {
    pub ptr: *mut T,
    pub len: c_ulong,
    pub data_length_and_use_fast_extend: u32,
}
