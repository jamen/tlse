use std::fmt;
use std::os::raw::{c_char, c_long, c_uchar, c_ulong};

#[derive(Debug)]
#[repr(C)]
pub struct CGameEvent {
    pub event_type: c_long,
    pub player: c_char,
    pub data: [c_uchar; 32],
    pub end_pos: c_uchar,
    /// Many of the events have data that is invalid, e.g. the event_type and data are seemingly
    /// random. This flag tells the game the event is valid and in use.
    pub valid: bool,
    /// I think this indicates whether the event should be replaced?
    /// Some events that were seemingly valid, but marked invalid, are marked with replacement.
    pub replacement: bool,
}

#[repr(C)]
pub struct CGameEventPackage {
    pub timestamp: c_long,
    pub events_count: c_ulong,
    pub events: [CGameEvent; 40],
}

#[derive(Debug)]
#[repr(C)]
pub struct CGameEventPackageSet {
    pub packages_count: c_ulong,
    pub packages: [CGameEventPackage; 50],
}

impl fmt::Debug for CGameEventPackage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CGameEventPackage")
            .field("timestamp", &self.timestamp)
            .field("events_count", &self.events_count)
            .field("events", &&self.events[..])
            .finish()
    }
}

// impl fmt::Debug for CGameEventPackageSet {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.debug_struct("CGameEventPackageSet")
//             .field("packages_count", &self.packages_count)
//             .field("packages", &&self.packages[..])
//             .finish()
//     }
// }
