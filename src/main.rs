#![no_std]
#![no_main]

mod arch;
mod debug;

use arch::x64::gdt;
use core::arch::asm;
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

    gdt::gdt_init();

    loop {
        unsafe {
            asm!("hlt", options(nomem, nostack));
        }
    }
}
