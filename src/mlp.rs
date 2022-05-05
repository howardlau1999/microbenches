#[repr(C)]
// Padded to 64 bytes to fill a cache line
pub struct PaddedPtr {
    pub next: *const PaddedPtr,
    pub padding: [u64; 7],
}

pub unsafe fn ptr_chase(pp: *mut *const PaddedPtr) {
    let mut p = *pp ;
    repeat100!(repeat2!(p = *(p as *const *const PaddedPtr)));
    *pp = p;
}