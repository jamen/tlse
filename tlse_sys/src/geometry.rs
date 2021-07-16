use std::os::raw::{c_float, c_long};

// TODO: Replace or re-implement these with `mint`?

/// Top left and bottom right positions.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct C2DBoxF {
    pub x1: c_float,
    pub y1: c_float,
    pub x2: c_float,
    pub y2: c_float,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct C2DCoordI {
    pub x: c_long,
    pub y: c_long,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct C2DVector {
    pub x: c_float,
    pub y: c_float,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct C3DVector {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

// TODO
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct C2DExtentsI {
    pub x: c_long,
    pub y: c_long,
}

// TODO
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct C2DBoxI {
    pub x1: c_long,
    pub y1: c_long,
    pub x2: c_long,
    pub y2: c_long,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct CRightHandedSet {
    pub up: C3DVector,
    pub forward: C3DVector,
}
