use std::os::raw::{c_char, c_float, c_long, c_ulong};

use crate::{
    BoostScopedPtr, C3DVector, CCountedPointer, CDefPointer, CEnginePrimitiveHandle,
    CIntelligentPointer, CThing, CThingGameObject, CxxList, CxxVector,
};

#[derive(Debug)]
#[repr(C)]
pub struct CThingCreatureBase {
    pub vmt: *mut (),
    pub c_thing_game_object: CThingGameObject,
    pub last_message_event_i_created_id: c_ulong,
    // TODO: pub combat_collision_debug_graphics: CxxVector<CEnginePrimitiveHandle>,
    pub combat_collision_debug_graphics: CxxVector<()>,
    // TODO: pub p_def: CDefPointer<CThingCreatureDef>,
    pub p_def: CDefPointer<()>,
    pub shot_accuracy_percentage: c_long,
    pub initial_pos: C3DVector,
    pub p_last_attacked_by_creature: CIntelligentPointer<CThingCreatureBase>,
    pub wf_last_attacked_by_Creature: c_ulong,
    // TODO: pub p_current_action: CCountedPointer<CCreatureActionBase>,
    pub p_current_action: CCountedPointer<()>,
    // TODO: pub p_queued_actions: CxxList<CCountedPointer<CCreatureActionBase>>,
    pub p_queued_actions: CxxList<CCountedPointer<()>>,
    pub movement_vector: C3DVector,
    pub head_pos_offset: C3DVector,
    pub idle_counter: c_long,
    pub turn_speed: c_float,
    // TODO: pub p_creature_interaction: CCountedPointer<CCreatureInteraction>,
    pub p_creature_interaction: CCountedPointer<()>,
    // TODO: pub p_tc_mode_manager: *mut CTCCreatureModeManager,
    pub p_tc_mode_manager: *mut (),
    pub previous_action_handedness: ECombatAnimationHandedness,
    pub previous_action_handedness_wd: c_long,
    // TODO: pub body_reorienter: BoostScopedPtr<CThingBodyReorienter>,
    pub body_rorienter: BoostScopedPtr<()>,
    pub combat_debug_graphics: CxxVector<CEnginePrimitiveHandle>,
    pub p_item_to_unseathe_after_cutscene: CIntelligentPointer<CThing>,
    /// c_rchar
    pub debug_text: c_char,
    pub unknown: u32,
}

#[derive(Debug)]
#[repr(C)]
pub enum ECombatAnimationHandedness {
    HANDED_RIGHT = 0,
    HANDED_LEFT = 1,
    HANDED_NONE = 2,
}
