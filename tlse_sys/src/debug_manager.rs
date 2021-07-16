#[derive(Debug)]
#[repr(C)]
pub struct CDebugManager {
    // pub system_manager: *mut CSystemManager,
// pub vcalid: bool,
// pub exclusive: bool,
// pub verbose: bool,
// pub can_do_verbose: bool,
// pub debug_manager_critical_section: RTL_CRITICAL_SECTION,
// pub log_messages: bool,
// pub log_errors: bool,
// // This is an stdio.h FILE but I don't know whats best to bind to.
// pub log_file: *mut (),
// pub project_directory: [WCHAR; 263],
// pub error_display_info: [CErrorDisplayInfo; 5],
// pub p_exclusive_assert: CExclusiveAssert,
// pub win_instance: HINSTANCE,
// pub win_handle: HWND,
// pub errors_off: CxxSet<c_ulong>,
}

#[derive(Debug)]
#[repr(C)]
pub struct CErrorDisplayInfo {
    pub display_errors: bool,
}
