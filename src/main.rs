#![no_std]
#![no_main]

// Importiamo i componenti della PS Vita
use psp2_sys as libvita;

// Questo pezzo serve a gestire gli errori se il programma crasha
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *mut *mut u8) -> i32 {
    // Per ora il programma non fa nulla, vogliamo solo vedere se compila
    // il "cuore" di Ruffle insieme al codice Vita.
    0
}
