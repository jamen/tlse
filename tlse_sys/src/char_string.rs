use std::cell::UnsafeCell;
use std::convert::TryFrom;
use std::fmt::{self, Debug};
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_long};
use std::ptr::NonNull;
use std::string::FromUtf8Error;

use crate::{bind, CBasicString};

#[repr(transparent)]
pub struct CCharString {
    pub inner: UnsafeCell<CCharStringInner>,
}

#[repr(C)]
pub struct CCharStringInner {
    pub ptr: *mut CCharStringData,
}

#[repr(C)]
pub struct CCharStringData {
    pub basic_string: CBasicString<c_char>,
    pub ref_count: c_long,
}

impl CCharString {
    bind! {
        pub extern "thiscall" fn constructor(
            this: NonNull<MaybeUninit<CCharStringInner>>,
            data: *mut c_char,
            len: c_long
        ) = 0x0099ebf0;

        pub extern "thiscall" fn clone_constructor(
            this: NonNull<MaybeUninit<CCharStringInner>>,
            other: NonNull<CCharStringInner>,
        ) = 0x0099ec30;

        pub extern "thiscall" fn destructor(
            this: NonNull<CCharStringInner>,
        ) = 0x00afa3f0;
    }
}

impl Clone for CCharString {
    fn clone(&self) -> Self {
        unsafe {
            let mut char_str: MaybeUninit<CCharStringInner> = MaybeUninit::zeroed();

            (CCharString::clone_constructor.assume_init())(
                NonNull::new_unchecked(&mut char_str),
                NonNull::new(self.inner.get()).unwrap(),
            );

            CCharString {
                inner: UnsafeCell::new(char_str.assume_init()),
            }
        }
    }
}

impl Drop for CCharString {
    fn drop(&mut self) {
        unsafe { (CCharString::destructor.assume_init())(NonNull::new_unchecked(self.inner.get())) }
    }
}

impl From<String> for CCharString {
    fn from(val: String) -> CCharString {
        unsafe {
            let mut char_str: MaybeUninit<CCharStringInner> = MaybeUninit::zeroed();
            let mut data: Vec<u8> = val.into_bytes();

            data.push(0);

            (CCharString::constructor.assume_init())(
                NonNull::new_unchecked(&mut char_str),
                data.as_mut_ptr().cast(),
                -1,
            );

            CCharString {
                inner: UnsafeCell::new(char_str.assume_init()),
            }
        }
    }
}

impl TryFrom<CCharString> for String {
    type Error = FromUtf8Error;

    fn try_from(val: CCharString) -> Result<Self, Self::Error> {
        unsafe {
            if (*val.inner.get()).ptr.is_null() {
                Ok(String::new())
            } else {
                let mut data = &mut (*(*val.inner.get()).ptr).basic_string;

                if data.ptr.is_null() {
                    Ok(String::new())
                } else {
                    let slice = std::slice::from_raw_parts(data.ptr as *mut u8, data.len as usize);
                    String::from_utf8(slice.to_owned())
                }
            }
        }
    }
}

impl Debug for CCharString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            String::try_from(self.clone()).or(Err(fmt::Error))?
        )
    }
}
