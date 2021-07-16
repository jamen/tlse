use crate::{CBaseClassNonCopyable, CThing};

#[derive(Debug)]
#[repr(C)]
pub struct CTCBase {
    pub vmt: *mut (),
    pub inherited_base_class_non_copyable: CBaseClassNonCopyable,
    pub parent_thing: *mut CThing,
    pub to_be_removed: bool,
    pub is_in_global_update_tcs: bool,
}
