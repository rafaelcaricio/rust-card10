use card10_sys::*;

/// Execute Python script or ELF file
pub fn exec(path: &str) -> ! {
    let mut pathbuf = super::str_to_cstr(path);
    unsafe {
        epic_exec(pathbuf.as_mut_ptr());
    }
    unreachable!()
}

/// Exit current l0dable
pub fn exit(ret: i32) -> ! {
    unsafe {
        epic_exit(ret);
    }
    unreachable!()
}

/// Cause a reboot
pub fn system_reset() -> ! {
    unsafe {
        epic_system_reset();
    }
    unreachable!()
}
