use std::os::raw::{c_double, c_float, c_long, c_ulong};

use crate::{
    bind, BoostScopedPtr, CASoundBank, CCharString, CCountedPointer, CDiskFileWin32,
    CDisplayEngine, CFontBank, CFrameRateSmoother, CGameComponent, CGameEvent,
    CGameEventPackageSet, CGamePlayerInterface, CGraphicDataBank, CNetworkClient, CPlayerManager,
    CWideString, CWorld, NGameText,
};

#[derive(Debug)]
#[repr(C)]
pub struct CMainGameComponent {
    // TODO: pub vmt: *mut CMainGameComponentVmt
    pub vmt: *mut (),
    pub c_game_component: CGameComponent,
    pub p_sample_bank: *mut CASoundBank,
    pub p_text_bank: CCountedPointer<NGameText::CDataBank>,
    pub p_player_manager: BoostScopedPtr<CPlayerManager>,
    pub p_player_interface: BoostScopedPtr<CGamePlayerInterface>,
    pub p_world: BoostScopedPtr<CWorld>,
    pub p_display_engine: BoostScopedPtr<CDisplayEngine>,
    pub p_lua: CCountedPointer<CLUA>,
    pub force_update_tick: bool,
    pub force_update_tick_speed_multiplier: c_float,
    pub force_update_tick_speed_desired_framerate: c_float,
    pub force_update_no_failed_updates: c_long,
    pub first_world_frame_update: bool,
    pub current_server_frame: c_long,
    pub input_server_frame: c_long,
    pub last_game_turn_force_rendered: c_long,
    pub current_frame_start_game_time: c_double,
    pub game_start_time: c_double,
    pub last_frame_render_duration: c_double,
    pub last_interpolation_info: CInterpolationInfo,
    // Verify this is correctly sized. Everything after this will be affected if not.
    pub event_package_set: CGameEventPackageSet,
    pub client: CNetworkClient,
    pub render_frames_since_last_game_update_count: c_ulong,
    pub world_seed: c_ulong,
    pub local_seed: c_ulong,
    pub p_debug_font: CCountedPointer<CFontBank>,
    pub p_cut_scene_main_font: CCountedPointer<CFontBank>,
    pub event_package_file: CDiskFileWin32,
    pub loading_event_packages: bool,
    pub saving_event_packages: bool,
    pub event_package_file_header: CEventPackageFileHeader,
    pub frame_rate_smoother: CFrameRateSmoother,
    pub last_render_frame_start_time: c_double,
    pub time_passed_since_last_update: c_float,
    pub last_update_time: c_float,
    pub world_update_turn: bool,
    pub rough_fps_counter: CRoughFrameCounter,
    pub next_component_to_run: *mut CGameComponent,
    pub p_main_graphic_bank: CCountedPointer<CGraphicDataBank>,
    pub init_structure: CMainGameComponentInit,
    pub initialised: bool,
    pub allow_render: bool,
    pub rendered: bool,
    pub debug_frames_unable_to_render_count: c_long,
}

#[derive(Debug)]
#[repr(C)]
pub struct CMainGameComponentInit {
    pub initial_world_name: CWideString,
    pub initial_world_holy_site_name: CWideString,
    pub initial_quest_name: CCharString,
    pub save_game_name: CWideString,
}

#[derive(Debug)]
#[repr(C)]
pub struct CEventPackageFileHeader {
    pub no_players: c_long,
}

#[derive(Debug)]
#[repr(C)]
pub struct CRoughFrameCounter {
    pub frame_start: c_ulong,
    pub fps: c_float,
}

#[derive(Debug)]
#[repr(C)]
pub struct CLUA {
    // TODO
}

#[derive(Debug)]
#[repr(C)]
pub struct CInterpolationInfo {
    pub current: self::CInterpolationInfoSet,
    pub last: self::CInterpolationInfoSet,
    pub paused_current: self::CInterpolationInfoSet,
    pub paused_last: self::CInterpolationInfoSet,
    pub bullet_time_current: self::CInterpolationInfoSet,
    pub bullet_time_last: self::CInterpolationInfoSet,
    pub wf_server_current_time: c_ulong,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInterpolationInfoSet {
    gt_predicted_time_since_last_render_frame: c_float,
    wf_interpolate: c_float,
}

impl CMainGameComponent {
    bind! {
        pub extern "thiscall" fn event_is_system_event(
            this: *mut CMainGameComponent,
            event: *mut CGameEvent
        ) -> bool = 0x00415ff2;

        pub extern "thiscall" fn update(
            this: *mut CMainGameComponent
        ) = 0x0041e5f2;

        pub extern "thiscall" fn is_controller_disconnected(
            this: *mut CMainGameComponent
        ) -> bool = 0x00416296;

        pub extern "thiscall" fn add_game_event(
            this: *mut CMainGameComponent,
            event: *mut CGameEvent
        ) = 0x009f1760;

        pub extern "thiscall" fn game_frame_difference_from_current(
            this: *mut CMainGameComponent,
            frame: c_long
        ) -> c_long = 0x004167a7;

        pub extern "thiscall" fn signal_frame_update(
            this: *mut CMainGameComponent
        ) = 0x00416047;

        pub extern "thiscall" fn add_event_package_set_to_save(
            this: *mut CMainGameComponent,
            event_package_set: *mut CGameEventPackageSet
        ) = 0x004161a7;

        pub extern "thiscall" fn update_from_event_package_set(
            this: *mut CMainGameComponent,
            event_package_set: *mut CGameEventPackageSet
        ) = 0x0041726d;

        pub extern "thiscall" fn is_time_for_server_update(
            this: *mut CMainGameComponent,
            current_server_frame: c_long
        ) -> bool = 0x0041674a;

        pub extern "thiscall" fn get_event_package_set_from_save(
            this: *mut CMainGameComponent,
            event_package_set: *mut CGameEventPackageSet
        ) -> bool = 0x00416148;

        pub extern "thiscall" fn init_window(
            this: *mut CMainGameComponent,
            p1: c_long,
            p2: c_long
        ) -> bool = 0x009a64b0;
    }
}
