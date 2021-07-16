use std::ffi::c_void;
use std::os::raw::{c_char, c_float, c_long, c_ulong};

use crate::{
    CCharString, CCountedPointer, CDebugManager, CDisplayManager, CDisplayManagerInit,
    CFontManager, CInitBaseClass, CInputManager, CInputManagerInit, CProfileManager, CWideString,
};

use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::HINSTANCE;
use winapi::shared::windef::HWND;
use winapi::um::winnt::RTL_CRITICAL_SECTION;
use winapi::um::winuser::WNDCLASSEXW;

#[repr(C)]
pub struct CSystemManager {
    pub vmt: *mut (),
    pub c_init_base_class: CInitBaseClass,
    pub windows_quit: bool,
    pub application_active: bool,
    pub application_has_input_focus: bool,
    pub application_guid: GUID,
    pub best_guid: GUID,
    pub p_best_guid: *mut GUID,
    pub prefer_primary_device: bool,
    pub critical_section: RTL_CRITICAL_SECTION,
    pub main_fibre: *mut c_void,
    pub lib_debug_manager: CCountedPointer<CDebugManager>,
    pub p_input_manager: CCountedPointer<CInputManager>,
    pub p_display_manager: CCountedPointer<CDisplayManager>,
    pub debug_manager: *mut CDebugManager,
    pub p_graphic_bank_manager: CCountedPointer<CGraphicBankManager>,
    pub p_mesh_bank_manager: CCountedPointer<CMeshBankManager>,
    pub p_font_manager: CCountedPointer<CFontManager>,
    pub window_initialised: bool,
    pub d_draw_initialised: bool,
    pub win_instance_handle: *mut HINSTANCE,
    pub application_win_handle: *mut HWND,
    pub win_show: u32,
    pub win_command_line: c_char,
    pub win_app_name: CWideString,
    pub win_class: WNDCLASSEXW,
    pub resolution_set: bool,
    pub init_flags: c_ulong,
    pub ever_been_active: bool,
    pub mouse_button_0_presed: bool,
    pub mouse_button_1_presed: bool,
    pub mouse_button_2_presed: bool,
    pub mouse_button_3_presed: bool,
    pub mouse_button_4_presed: bool,
    pub mouse_z_move: c_long,
    pub window_valid: bool,
    pub restore_exclusive: bool,
    pub restore: bool,
    pub wait_while_inactive: bool,
    pub attached_to_external_window: bool,
    pub prevent_exclusive_mode_changes: bool,
    pub using_accurate_timer: bool,
    // Pointer to an unknown function type.
    pub app_reinit_func: *mut (),
    pub app_reinit_context: *mut c_void,
    pub profile_manager: CCountedPointer<CProfileManager>,
    pub system_colours: CGuiColoursInfo,
    pub temp_path: CWideString,
    pub application_name: CCharString,
    pub force_music_play: bool,
    pub force_music_play_offset: c_float,
    // Pointer to an unknown function type.
    pub error_func: *mut (),
    // Pointer to an unknown function type.
    pub ime_message_func: *mut (),
    // Pointer to an unknown function type.
    pub mesage_filter: *mut (),
}

#[derive(Debug)]
#[repr(C)]
pub struct CSystemManagerInit {
    pub instance_handle: *mut HINSTANCE,
    pub command_line: *mut c_char, // RChar ?
    pub windows_show: i32,
    pub app_name: CWideString,
    pub exclusive_mode: bool,
    pub prefer_primary_display_device: bool,
    pub flags: c_ulong,
    pub prevent_hardware_acceleration: bool,
    pub attach_to_external_window: bool,
    pub application_window_handle: *mut HWND,
    pub prevent_exclusive_mode_changes: bool,
    pub use_accurate_timer: bool,
    pub app_reinit_func: *mut (), // Unknown function pointer
    pub app_reinit_context: *mut c_void,
    pub font_bank_handle: CCharString,
    pub streaming_font_bank_handle: CCharString,
    pub wait_while_inactive: bool,
    pub skip_config_detection: bool,
    pub input_manager_init: CInputManagerInit,
    pub display_manager_init: CDisplayManagerInit,
    pub use_triple_buffer: bool,
    pub is_error_failure_func: *mut (), // Unknown function pointer
    pub identifier: [c_char; 17],       // ? probably wrong
    pub scratch_buffer_size: c_long,
    pub logfile_name: CWideString,
    pub temp_path: CWideString,
}

// TODO
#[derive(Debug)]
#[repr(C)]
pub struct CGuiColoursInfo {}

// TODO
#[derive(Debug)]
#[repr(C)]
pub struct CGraphicBankManager {}

// TODO
#[derive(Debug)]
#[repr(C)]
pub struct CMeshBankManager {}

impl CSystemManagerInit {
    bind! {
        pub extern "fastcall" fn constructor(this: *mut CSystemManagerInit) = 0x00403b10;
    }
}
