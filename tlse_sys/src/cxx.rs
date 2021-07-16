use std::fmt;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::os::raw::{c_char, c_ulong};

// use winapi::shared::basetsd::UINT32;

/// [`boost::scoped_ptr`](https://www.boost.org/doc/libs/1_39_0/libs/smart_ptr/scoped_ptr.htm)
#[derive(Debug)]
#[repr(C)]
pub struct BoostScopedPtr<T>(pub *mut T);

#[derive(Debug, Default)]
#[repr(C)]
pub struct CxxAllocator<T> {
    t: PhantomData<T>,
}

#[derive(Debug)]
#[repr(C)]
pub struct CxxBasicString<A: Sized, B = CxxCharTraits<A>, C = CxxAllocator<A>> {
    pub start: *mut A,
    pub last: *mut A,
    pub end: *mut A,
    pub unknown_3: usize,
    pub unknown_4: *mut A,
    pub unknown_5: *mut A,
    char_traits: B,
    alloc: C,
}

/// This has no fields and only exists for static methods.
#[derive(Debug, Default)]
#[repr(C)]
pub struct CxxCharTraits<A> {
    _value_type: PhantomData<A>,
}

#[derive(Debug)]
#[repr(C)]
pub struct CxxLess<T> {
    _elem_type: PhantomData<T>,
}

#[derive(Debug)]
#[repr(C)]
pub struct CxxList<T, A = super::CxxAllocator<T>> {
    pub proxy: *mut (),
    pub head: *mut CxxListNode<T>,
    pub size: u32,
    pub _alloc_node: CxxAllocator<CxxListNode<T>>,
    pub _alloc_value: A,
}

#[derive(Debug)]
#[repr(C)]
pub struct CxxListNode<T> {
    pub next: *mut CxxListNode<T>,
    pub prev: *mut CxxListNode<T>,
    pub value: T,
}

#[derive(Debug)]
#[repr(C)]
pub struct CxxMap<Key, T, Compare = CxxLess<Key>, Alloc = CxxAllocator<CxxPair<Key, T>>> {
    pub proxy: *mut (),
    pub comp: Compare,
    pub head: *mut CxxMap<T, Key, Compare, Alloc>,
    pub _aloc_node: CxxAllocator<CxxMap<T, Key, Compare, Alloc>>,
    pub _alloc_value: Alloc,
}

#[derive(Debug)]
#[repr(C)]
pub struct CxxPair<A, B>(pub A, pub B);

#[derive(Debug)]
#[repr(C)]
pub struct CxxSet<T, C = super::CxxLess<T>, A = super::CxxAllocator<T>> {
    pub comp: C,
    pub head: *mut CxxSetNode,
    pub size: u32,
    pub alloc_node: super::CxxAllocator<CxxSet<T, C, A>>,
    pub alloc_value: A,
    _elem_type: PhantomData<T>,
}

pub struct CxxSetNode {
    pub left: *mut CxxSetNode,
    pub parent: *mut CxxSetNode,
    pub right: *mut CxxSetNode,
    // Should this be a type parameter?
    pub val: c_ulong,
    pub color: c_char,
    pub is_nil: c_char,
}

#[repr(C)]
pub struct CxxVector<T, A = CxxAllocator<T>> {
    pub first: *mut T,
    pub last: *mut T,
    /// This marks the capacity of the vector. The "last" field marks the last element.
    pub end: *mut T,
    pub allocator: A,
}

impl<T, A> CxxVector<T, A> {
    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.first, self.len()) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.first, self.len()) }
    }

    pub fn len(&self) -> usize {
        Self::ptr_offset_from(self.first, self.last)
    }

    pub fn capacity(&self) -> usize {
        Self::ptr_offset_from(self.first, self.end)
    }

    // Using this scary method until ptr_offset_from is stabilized.
    // https://doc.rust-lang.org/std/primitive.pointer.html#method.offset_from
    fn ptr_offset_from(a: *mut T, b: *mut T) -> usize {
        let size = std::mem::size_of::<T>();
        if size == 0 {
            return 0;
        }
        debug_assert!(
            !a.is_null() && !b.is_null(),
            "The pointers must be non-null."
        );
        debug_assert!(
            a <= b,
            "The first pointer cannot come after the second pointer."
        );
        let dist = b as usize - a as usize;
        dist / size
    }
}

impl<T: fmt::Debug, A> fmt::Debug for CxxVector<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.as_slice()).finish()
    }
}
