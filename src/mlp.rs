use std::arch::asm;
use criterion::black_box;

#[repr(C)]
// Padded to 64 bytes to fill a cache line
pub struct PaddedPtr {
    pub next: *const PaddedPtr,
    pub padding: [u64; 7],
}

pub fn ptr_chase(pp: *mut *const PaddedPtr) {
    let mut p = unsafe { *pp };
    repeat2!(repeat100!(unsafe {
        asm!(
            "mov {0}, [{0}]",
            inout(reg) p,
        )
    }));
    unsafe { *pp = p };
}