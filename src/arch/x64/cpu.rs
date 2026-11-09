use core::arch::asm;

#[inline(always)]
pub fn read_cr2() -> u64 {
    let v: u64;
    unsafe {
        asm!("mov {}, cr2", out(reg) v, options(nomem, nostack, preserves_flags));
    }
    v
}
