#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod arch;
mod debug;
mod drivers;

use crate::{arch::x64::idt, debug::Debug};
use arch::x64::gdt;
use core::{arch::asm, panic::PanicInfo};
use limine::{
    BaseRevision, RequestsEndMarker, RequestsStartMarker,
    request::{HhdmRequest, MemmapRequest},
};

#[used]
#[unsafe(link_section = ".limine_requests_start")]
static START: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".limine_requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".limine_requests")]
static HHDM: HhdmRequest = HhdmRequest::new();

#[used]
#[unsafe(link_section = ".limine_requests")]
static MEMMAP: MemmapRequest = MemmapRequest::new();

#[used]
#[unsafe(link_section = ".limine_requests_end")]
static END: RequestsEndMarker = RequestsEndMarker::new();

#[unsafe(no_mangle)]
extern "C" fn kernel() -> ! {
    assert!(BASE_REVISION.is_supported());

    gdt::init();
    idt::init();

    loop {
        unsafe {
            asm!("hlt", options(nomem, nostack));
        }
    }
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info.message());
    loop {
        unsafe { asm!("cli; hlt") }
    }
}
