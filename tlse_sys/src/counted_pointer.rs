use std::ffi::c_void;
use std::os::raw::c_long;

#[derive(Debug)]
#[repr(C)]
pub struct CCountedPointer<T> {
    pub data: *mut T,
    pub info: *mut CCPPointerInfo,
}

#[derive(Debug)]
#[repr(C)]
pub struct CCPPointerInfo {
    pub ref_count: c_long,
    /// Needs looking into. Produced an unnamed type that is 4 bytes. Its probably some function
    /// pointer.
    pub delete_func: usize,
    pub data: *mut c_void,
}
