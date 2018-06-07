use std::mem::{transmute, uninitialized};
use std::ptr;
use std::slice;

use libc;

pub const SEGMENT_SIZE: usize = 512;

pub struct Segment {
    inner: *mut u8,
}

impl Segment {
    pub fn new() -> Segment {
        unsafe {
            let inner = alloc();
            ptr::write_bytes(inner, 0, SEGMENT_SIZE);
            Segment { inner }
        }
    }

    pub fn id(&self) -> u32 {
        unsafe {
            let p: *const u32 = transmute(self.inner);
            ptr::read(p)
        }
    }

    pub fn set_id(&mut self, id: u32) {
        unsafe {
            let p: *mut u32 = transmute(self.inner);
            ptr::write(p, id);
        }
    }
}

impl AsRef<[u8]> for Segment {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.inner, SEGMENT_SIZE) }
    }
}

impl AsMut<[u8]> for Segment {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.inner, SEGMENT_SIZE) }
    }
}

impl Drop for Segment {
    fn drop(&mut self) {
        unsafe { dealloc(self.inner) };
    }
}

unsafe fn alloc() -> *mut u8 {
    let mut p: *mut u8 = uninitialized();
    libc::posix_memalign(
        &mut p as *mut *mut u8 as *mut *mut libc::c_void,
        SEGMENT_SIZE,
        SEGMENT_SIZE,
    );
    p
}

unsafe fn dealloc(p: *mut u8) {
    libc::free(p as *mut libc::c_void);
}
