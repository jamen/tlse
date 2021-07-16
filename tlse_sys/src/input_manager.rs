use std::os::raw::{c_long, c_ulong};

use crate::{BoostScopedPtr, CBaseClass, CCountedPointer, CDiskFileWin32, CxxVector};

#[derive(Debug)]
#[repr(C)]
pub struct CInputManager {
    // pub inherited_base_class: CBaseClass,
// pub processed_events: CxxVector<CInputEvent>,
// pub event_store: CxxVector<CInputEvent>,
// pub stored_event_count: c_long,
// pub event_scans_running: c_long,
// pub p_mouse: BoostScopedPtr<CMouse>,
// pub p_keyboard: BoostScopedPtr<CKeyboard>,
// pub p_joysticks: CxxVector<CCountedPointer<CJoystick>>,
// pub loading: bool,
// pub saving: bool,
// pub checksum: CChecksum,
// pub loaded_event_packages: CxxVector<CInputEventPackage>,
// pub current_file_package_index: c_long,
// pub timestamp: c_long,
// pub package_file: CDiskFileWin32,
// pub update_count: c_ulong,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputManagerInit {
    pub use_mouse: bool,
    pub use_joystick: bool,
    pub use_keyboard: bool,
}
