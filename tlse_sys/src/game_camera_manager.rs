use crate::CCamera;

#[derive(Debug)]
#[repr(C)]
pub struct CGameCameraManager {
    // TODO: CGameCameraManagerVmt
    pub vmt: *mut (),
    pub _padding: u32,
    pub camera: CCamera,
    pub old_camera: CCamera,
    // TODO: Everything else
}

#[derive(Debug)]
#[repr(C)]
pub struct CAIGameCameraBase {}

#[derive(Debug)]
#[repr(C)]
pub struct CAGameCameraBase {}
