use std::fmt;
use std::os::raw::{c_double, c_long, c_uchar, c_ulong};

use crate::{CGameEvent, CGameEventPackage, CInitBaseClass, CMainGameComponent};

#[repr(C)]
pub struct CNetworkClient {
    // TODO: pub vmt: *mut CNetworkClientVmt,
    pub vmt: *mut (),
    pub c_init_base_class: CInitBaseClass,
    pub server: CNetworkServer,
    pub receive_buffer: [c_uchar; 8192],
    pub local_event_package: CGameEventPackage,
    pub last_update_time: c_double,
    pub first_time: bool,
    pub host: bool,
    pub local_game: bool,
    pub local_player: *const CNetworkPlayer,
    pub host_player: *const CNetworkPlayer,
    pub local_frame: c_long,
    pub checksum1: c_ulong,
    pub checksum2: c_ulong,
    pub game_component: *mut CMainGameComponent,
}

/// Unknown
#[derive(Debug)]
#[repr(C)]
pub struct CNetworkPlayer {}

#[derive(Debug)]
#[repr(C)]
pub struct CNetworkServer {
    // TODO: pub vmt: *mut CNetworkServerVmt,
    pub vmt: *mut (),
    pub inherited_init_base_class: CInitBaseClass,
}

impl CNetworkClient {
    bind! {
        pub extern "thiscall" fn send_game_event(this: *mut Self, event: *mut CGameEvent) = 0x004ae9a0;
    }
}

impl fmt::Debug for CNetworkClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CNetworkClient")
            .field("c_init_base_class", &self.c_init_base_class)
            .field("server", &self.server)
            .field("receive_buffer", &&self.receive_buffer[..])
            .field("local_event_package", &self.local_event_package)
            .field("last_update_time", &self.last_update_time)
            .field("first_time", &self.first_time)
            .field("host", &self.host)
            .field("local_game", &self.local_game)
            .field("local_player", &self.local_player)
            .field("host_player", &self.host_player)
            .field("local_frame", &self.local_frame)
            .field("checksum1", &self.checksum1)
            .field("checksum2", &self.checksum2)
            .field("game_component", &self.game_component)
            .finish()
    }
}
