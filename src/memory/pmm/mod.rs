use crate::println;

mod bitmap;

pub fn init() {
    println!("[PMM] Initializing...");
    bitmap::init();
    println!("[PMM] OK");
}
