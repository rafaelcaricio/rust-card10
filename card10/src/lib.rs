#![no_std]

pub use max32665;
pub use cortex_m_rt as _;
pub use cortex_m_rt::entry;

#[link(name = "card10")]
extern {
    fn card10_init();
    fn card10_diag();
}

#[no_mangle]
pub extern "C" fn puts() {
    /* Stub */
}

#[no_mangle]
pub extern "C" fn printf() {
    /* Stub */
}

pub fn init() {
    unsafe {
        card10_init();
        card10_diag();
    }
}
