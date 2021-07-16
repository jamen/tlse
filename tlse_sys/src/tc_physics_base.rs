use std::os::raw::{c_float, c_long};

use crate::{C3DVector, CDefPointer, CTCBase};

#[derive(Debug)]
#[repr(C)]
pub struct CTCPhysicsBase {
    pub vmt: *mut (),
    pub ctc_base: CTCBase,
    pub position: C3DVector,
    pub old_position: C3DVector,
    pub old_position_last_frame_set: c_long,
    pub velocity: C3DVector,
    pub radius: c_float,
    // TODO: pub physics_def: CDefPointer<CPhysicsDef>,
    pub physics_Def: CDefPointer<()>,
}
