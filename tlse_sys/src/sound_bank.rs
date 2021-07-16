use std::os::raw::c_ulong;

use crate::{CBaseClassNonCopyable, CCountedPointer, CVectorMap};

#[derive(Debug)]
#[repr(C)]
pub struct CASoundBank {
    pub c_base_class_non_copyable: CBaseClassNonCopyable,
    pub p_symbol_map: CCountedPointer<CCRCSymbolMap>,
}

#[derive(Debug)]
#[repr(C)]
pub struct CCRCSymbolMap {
    pub inherited_base_class_non_copyable: CBaseClassNonCopyable,
    pub ulong_map: CVectorMap<c_ulong, c_ulong>,
}
