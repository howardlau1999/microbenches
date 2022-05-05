#[repr(C)]
// Padded to 64 bytes to fill a cache line
pub struct PaddedPtr {
    pub next: *const PaddedPtr,
    pub padding: [u64; 7],
}

pub unsafe fn ptr_chase(ptr: *mut *const PaddedPtr) {
    let mut p = *ptr;
    repeat100!(repeat2!(
        p = *(p as *const *const PaddedPtr));
    );
    *ptr = p;
}

pub unsafe fn multi_ptr_chase_2(ptrs: [*mut *const PaddedPtr; 2]) {
    let mut ptr0 = *ptrs[0];
    let mut ptr1 = *ptrs[1];
    repeat100!(repeat2!(
        {
            ptr0 = *(ptr0 as *const *const PaddedPtr);
            ptr1 = *(ptr1 as *const *const PaddedPtr);
        }
    ));
    *ptrs[0] = ptr0;
    *ptrs[1] = ptr1;
}