use crate::{CCharString, CCountedPointer, CFontBank};

#[derive(Debug)]
#[repr(C)]
pub struct CFontManager {
    // TODO
}

impl CFontManager {
    bind! {
        pub extern "thiscall" fn get_font(this: *mut Self, name: *const CCharString) -> CCountedPointer<CFontBank> = 0x009e2c80;
    }
}
