use crate::CxxVector;

#[derive(Debug)]
#[repr(C)]
pub struct CArray<T> {
    pub inherited_cxx_vector: CxxVector<T>,
}
