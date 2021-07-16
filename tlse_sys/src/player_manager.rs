use std::mem::MaybeUninit;
use std::os::raw::c_long;
use std::ptr::NonNull;

use crate::{
    CBaseClassNonCopyable, CCharString, CGameDefinitionManager, CMainGameComponent, CPlayer,
    CxxVector,
};

#[derive(Debug)]
#[repr(C)]
pub struct CPlayerManager {
    pub vmt: *mut (),
    pub inherited_base_class_non_copyable: CBaseClassNonCopyable,
    pub component: *mut CMainGameComponent,
    /// Apparently this is a forward declaration with no actual definition. See also
    /// CDefinitionManager.
    pub definition_manager: *const CGameDefinitionManager,
    pub players: CxxVector<*mut CPlayer>,
    pub player_neutral: c_long,
    pub main_player: c_long,
    pub hero_swap_player_script_names: CxxVector<CCharString>,
}

impl CPlayerManager {
    bind! {
        pub extern "thiscall" fn get_main_player(this: *mut CPlayerManager) -> *mut CPlayer = 0x00449970;
    }
}
