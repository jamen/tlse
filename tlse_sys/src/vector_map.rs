use std::marker::PhantomData;

use crate::{CArray, CxxPair};

#[derive(Debug)]
#[repr(C)]
pub struct CVectorMap<K, V, C = CKeyPairCompareLess<K, V>> {
    pub c_array: CArray<CxxPair<K, V>>,
    pub compare: C,
    pub dirty: bool,
}

#[derive(Debug)]
#[repr(C)]
pub struct CKeyPairCompareLess<A, B> {
    a: PhantomData<A>,
    b: PhantomData<B>,
}
