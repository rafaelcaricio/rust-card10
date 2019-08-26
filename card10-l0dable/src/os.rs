use card10_sys::*;

pub fn exec(path: &str) -> ! {
    let mut pathbuf = super::str_to_cstr(path);
    unsafe {
        epic_exec(pathbuf.as_mut_ptr());
    }
    unreachable!()
}

pub fn exit(ret: i32) -> ! {
    unsafe {
        epic_exit(ret);
    }
    unreachable!()
}

pub fn system_reset() -> ! {
    unsafe {
        epic_system_reset();
    }
    unreachable!()
}
