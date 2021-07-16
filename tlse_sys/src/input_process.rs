use std::os::raw::{c_char, c_long};

use crate::{
    CBaseClassNonCopyable, CGameDefinitionManager, CGameEvent, CGamePlayerInterface, CLinkedList,
    CLinkedListPosition, CMainGameComponent, CPlayer, CWorld,
};

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessManager {
    pub vmt: *mut (),
    pub inherited_base_class_non_copyable: CBaseClassNonCopyable,
    pub input_process_list: CLinkedList<CAInputProcess>,
    pub queued_processed_inputs: [CProcessedInput; 10],
    pub queued_processed_inputs_count: c_long,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessGameBase {
    pub vmt: *mut (),
    pub ca_input_process: CAInputProcess,
    pub world: *mut CWorld,
    pub player: *mut CPlayer,
    pub definition_manager: *const CGameDefinitionManager,
    pub component: *const CMainGameComponent,
    pub p_game_player_interface: *const CGamePlayerInterface,
}

#[derive(Debug)]
#[repr(C)]
pub struct CAInputProcess {
    pub vmt: *mut (),
    pub inherited_base_class_non_copyable: CBaseClassNonCopyable,
    pub p_player_interface: CInputProcessManager,
    pub input_process_list_pos: CLinkedListPosition,
}

#[derive(Debug)]
#[repr(C)]
pub struct CProcessedInput {
    pub player: c_long,
    pub event_type: EProcessedEventType,
    pub game_events: [CGameEvent; 4],
    pub game_events_count: c_char,
    pub priority: EGameEventPriority,
}

#[derive(Debug)]
#[repr(C)]
pub enum EGameEventPriority {
    GAME_EVENT_PRIORITY_NULL = 0,
    GAME_EVENT_PRIORITY_MIN = 1,
    GAME_EVENT_PRIORITY_LOW = 2,
    GAME_EVENT_PRIORITY_MEDIUM = 3,
    GAME_EVENT_PRIORITY_HIGH = 4,
    GAME_EVENT_PRIORITY_MAX = 5,
}

#[derive(Debug)]
#[repr(C)]
pub enum EProcessedEventType {
    PROCESSED_INPUT_NULL = 0,
    PROCESSED_INPUT_GAME_EVENTS = 1,
    PROCESSED_INPUT_PERFORMED_EVENT = 2,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessBButtonExitMode {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessBetting {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessBlock {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessBoastUI {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessCameraLookAround {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessClickPastText {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessCombat {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessConsole {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessControlCreature {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessControlCreatureActivateZTarget {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessControlCreatureActivateZTargetOnPress {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessControlCreatureRightStick {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessControlFreeCamera {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessControlSpirit {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessCreatureMovement {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessCreatureMovementWatchForControlAngleChange {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessCreditsUI {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessCutScene {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessCycleSpecialCameraModes {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessDead {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessDebugControls {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessDigging {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessFireheartMinigame {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessFirstPerson {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessFirstPersonLookAround {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessFirstPersonTargeting {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessFishing {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessFreezeControls {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessHeroAbilitiesScreen {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessHeroInformationScreens {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInGameMenu {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInventory {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInventoryClothing {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInventoryExperienceScreen {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInventoryMagicScreen {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInventoryMapScreen {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInventoryQuestsScreen {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInventoryStatsScreen {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInventoryTradeScreen {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInventoryWeapons {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessJumpingAndRolling {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessLightning {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessMain {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessOracleMinigame {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessParalysed {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessPhotojournalCapture {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessProjectileTargetingAnalogueZoom {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessQuestCompletionUI {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessQuickAccessItems {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessQuickAccessMenu {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessRebootGame {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessSetRangedWeaponMode {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessSetRangedWeaponThirdPersonMode {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessSpecialAbilities {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessStrafe {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessTargetLockCycleTargets {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessTargetLockRightStickTargetSelect {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessTavernGame {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessToggleViewHeroMode {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessUseEnvironment {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessUseRangedWeapon {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessUseRangedWeaponZLock {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessWatchForRangedWeaponThirdPersonModeTermination {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessWatchForWillChargeUpThirdPersonModeTermination {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessYesNoQuestion {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessZTarget {
    pub inherited_input_process_game_base: CInputProcessGameBase,
}
