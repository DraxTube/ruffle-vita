#![no_std]
#![no_main]

// Usiamo la nuova libreria vitasdk
use vitasdk_sys as libvita;

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *mut *mut u8) -> i32 {
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
