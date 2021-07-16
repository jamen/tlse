use super::CObservable;

#[derive(Debug)]
#[repr(C)]
pub struct CManager {
    c_observable: CObservable,
}
