#![no_std]
#![no_main]

mod debug;

use core::arch::asm;

#[unsafe(no_mangle)]
extern "C" fn kernel() -> ! {
    loop {
        unsafe {
            asm!("hlt", options(nomem, nostack));
        }
    }
}
