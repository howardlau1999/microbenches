
pub type Ptr = *const u64;

pub unsafe fn ptr_chase(ptr: *mut Ptr) {
    let mut p = *ptr;
    repeat100!(repeat2!(
        {
            p = *(p as *const Ptr)
        }
    ));
    *ptr = p;
}

pub unsafe fn multi_ptr_chase_2(ptrs: [*mut Ptr; 2]) {
    let mut ptr0 = *ptrs[0];
    let mut ptr1 = *ptrs[1];
    repeat100!(repeat2!(
        {
            ptr0 = *(ptr0 as *const Ptr);
            ptr1 = *(ptr1 as *const Ptr);
        }
    ));
    *ptrs[0] = ptr0;
    *ptrs[1] = ptr1;
}