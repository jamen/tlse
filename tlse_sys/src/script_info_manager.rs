#[derive(Debug)]
#[repr(C)]
pub struct CScriptInfoManager {}

impl CScriptInfoManager {
    bind! {
        pub extern "thiscall" fn register_scripts(this: *mut CScriptInfoManager) = 0x00cd52d0;
    }
}
