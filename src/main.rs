#![no_std]
#![no_main]

use vitasdk_sys as libvita;
use core::panic::PanicInfo;

// 1. Definiamo la memoria Heap (essenziale per la PS Vita)
// Queste variabili dicono alla console quanta RAM riservare all'app.
#[no_mangle]
pub static mut _newlib_heap_size_user: u32 = 32 * 1024 * 1024; // 32MB di RAM

// 2. Configurazione della soglia di memoria
#[no_mangle]
pub static var_vita_memory_threshold: u32 = 0;

// 3. Funzione di Main (Punto di ingresso della PS Vita)
#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *mut *mut u8) -> i32 {
    // Per ora facciamo solo un test simbolico. 
    // In futuro qui inizializzeremo il motore Ruffle.
    0
}

// 4. Gestione del Panic (Cosa succede se il codice crasha)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
