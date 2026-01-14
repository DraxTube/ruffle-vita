use vitasdk_sys as libvita;

// Definiamo correttamente la variabile per la memoria della PS Vita
#[no_mangle]
pub static var_vita_memory_threshold: u32 = 0; 

fn main() {
    println!("Ruffle PS Vita Starting...");
}
