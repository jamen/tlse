use crate::{CIntelligentPointer, CThing, CThingPhysical};

#[derive(Debug)]
#[repr(C)]
pub struct CThingGameObject {
    pub vmt: *mut (),
    pub c_thing_physical: CThingPhysical,
    pub p_thing_standing_on: CIntelligentPointer<CThing>,
    pub unknown: u32,
}
