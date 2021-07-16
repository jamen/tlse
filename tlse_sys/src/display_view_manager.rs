use crate::{CCountedPointer, NDisplayView};

#[derive(Debug)]
#[repr(C)]
pub struct CDisplayViewManager {
    pub current_view: CCountedPointer<NDisplayView::CViewBase>,
}
