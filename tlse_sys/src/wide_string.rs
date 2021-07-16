use std::cell::UnsafeCell;
use std::convert::TryFrom;
use std::fmt::{self, Debug};
use std::mem::MaybeUninit;
use std::os::raw::c_long;
use std::ptr::NonNull;
use std::string::FromUtf16Error;

use winapi::ctypes::wchar_t;

use crate::CxxBasicString;

#[repr(transparent)]
pub struct CWideString {
    pub inner: UnsafeCell<CWideStringInner>,
}

#[repr(C)]
pub struct CWideStringInner {
    pub ptr: *mut CWideStringData,
}

impl CWideString {
    bind! {
        pub extern "thiscall" fn constructor(
            this: NonNull<MaybeUninit<CWideStringInner>>,
            s: *mut wchar_t
        ) = 0x0099b6b0;

        pub extern "thiscall" fn clone_constructor(
            this: NonNull<MaybeUninit<CWideStringInner>>,
            other: NonNull<CWideStringInner>,
        ) = 0x0099b720;

        pub extern "thiscall" fn destructor(
            this: NonNull<CWideStringInner>,
        ) = 0x0099b510;
    }
}

impl Clone for CWideString {
    fn clone(&self) -> Self {
        unsafe {
            let mut char_str: MaybeUninit<CWideStringInner> = MaybeUninit::zeroed();

            (CWideString::clone_constructor.assume_init())(
                NonNull::new_unchecked(&mut char_str),
                NonNull::new(self.inner.get()).unwrap(),
            );

            CWideString {
                inner: UnsafeCell::new(char_str.assume_init()),
            }
        }
    }
}

impl Drop for CWideString {
    fn drop(&mut self) {
        unsafe { (CWideString::destructor.assume_init())(NonNull::new_unchecked(self.inner.get())) }
    }
}

impl From<String> for CWideString {
    fn from(s: String) -> CWideString {
        unsafe {
            let mut wide_str: MaybeUninit<CWideStringInner> = MaybeUninit::zeroed();
            let mut data: Vec<u16> = s.encode_utf16().collect();

            data.push(0);

            (Self::constructor.assume_init())(
                NonNull::new_unchecked(&mut wide_str),
                data.as_mut_ptr(),
            );

            CWideString {
                inner: UnsafeCell::new(wide_str.assume_init()),
            }
        }
    }
}

impl TryFrom<CWideString> for String {
    type Error = FromUtf16Error;

    fn try_from(val: CWideString) -> Result<Self, Self::Error> {
        unsafe {
            if (*val.inner.get()).ptr.is_null() {
                Ok(String::new())
            } else {
                let mut data = &mut (*(*val.inner.get()).ptr).data;

                if data.start.is_null() || data.last.is_null() {
                    Ok(String::new())
                } else {
                    let len = data.start.offset_from(data.last);
                    let slice = std::slice::from_raw_parts(data.start, len as usize);
                    String::from_utf16(slice)
                }
            }
        }
    }
}

impl Debug for CWideString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            String::try_from(self.clone()).or(Err(fmt::Error))?
        )
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct CWideStringData {
    pub data: CxxBasicString<wchar_t>,
    pub ref_count: c_long,
}
