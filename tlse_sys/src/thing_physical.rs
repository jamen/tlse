use std::os::raw::{c_float, c_long};

use crate::{CIntelligentPointer, CThing, CxxList};

#[derive(Debug)]
#[repr(C)]
pub struct CThingPhysical {
    pub vmt: *mut (),
    pub inherited_thing: CThing,
    // TODO: pub health_over_time_effects: CxxList<CThingPhysical::CHealthOtherTimeHelper>,
    pub health_over_time_effects: CxxList<()>,
    pub scale: c_float,
    pub p_last_creator: CIntelligentPointer<CThingPhysical>,
    pub p_original_creator: CIntelligentPointer<CThingPhysical>,
    pub max_health: c_float,
    pub health: c_float,
    pub can_become_between_camera_and_observed_thing: c_long,
    pub unknown: u32,
}
