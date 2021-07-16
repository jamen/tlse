use std::os::raw::c_ulong;

use crate::{C2DBoxF, C2DVector, CDisplayManager, CFontBank, CRGBColour, CWideString};

#[derive(Debug)]
#[repr(C)]
pub struct CRenderManager {
    // TODO
}

#[derive(Debug)]
#[repr(C)]
pub struct CRenderManager2D {
    // TODO: CRenderManager2DVmt
    pub vmt: *mut (),
    pub core: CRenderManagerCore,
    // TODO
}

#[derive(Debug)]
#[repr(C)]
pub struct CRenderManagerCore {
    // TODO: CRenderManagerCoreVmt
    pub vmt: *mut (),
    pub state_manager: CRenderStateManager,
    pub display_manager: *mut CDisplayManager,
    // TODO
}

// TODO
#[derive(Debug)]
#[repr(C)]
pub struct CRenderStateManager {
    _all: [u8; 14908],
}

impl CRenderManager2D {
    bind! {
        pub extern "thiscall" fn draw_2d_text(
            this: *mut CRenderManager2D,
            param_1: *mut C2DVector,
            param_2: CWideString,
            param_3: *mut CFontBank,
            param_4: *mut CRGBColour,
            param_5: c_ulong
        ) -> bool = 0x009dd8f0;

        pub extern "thiscall" fn draw_2d_box(
            this: *mut CRenderManager2D,
            param_1: *mut C2DBoxF,
            param_2: *mut CRGBColour,
            param_3: c_ulong,
        ) -> bool = 0x009de3f0;
    }
}
