use libc::size_t;
use std::{slice, str};

#[repr(C)]
pub struct RustAllocatedString {
    data: *const u8,
    length: size_t,
}

impl From<String> for RustAllocatedString {
    fn from(string: String) -> Self {
        let bytes = string.into_boxed_str().into_boxed_bytes();
        let raw = Box::into_raw(bytes);

        unsafe {
            RustAllocatedString {
                data: (*raw).as_ptr(),
                length: (*raw).len(),
            }
        }
    }
}

impl Drop for RustAllocatedString {
    fn drop(&mut self) {
        unsafe {
            let slice = slice::from_raw_parts_mut(self.data as *mut u8, self.length);
            Box::from_raw(slice);
        }
    }
}

impl RustAllocatedString {
    pub fn as_str(&self) -> &str {
        let slice = unsafe { slice::from_raw_parts(self.data, self.length) };
        str::from_utf8(slice).unwrap()
    }
}

#[repr(C)]
pub struct Slice<'a> {
    data: &'a u8,
    length: size_t,
}

impl<'a> Into<&'a str> for Slice<'a> {
    fn into(self) -> &'a str {
        str::from_utf8(self.into()).unwrap()
    }
}

impl<'a> Into<String> for Slice<'a> {
    fn into(self) -> String {
        let slice: &'a str = self.into();
        slice.to_owned()
    }
}

impl<'a> Into<&'a [u8]> for Slice<'a> {
    fn into(self) -> &'a [u8] {
        unsafe { slice::from_raw_parts(self.data, self.length) }
    }
}

impl From<&'static [u8]> for Slice<'static> {
    fn from(bytes: &'static [u8]) -> Self {
        Slice {
            data: unsafe { &*bytes.as_ptr() },
            length: bytes.len(),
        }
    }
}

impl From<&'static str> for Slice<'static> {
    fn from(string: &'static str) -> Self {
        Slice::from(string.as_bytes())
    }
}

pub type ExternallyAllocatedByteArr<'a> = Slice<'a>;
pub type StaticRustAllocatedString = Slice<'static>;