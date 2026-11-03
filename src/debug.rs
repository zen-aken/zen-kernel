use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;

struct Dbg;

impl Write for Dbg {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            unsafe {
                asm!("out dx, al", in("dx") 0xE9u16, in("al") b, options(nomem, nostack));
            }
        }
        Ok(())
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe { asm!("cli", options(nomem, nostack)) }
    let _ = writeln!(Dbg, "\nPANIC: {}", info);
    loop {
        unsafe { asm!("hlt", options(nomem, nostack)) }
    }
}
