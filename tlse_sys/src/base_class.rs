use std::marker::PhantomData;
use std::os::raw::{c_uchar, c_ulong};

#[derive(Debug)]
#[repr(C)]
pub struct CBaseObject {
    pub vmt: *mut (),
    pub c_base_class: CBaseClass,
    pub intelligent_pointer: *mut CBaseObjectPointer,
}

#[derive(Debug)]
#[repr(C)]
pub struct CBaseObjectPointer {
    pub object: *mut CBaseObject,
    pub ref_count: c_ulong,
}

#[derive(Debug)]
#[repr(C)]
pub struct CBaseClass {}

#[derive(Debug)]
#[repr(C)]
pub struct CBaseClassNonCopyable {
    pub inherited_base_class: CBaseClass,
}

#[derive(Debug)]
#[repr(C)]
pub struct CClass {
    pub vmt: *mut (),
    pub c_base_object: CBaseObject,
    // TODO: pub class_list_pos: CxxListIterator<*mut CClass>,
    pub class_list_pos: u32,
    // TODO: pub type_list_pos: CxxListIterator<*mut CClass>,
    pub type_list_pos: u32,
    pub ty: c_uchar,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInitBaseClass {
    pub vmt: *mut (),
    pub c_base_class: CBaseClass,
    // pub valid: bool,
}

#[derive(Debug)]
#[repr(C)]
pub struct CBaseIntelligentPointer {
    pub unknown: u32,
    pub ptr: *mut CBaseObjectPointer,
}

#[derive(Debug)]
#[repr(C)]
pub struct CIntelligentPointer<T> {
    pub vmt: *mut (),
    pub inherited_base_intelligent_pointer: CBaseIntelligentPointer,
    _elem_type: PhantomData<T>,
}
