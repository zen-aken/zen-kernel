use crate::{arch::x64::cpu::read_cr2, println};
use core::arch::asm;

#[repr(C)]
pub struct InterruptStackFrame {
    ip: usize,
    cs: usize,
    flags: usize,
    sp: usize,
    ss: usize,
}

pub extern "x86-interrupt" fn page_fault(stack_frame: InterruptStackFrame, error: u64) {
    let cr2 = read_cr2();
    println!("---------- Page Fault ----------");
    println!("Fault address: {:#x}", cr2);
    println!("Error code:    {:#x}", error);
    println!(
        "  present={} write={} user={} rsvd={} fetch={}",
        error & 1 != 0,
        error & 2 != 0,
        error & 4 != 0,
        error & 8 != 0,
        error & 16 != 0
    );
    println!("RIP:   {:#x}", stack_frame.ip);
    println!("CS:    {:#x}", stack_frame.cs);
    println!("FLAGS: {:#x}", stack_frame.flags);
    println!("SP:    {:#x}", stack_frame.sp);
    println!("SS:    {:#x}", stack_frame.ss);

    loop {
        unsafe { asm!("cli; hlt", options(nomem, nostack)) }
    }
}

pub extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, error: u64) -> ! {
    println!("---------- Double Fault ----------");
    println!("DOUBLE FAULT (error: {})", error);
    println!("RIP:   {:#x}", stack_frame.ip);
    println!("CS:    {:#x}", stack_frame.cs);
    println!("FLAGS: {:#x}", stack_frame.flags);
    println!("SP:    {:#x}", stack_frame.sp);
    println!("SS:    {:#x}", stack_frame.ss);

    loop {
        unsafe {
            core::arch::asm!("cli; hlt", options(nomem, nostack));
        }
    }
}
