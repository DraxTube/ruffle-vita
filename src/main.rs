#![no_std]
#![no_main]

// Importiamo i componenti della PS Vita
use psp2_sys as libvita;

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *mut *mut u8) -> i32 {
    // 1. Inizializzazione Hardware della Vita (Schermo e Audio)
    // Qui andranno i comandi per accendere il display
    
    // 2. Caricamento del motore Ruffle
    // let player = ruffle_core::Player::new(...);

    // 3. Ciclo infinito per far girare il gioco
    loop {
        // Leggi input (tasti Vita)
        // Disegna frame Flash sullo schermo GXM
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
