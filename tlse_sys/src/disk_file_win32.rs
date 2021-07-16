#[derive(Debug)]
#[repr(C)]
pub struct CDiskFileWin32 {
    // TODO: pub vmt: *mut CDiskFileWin32Vmt,
    pub vmt: *mut (),
    // TODO
}
