use std::os::raw::{c_double, c_float, c_long, c_ulong};

use crate::{
    C2DBoxF, CBaseClassNonCopyable, CCamera, CCountedPointer, CDisplayViewManager,
    CGameDefinitionManager, CGameEvent, CGraphicDataBank, CInitBaseClass, CMainGameComponent,
    CMeshDataBank, CPlayerManager, CRGBColour, CWorld,
};

/// TODO: This is behind a pointer so I've left it empty for now.
#[derive(Debug)]
#[repr(C)]
pub struct CDisplayEngine {
    pub vmt: *mut (),
    pub inherited_base_class_non_copyable: CBaseClassNonCopyable,
    pub engine_preload_flags: c_ulong,
    pub component: *const CMainGameComponent,
    pub player_manager: *const CPlayerManager,
    pub definition_manager: *const CGameDefinitionManager,
    pub engine_graphic_bank: *const CGraphicDataBank,
    pub mesh_bank: *mut CMeshDataBank,
    pub main_window: C2DBoxF,
    pub engine_3d: *mut CIEngine,
    pub camera: CCamera,
    pub frame: c_long,
    pub draw_game: bool,
    pub letter_box: CLetterBoxModeInfo,
    pub camera_inputs_on: bool,
    pub last_refresh_time: c_double,
    pub last_render_time_length: c_double,
    pub last_world_update_render_time: c_double,
    pub prepare_primitives: bool,
    pub gamma_ramp: c_float,
    pub old_gamma_ramp: c_float,
    pub screen_fade_out_info: CFadeInFadeOutBase,
    pub screen_fade_out_locked: bool,
    pub draw_memory_use: bool,
    pub draw_debug_page: c_long,
    pub p_view_manager: CCountedPointer<CDisplayViewManager>,
    pub initial_faide_readyness_count: c_long,
    pub initial_fade_active: bool,
    pub screen_fade_duration: c_float,
    pub time_since_fade_started: c_float,
    pub world: *const CWorld,
}

#[derive(Debug)]
#[repr(C)]
pub struct CLetterBoxModeInfo {
    pub inherited_fade_in_fade_out_base: CFadeInFadeOutBase,
    pub ratio: c_float,
}

#[derive(Debug)]
#[repr(C)]
pub struct CFadeInFadeOutBase {
    pub active: bool,
    pub fade_in_time: c_float,
    pub fade_out_time: c_float,
    pub closing: bool,
    pub opening: bool,
    pub open_timer: c_float,
    pub close_timer: c_float,
    pub to_colour: CRGBColour,
}

/// The methods on this are more interesting. Maybe CI = Class Interface?
#[derive(Debug)]
#[repr(C)]
pub struct CIEngine {
    pub vmt: *mut (),
    pub inherited_init_base_class: CInitBaseClass,
    pub active: bool,
}

impl CDisplayEngine {
    bind! {
        pub extern "thiscall" fn receive_feedback_game_event(this: *mut Self, event: *mut CGameEvent) = 0x00434a30;
    }
}
