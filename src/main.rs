use vitasdk_sys as libvita;

// Definiamo la memoria per la PS Vita
#[no_mangle]
pub var_vita_memory_threshold: u32 = 0; // Usa il default del sistema

fn main() {
    println!("Ruffle PS Vita Starting...");
    // Qui andrà il loop di Ruffle
}
