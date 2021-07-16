use std::os::raw::c_ulong;

use crate::{CBaseClassNonCopyable, CCharString, CFontManager};

#[derive(Debug)]
#[repr(C)]
pub struct CFontBank {
    pub vmt: *mut (),
    pub inherited_base_class_non_copyable: CBaseClassNonCopyable,
    pub font_height: c_ulong,
    pub name: CCharString,
    pub p_manager: *mut CFontManager,
}
