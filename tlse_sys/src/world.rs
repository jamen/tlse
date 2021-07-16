use std::os::raw::{c_double, c_float, c_long, c_ulong};

use crate::{
    BoostScopedPtr, C3DAnimationManager, C3DVector, CAIGameCameraBase, CArray,
    CBaseClassNonCopyable, CBulletTimeManager, CCamera, CCharString, CCountedPointer,
    CEnginePrimitiveHandle, CEnvironment, CFactionManager, CGameCameraManager,
    CGameDefinitionManager, CGameEvent, CGameScriptInterface, CGameTimeManager,
    CIntelligentPointer, CMeshDataBank, CMusicManager, CNavigationManager, CPlayerManager,
    CScriptInfoManager, CThing, CThingManager, CWideString, CWorldMap, CxxList, CxxVector,
};

use super::CMainGameComponent;

#[derive(Debug)]
#[repr(C)]
pub struct CWorld {
    pub vmt: *mut (),
    pub inherited_c_base_class_non_copyable: CBaseClassNonCopyable,
    pub inherited_ci_draw_world: CIDrawWorld,
    pub component: *mut CMainGameComponent,
    pub player_manager: *mut CPlayerManager,
    pub definition_manager: *const CGameDefinitionManager,
    pub p_world_map: BoostScopedPtr<CWorldMap>,
    pub p_environment: BoostScopedPtr<CEnvironment>,
    pub p_game_time_manager: BoostScopedPtr<CGameTimeManager>,
    pub p_thing_search_tools: BoostScopedPtr<CThingSearchTools>,
    pub p_atmos_processor: CCountedPointer<CAtmosProcess>,
    pub p_game_camera: BoostScopedPtr<CAIGameCameraBase>,
    pub p_game_camera_manager: BoostScopedPtr<CGameCameraManager>,
    pub p_current_game_camera: *mut CAIGameCameraBase,
    pub p_game_script_interface: BoostScopedPtr<CGameScriptInterface>,
    pub p_main_mesh_bank: CCountedPointer<CMeshDataBank>,
    pub p_animation_manager: *mut C3DAnimationManager,
    pub p_navigation_manager: BoostScopedPtr<CNavigationManager>,
    pub p_thing_combat_manager: BoostScopedPtr<CCombatManager>,
    pub p_thing_manager: BoostScopedPtr<CThingManager>,
    pub p_faction_manager: BoostScopedPtr<CFactionManager>,
    pub p_script_info_manager: CCountedPointer<CScriptInfoManager>,
    pub p_message_event_manager: CCountedPointer<CMessageEventManager>,
    pub p_bullet_time_manager: BoostScopedPtr<CBulletTimeManager>,
    pub p_music_manager: CCountedPointer<CMusicManager>,
    pub p_opinion_reaction_manager: CCountedPointer<COpinionReactionManager>,
    pub p_script_conversation_manager: BoostScopedPtr<CScriptConversationManager>,
    pub just_loaded: bool,
    pub current_world_name: CCharString,
    pub console_pause_at_frame_number: c_long,
    pub frame_started_3d_rendering: c_long,
    pub last_update_time_length: c_double,
    pub last_update_time: c_double,
    pub countdown_timer: c_long,
    pub paused: bool,
    pub slow_motion: bool,
    pub show_debug_text: bool,
    pub show_fps_text: bool,
    pub show_profile_text: bool,
    pub initial_active_quests: CArray<CCharString>,
    pub registered_quests: CxxVector<CCharString>,
    pub active_test_quests: CxxVector<CTestQuest>,
    pub creature_generation_disabled_groups: c_ulong,
    pub creature_generation_enabled: bool,
    pub teleporting_enabled: bool,
    pub experience_spending_enabled: bool,
    pub saving_enabled: bool,
    pub dont_populate_next_loaded_region: bool,
    pub hero_sleeping_enabled: bool,
    pub map_table_show_quest_cards_on_used: bool,
    pub screen_to_fade_in_on_next_region_change: bool,
    pub done_extra_frame_update_before_region_load_screen_fade_in: bool,
    pub mini_map_enabled: bool,
    pub mini_map_active_before_disabled: bool,
    pub region_loaded_display_region: bool,
    pub guild_master_messages_enabled: bool,
    pub summoner_death_explosion_affects_hero: bool,
    pub waiting_for_inventory_tutorial_to_finish: bool,
    pub hero_information_screen_mode_after_tutorial: bool,
    pub frame_cached_lod_center: c_long,
    pub cached_lod_center: C3DVector,
    // pub save_game_load_status: self::ESaveGameLoadStatus,
    pub save_game_load_status: u32,
    pub save_game_path_name: CWideString,
    pub auto_save_loacked: bool,
    pub serialising_about_to_load_hero_state: bool,
    pub serialising_hero_state: bool,
    pub serialising_non_persistent_quest_items: bool,
    // pub region_load_status: self::ERegionLoadStatus,
    pub region_load_status: u32,
    pub region_load_start_pos: C3DVector,
    pub region_load_start_angle_xy: c_float,
    pub region_load_followers: CxxVector<CIntelligentPointer<CThing>>,
    pub pervious_region: c_long,
    pub number_of_times_freeze_controls_mode_added_during_region_load: c_long,
    pub region_load_via_teleport: bool,
    pub region_load_via_door: bool,
    pub put_into_pause_mode_on_region_change: bool,
    pub region_load_waiting_for_confirmation: bool,
    pub region_load_screen_fully_faded: bool,
    pub region_load_screen_was_faded_out: bool,
    pub waiting_for_reset_to_front_end_confirmation: bool,
    // pub most_recent_save_type: self::ESaveType,
    pub most_recent_save_type: u32,
    // pub most_recent_save_type_before_manual_save: self::ESaveType,
    pub most_recent_save_type_before_manual_save: u32,
    pub most_recent_manual_save_name: CWideString,
    pub auto_save_check_point_exists: bool,
    pub save_game_marker_pos: C3DVector,
    pub save_game_marker_angle_xy: c_float,
    pub guild_seal_recall_pos: C3DVector,
    pub guild_seal_recall_angle_xy: c_float,
    pub weather_masking_primitives_sent: bool,
    pub weather_masking_primitive_handles: CxxVector<CEnginePrimitiveHandle>,
    pub atmos_banks_waiting_to_copy: CxxList<CAtmosProcess>,
    // pub player_spawn_status: self::EPlayerSpawnStatus,
    pub player_spawn_status: u32,
    pub villager_reaction_debug: bool,
    pub start_time: c_double,
    pub time_played: c_double,
    pub has_initialised_start_time: bool,
}

// TODO
#[derive(Debug)]
#[repr(C)]
pub struct CTestQuest {}

#[derive(Debug)]
#[repr(C)]
pub struct CScriptConversationManager {}

#[derive(Debug)]
#[repr(C)]
pub struct CThingSearchTools {}

#[derive(Debug)]
#[repr(C)]
pub struct CIDrawWorld {
    // Does it really have a vtable?
    pub vmt: *mut (),
}

#[derive(Debug)]
#[repr(C)]
pub struct CMessageEventManager {}

#[derive(Debug)]
#[repr(C)]
pub struct COpinionReactionManager {}

#[derive(Debug)]
#[repr(C)]
pub struct CAtmosProcess {}

#[derive(Debug)]
#[repr(C)]
pub struct CAtmosCopyInfo {}

#[derive(Debug)]
#[repr(C)]
pub struct CCombatManager {}

#[derive(Debug)]
#[repr(C)]
pub enum EPlayerSpawnStatus {
    PLAYER_SPAWN_STATUS_NULL = 0,
    PLAYER_SPAWN_STATUS_START = 1,
    PLAYER_SPAWN_STATUS_END = 2,
}

#[derive(Debug)]
#[repr(C)]
pub enum ESaveType {
    SAVE_TYPE_NONE = 0,
    SAVE_TYPE_MANUAL_SAVE = 1,
    SAVE_TYPE_AUTO_SAVE = 2,
    SAVE_TYPE_QUEST_START_SAVE = 3,
}

#[derive(Debug)]
#[repr(C)]
pub enum ESaveGameLoadStatus {
    SAVE_LOAD_STATUS_NONE = 0,
    SAVE_LOAD_STATUS_FADE_OUT = 1,
    SAVE_LOAD_STATUS_FADING_OUT = 2,
    SAVE_LOAD_STATUS_LOADING = 3,
    SAVE_LOAD_STATUS_FADE_IN = 4,
}

#[derive(Debug)]
#[repr(C)]
pub enum ERegionLoadStatus {
    NOT_LOADING_REGION = 0,
    WAITING_FOR_LOCKED_REGION_CONFIRMATION = 1,
    WAITING_FOR_CONFIRMATION = 2,
    WAITING_FOR_TELEPORT_EFFECT = 3,
    READY_TO_BEGIN_FADE_OUT = 4,
    WAITING_FOR_FADE_OUT = 5,
    LOADING_NEW_REGION = 6,
    LOADING_RESOURCES = 7,
    READY_FOR_FADE_IN = 8,
    WAITING_FOR_FADE_IN = 9,
}

impl CWorld {
    bind! {
        pub extern "thiscall" fn event_is_world_event(
            this: *mut CWorld,
            event: *mut CGameEvent
        ) -> bool = 0x0049d8c0;

        pub extern "thiscall" fn receive_local_event(
            this: *mut CWorld,
            event: *mut CGameEvent
        ) = 0x0049e1d0;

        pub extern "thiscall" fn get_current_render_camera(
            this: *mut CWorld,
            r: *mut CCamera
        ) = 0x0049da60;
    }
}
