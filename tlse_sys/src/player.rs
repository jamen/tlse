use std::os::raw::c_long;

use crate::{
    BoostScopedPtr, C3DVector, CCamera, CCountedPointer, CDefPointer, CGameDefinitionManager,
    CGamePlayerInterface, CInitBaseClass, CInputProcessBButtonExitMode, CInputProcessBetting,
    CInputProcessBlock, CInputProcessBoastUI, CInputProcessCameraLookAround,
    CInputProcessClickPastText, CInputProcessCombat, CInputProcessConsole,
    CInputProcessControlCreature, CInputProcessControlCreatureActivateZTarget,
    CInputProcessControlCreatureActivateZTargetOnPress, CInputProcessControlCreatureRightStick,
    CInputProcessControlFreeCamera, CInputProcessControlSpirit, CInputProcessCreatureMovement,
    CInputProcessCreatureMovementWatchForControlAngleChange, CInputProcessCreditsUI,
    CInputProcessCutScene, CInputProcessCycleSpecialCameraModes, CInputProcessDead,
    CInputProcessDebugControls, CInputProcessDigging, CInputProcessFireheartMinigame,
    CInputProcessFirstPerson, CInputProcessFirstPersonLookAround,
    CInputProcessFirstPersonTargeting, CInputProcessFishing, CInputProcessFreezeControls,
    CInputProcessHeroAbilitiesScreen, CInputProcessHeroInformationScreens, CInputProcessInGameMenu,
    CInputProcessInventory, CInputProcessInventoryClothing, CInputProcessInventoryExperienceScreen,
    CInputProcessInventoryMagicScreen, CInputProcessInventoryMapScreen,
    CInputProcessInventoryQuestsScreen, CInputProcessInventoryStatsScreen,
    CInputProcessInventoryTradeScreen, CInputProcessInventoryWeapons,
    CInputProcessJumpingAndRolling, CInputProcessLightning, CInputProcessMain,
    CInputProcessOracleMinigame, CInputProcessParalysed, CInputProcessPhotojournalCapture,
    CInputProcessProjectileTargetingAnalogueZoom, CInputProcessQuestCompletionUI,
    CInputProcessQuickAccessItems, CInputProcessQuickAccessMenu, CInputProcessRebootGame,
    CInputProcessSetRangedWeaponMode, CInputProcessSetRangedWeaponThirdPersonMode,
    CInputProcessSpecialAbilities, CInputProcessStrafe, CInputProcessTargetLockCycleTargets,
    CInputProcessTargetLockRightStickTargetSelect, CInputProcessTavernGame,
    CInputProcessToggleViewHeroMode, CInputProcessUseEnvironment, CInputProcessUseRangedWeapon,
    CInputProcessUseRangedWeaponZLock, CInputProcessWatchForRangedWeaponThirdPersonModeTermination,
    CInputProcessWatchForWillChargeUpThirdPersonModeTermination, CInputProcessYesNoQuestion,
    CInputProcessZTarget, CIntelligentPointer, CMainGameComponent, CPlayerDef, CPlayerManager,
    CThing, CThingManager, CThingPlayerCreature, CWideString, CWorld, CWorldMap, CxxList,
};

#[derive(Debug)]
#[repr(C)]
pub struct CPlayer {
    pub vmt: *mut (),
    pub c_init_base_class: CInitBaseClass,
    pub drawing_free_cam_debug: bool,
    pub player_is_character: bool,
    pub player_interface: *mut CGamePlayerInterface,
    pub player_manager: *mut CPlayerManager,
    pub component: *const CMainGameComponent,
    pub definition_manager: *const CGameDefinitionManager,
    pub world: *mut CWorld,
    pub thing_manager: *mut CThingManager,
    pub p_world_map: *mut CWorldMap,
    pub number: c_long,
    pub controlled_creature: CIntelligentPointer<CThingPlayerCreature>,
    pub player_character: CIntelligentPointer<CThingPlayerCreature>,
    pub player_respawn_delay: c_long,
    pub current_free_camera: CCamera,
    pub free_camera: CCamera,
    pub old_free_camera: CCamera,
    pub respawn_info: self::CThingRespawnInfo,
    pub p_hero_log_book: CCountedPointer<CHeroLogBook>,
    pub input_process_main: BoostScopedPtr<CInputProcessMain>,
    pub input_process_control_creature: BoostScopedPtr<CInputProcessControlCreature>,
    pub input_process_z_target: BoostScopedPtr<CInputProcessZTarget>,
    pub input_process_dead: BoostScopedPtr<CInputProcessDead>,
    pub input_process_inventory: BoostScopedPtr<CInputProcessInventory>,
    pub input_process_inventory_clothing: BoostScopedPtr<CInputProcessInventoryClothing>,
    pub input_process_inventory_weapons: BoostScopedPtr<CInputProcessInventoryWeapons>,
    pub input_process_hero_abilities_screen: BoostScopedPtr<CInputProcessHeroAbilitiesScreen>,
    pub input_process_inventory_map_screen: BoostScopedPtr<CInputProcessInventoryMapScreen>,
    pub input_process_inventory_stats_screen: BoostScopedPtr<CInputProcessInventoryStatsScreen>,
    pub input_process_inventory_magic_screen: BoostScopedPtr<CInputProcessInventoryMagicScreen>,
    pub input_process_inventory_experience_screen:
        BoostScopedPtr<CInputProcessInventoryExperienceScreen>,
    pub input_process_inventory_trade_screen: BoostScopedPtr<CInputProcessInventoryTradeScreen>,
    pub input_process_inventory_quests_screen: BoostScopedPtr<CInputProcessInventoryQuestsScreen>,
    pub input_process_hero_information_screens: BoostScopedPtr<CInputProcessHeroInformationScreens>,
    pub input_process_first_person: BoostScopedPtr<CInputProcessFirstPerson>,
    pub input_process_click_past_text: BoostScopedPtr<CInputProcessClickPastText>,
    pub input_process_yes_no_question: BoostScopedPtr<CInputProcessYesNoQuestion>,
    pub input_process_freeze_controls: BoostScopedPtr<CInputProcessFreezeControls>,
    pub input_process_control_free_camera: BoostScopedPtr<CInputProcessControlFreeCamera>,
    pub input_process_creature_movement: BoostScopedPtr<CInputProcessCreatureMovement>,
    pub input_process_use_environment: BoostScopedPtr<CInputProcessUseEnvironment>,
    pub input_process_block: BoostScopedPtr<CInputProcessBlock>,
    pub input_process_debug_controls: BoostScopedPtr<CInputProcessDebugControls>,
    pub input_process_quick_access_items: BoostScopedPtr<CInputProcessQuickAccessItems>,
    pub input_process_combat: BoostScopedPtr<CInputProcessCombat>,
    pub input_process_special_abilities: BoostScopedPtr<CInputProcessSpecialAbilities>,
    pub input_process_control_creature_right_stick:
        BoostScopedPtr<CInputProcessControlCreatureRightStick>,
    pub input_process_right_stick_look_around: BoostScopedPtr<CInputProcessCameraLookAround>,
    pub input_process_cycle_special_camera_modes:
        BoostScopedPtr<CInputProcessCycleSpecialCameraModes>,
    pub input_process_reboot_game: BoostScopedPtr<CInputProcessRebootGame>,
    pub input_process_jumping_and_rolling: BoostScopedPtr<CInputProcessJumpingAndRolling>,
    pub input_process_first_person_targeting: BoostScopedPtr<CInputProcessFirstPersonTargeting>,
    pub input_process_use_ranged_weapon: BoostScopedPtr<CInputProcessUseRangedWeapon>,
    pub input_process_control_creature_activate_z_target:
        BoostScopedPtr<CInputProcessControlCreatureActivateZTarget>,
    pub input_process_control_creature_activate_z_target_on_press:
        BoostScopedPtr<CInputProcessControlCreatureActivateZTargetOnPress>,
    pub input_process_left_stick_look_around: BoostScopedPtr<CInputProcessFirstPersonLookAround>,
    pub input_process_in_game_menu: BoostScopedPtr<CInputProcessInGameMenu>,
    pub input_process_control_spirit: BoostScopedPtr<CInputProcessControlSpirit>,
    pub input_process_tavern_game: BoostScopedPtr<CInputProcessTavernGame>,
    pub input_process_set_ranged_weapon_mode: BoostScopedPtr<CInputProcessSetRangedWeaponMode>,
    pub input_process_cut_scene: BoostScopedPtr<CInputProcessCutScene>,
    pub input_process_fishing: BoostScopedPtr<CInputProcessFishing>,
    pub input_process_digging: BoostScopedPtr<CInputProcessDigging>,
    pub input_process_paralysed: BoostScopedPtr<CInputProcessParalysed>,
    pub input_process_boast_ui: BoostScopedPtr<CInputProcessBoastUI>,
    pub input_process_set_ranged_weapon_third_person:
        BoostScopedPtr<CInputProcessSetRangedWeaponThirdPersonMode>,
    pub input_process_use_ranged_weapon_z_lock: BoostScopedPtr<CInputProcessUseRangedWeaponZLock>,
    pub input_process_watch_for_ranged_weapon_third_person_mode_termination:
        BoostScopedPtr<CInputProcessWatchForRangedWeaponThirdPersonModeTermination>,
    pub input_process_watch_for_will_charge_up_third_person_mode_termination:
        BoostScopedPtr<CInputProcessWatchForWillChargeUpThirdPersonModeTermination>,
    pub input_process_target_lock_cycle_targets:
        BoostScopedPtr<CInputProcessTargetLockCycleTargets>,
    pub input_process_quick_access_menu: BoostScopedPtr<CInputProcessQuickAccessMenu>,
    pub input_process_quest_completion_ui: BoostScopedPtr<CInputProcessQuestCompletionUI>,
    pub input_process_target_lock_right_stick_target_select:
        BoostScopedPtr<CInputProcessTargetLockRightStickTargetSelect>,
    pub input_process_b_button_exit_mode: BoostScopedPtr<CInputProcessBButtonExitMode>,
    pub input_process_credits_ui: BoostScopedPtr<CInputProcessCreditsUI>,
    pub input_process_betting: BoostScopedPtr<CInputProcessBetting>,
    pub input_process_lightning: BoostScopedPtr<CInputProcessLightning>,
    pub input_process_watch_for_control_angle_change:
        BoostScopedPtr<CInputProcessCreatureMovementWatchForControlAngleChange>,
    pub input_process_oracle_minigame: BoostScopedPtr<CInputProcessOracleMinigame>,
    pub input_process_fireheart_minigame: BoostScopedPtr<CInputProcessFireheartMinigame>,
    pub input_process_strafe: BoostScopedPtr<CInputProcessStrafe>,
    pub input_process_console: BoostScopedPtr<CInputProcessConsole>,
    pub input_process_projectile_targeting_analogue_zoom:
        BoostScopedPtr<CInputProcessProjectileTargetingAnalogueZoom>,
    pub input_process_photojournal_capture: BoostScopedPtr<CInputProcessPhotojournalCapture>,
    pub input_process_toggle_view_hero_mode: BoostScopedPtr<CInputProcessToggleViewHeroMode>,
    pub local: bool,
    pub show_world_thing: bool,
    pub z_targeting: bool,
    pub projectile_target_locked: bool,
    pub right_trigger_target_locked: bool,
    pub player_modes: CxxList<EPlayerMode>,
    pub supress_full_mode_removal: bool,
    pub disallow_mode_changes: bool,
    pub initial_player_mode_set: bool,
    pub aggressive_mode: bool,
    pub spell_mode: bool,
    pub expression_shift_mode: bool,
    pub pc_projectile_weapon_third_person_aiming_mode: bool,
    pub kill_everything_mode: bool,
    pub player_def: CDefPointer<CPlayerDef>,
    pub using_free_cam: bool,
    pub free_camera_tracking_player: bool,
    pub free_camera_tracking_player_yz: bool,
    pub controlling_free_camera: bool,
    pub keep_abilities_during_cutscenes: bool,
    pub joystick_device_number: c_long,
}

#[derive(Debug)]
#[repr(C)]
pub struct CThingRespawnInfo {
    pub died_position: C3DVector,
    pub died_info_set: bool,
}

#[derive(Debug)]
#[repr(C)]
pub struct CHeroLogBook {
    // TODO
}

#[derive(Debug)]
#[repr(C)]
pub struct CHeroLogBookEntry {
    pub name: CWideString,
    pub abbreviated_name: CWideString,
    pub content: CWideString,
}

#[derive(Debug)]
#[repr(C)]
pub enum EPlayerMode {
    PLAYER_MODE_NULL = 0x0000,
    PLAYER_MODE_CONTROL_CREATURE = 0x0001,
    PLAYER_MODE_Z_TARGET = 0x0002,
    PLAYER_MODE_DEAD = 0x0003,
    PLAYER_MODE_NAVIGATE_INVENTORY = 0x0004,
    PLAYER_MODE_NAVIGATE_INVENTORY_CLOTHING = 0x0005,
    PLAYER_MODE_NAVIGATE_INVENTORY_WEAPONS = 0x0006,
    PLAYER_MODE_NAVIGATE_INVENTORY_ABILITIES_SCREEN = 0x0007,
    PLAYER_MODE_NAVIGATE_INVENTORY_MAP_SCREEN = 0x0008,
    PLAYER_MODE_NAVIGATE_INVENTORY_MAGIC_SCREEN = 0x0009,
    PLAYER_MODE_NAVIGATE_INVENTORY_STATS_SCREEN = 0x000a,
    PLAYER_MODE_NAVIGATE_INVENTORY_EXPERIENCE_SCREEN = 0x000b,
    PLAYER_MODE_NAVIGATE_INVENTORY_TRADE_SCREEN = 0x000c,
    PLAYER_MODE_NAVIGATE_INVENTORY_QUESTS_SCREEN = 0x000d,
    PLAYER_MODE_CLICK_PAST_TEXT = 0x000e,
    PLAYER_MODE_YES_NO_QUESTION = 0x000f,
    PLAYER_MODE_FIRST_PERSON = 0x0010,
    PLAYER_MODE_FREEZE_CONTROLS = 0x0011,
    PLAYER_MODE_SPECIAL_ABILITY_FREEZE_CONTROLS = 0x0012,
    PLAYER_MODE_CONTROL_CAMERA = 0x0013,
    PLAYER_MODE_LOOK_THROUGH_WINDOW = 0x0014,
    PLAYER_MODE_REBOOT_GAME = 0x0015,
    PLAYER_MODE_FIRST_PERSON_TARGETING = 0x0016,
    PLAYER_MODE_NAVIGATE_IN_GAME_MENU = 0x0017,
    PLAYER_MODE_CONTROL_SPIRIT = 0x0018,
    PLAYER_MODE_USE_PROJECTILE_WEAPON = 0x0019,
    PLAYER_MODE_TAVERN_GAME = 0x001a,
    PLAYER_MODE_CUT_SCENE = 0x001b,
    PLAYER_MODE_CHARGE_QUICK_ACCESS = 0x001c,
    PLAYER_MODE_FISHING = 0x001d,
    PLAYER_MODE_DIGGING = 0x001e,
    PLAYER_MODE_PARALYSED = 0x001f,
    PLAYER_MODE_BOAST_UI = 0x0020,
    PLAYER_MODE_BERSERK = 0x0021,
    PLAYER_MODE_USE_PROJECTILE_WEAPON_THIRD_PERSON = 0x0022,
    PLAYER_MODE_CHARGE_UP_WILL_SPELL = 0x0023,
    PLAYER_MODE_QUICK_ACCESS_MENU = 0x0024,
    PLAYER_MODE_QUEST_COMPLETION_UI = 0x0025,
    PLAYER_MODE_CREDITS_UI = 0x0026,
    PLAYER_MODE_BETTING = 0x0027,
    PLAYER_MODE_LIGHTNING = 0x0028,
    PLAYER_MODE_ORACLE_MINIGAME = 0x0029,
    PLAYER_MODE_FIREHEART_MINIGAME = 0x002a,
    PLAYER_MODE_LIVE_GUI = 0x002b,
    PLAYER_MODE_CONSOLE = 0x002c,
    PLAYER_MODE_TAKE_SCREENSHOT_FOR_PHOTOJOURNAL = 0x002d,
    PLAYER_MODE_PC_PROJECTILE_WEAPON_THIRD_PERSON_AIMING = 0x002e,
    PLAYER_MODE_VIEW_HERO = 0x002f,
    MAX_NO_PLAYER_MODES = 0x0030,
}

impl CPlayer {
    bind! {
        pub extern "thiscall" fn get_p_controlled_creature(this: *mut Self) -> *mut CThingPlayerCreature = 0x00487dc0;

        pub extern "thiscall" fn get_p_player_character_thing(this: *mut Self) -> *mut CThing = 0x00487b70;
    }
}
