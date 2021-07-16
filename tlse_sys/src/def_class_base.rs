use std::marker::PhantomData;
use std::os::raw::{c_long, c_ulong};

use crate::{CDefinitionManager, CVectorMap};

#[derive(Debug)]
#[repr(C)]
pub struct CDefClassBase {
    pub vmt: *mut (),
    pub c_def_pointee_base: CDefPointeeBase,
    pub p_def_manager: *mut CDefinitionManager,
    pub global_index: c_ulong,
    pub setup: (),                // unknown empty type
    pub template: (),             // unknown empty type
    pub default_vals_applied: (), // unknown empty type
}

#[derive(Debug)]
#[repr(C)]
pub struct CDefPointer<T> {
    pub object: *mut CDefPointeeBase,
    _elem_type: PhantomData<T>,
}

#[derive(Debug)]
#[repr(C)]
pub struct CDefPointeeBase {
    pub vmt: *mut (),
    pub inherited_resource: CResource,
}

#[derive(Debug)]
#[repr(C)]
pub struct CResource {
    pub vmt: *mut (),
    pub inherited_iv_counted_pointee_base: CIVCountedPointeeBase,
    pub resource_list: *mut CResourceList,
    pub prev_resource: *mut CResource,
    pub next_resource: *mut CResource,
    pub resource_size: c_ulong,
    pub last_used_frame: c_ulong,
}

#[derive(Debug)]
#[repr(C)]
pub struct CResourceList {
    pub vmt: *mut (),
    pub inherited_failed_allocation_handler: CFailedAllocationHandler,
    pub head: CResource,
    pub resource_count: c_ulong,
    pub allocated_memory: c_long,
    pub maximum_memory: c_long,
    pub current_frame: c_ulong,
    pub debug_stats_frame: c_ulong,
    pub unloaded_delay: c_ulong,
    pub unload_this_frame: c_ulong,
}

#[derive(Debug)]
#[repr(C)]
pub struct CFailedAllocationHandler {}

#[derive(Debug)]
#[repr(C)]
pub struct CIVCountedPointeeBase {
    pub ref_count: c_ulong,
}

#[derive(Debug)]
#[repr(C)]
pub struct CParentDefClassBase {
    pub vmt: *mut (),
    pub c_def_class_base: CDefClassBase,
    pub instantiation_name: CDefString,
    pub sub_def_info_map: CVectorMap<c_ulong, self::CSubDefInfo>,
}

#[derive(Debug)]
#[repr(C)]
pub struct CSubDefInfo {
    pub def_index: c_long,
    pub original_parent_def_index: c_long,
}

#[derive(Debug)]
#[repr(C)]
pub struct CDefString {
    pub table_pos: c_long,
}
