use core::arch::asm;

use crate::{dprintln, println};

const STACK_SIZE: usize = 16 * 1024;

#[repr(C, packed)]
pub struct TSS {
    reserved0: u32,
    rsp: [u64; 3],
    reserved1: u64,
    ist: [u64; 7],
    reserved2: u64,
    reserved3: u16,
    iopb: u16,
}

#[repr(C, packed)]
pub struct TssEntry {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    limit_and_flags: u8,
    base_high: u8,
    base_upper: u32,
    reserved: u32,
}

impl TssEntry {
    pub const fn new(limit: u64, base: u64, access: u8, flags: u8) -> TssEntry {
        TssEntry {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_mid: ((base >> 16) & 0xFF) as u8,
            access: access,
            limit_and_flags: (((limit >> 16) & 0xF) as u8) | ((flags & 0xF) << 4),
            base_high: ((base >> 24) & 0xFF) as u8,
            base_upper: (base >> 32) as u32,
            reserved: 0,
        }
    }

    pub const fn null() -> TssEntry {
        TssEntry {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access: 0,
            limit_and_flags: 0,
            base_high: 0,
            base_upper: 0,
            reserved: 0,
        }
    }
}

pub static mut TSS_TABLE: TSS = TSS {
    reserved0: 0,
    rsp: [0; 3],
    reserved1: 0,
    ist: [0; 7],
    reserved2: 0,
    reserved3: 0,
    iopb: core::mem::size_of::<TSS>() as u16,
};

#[allow(dead_code)]
#[repr(align(16))]
struct Stack([u8; STACK_SIZE]);

static mut KERNEL_STACK: Stack = Stack([0; STACK_SIZE]);
static mut DF_STACK: Stack = Stack([0; STACK_SIZE]);

pub fn init() {
    dprintln!("initializing TSS.");
    dprintln!("Setting up TSS entry's.");
    unsafe {
        TSS_TABLE.rsp[0] = (&raw const KERNEL_STACK as u64) + STACK_SIZE as u64;
        TSS_TABLE.ist[0] = (&raw const DF_STACK as u64) + STACK_SIZE as u64;
        asm!("ltr {0:x}", in(reg) 0x28u16, options(nostack, preserves_flags));
        dprintln!("TSS pointer loaded.")
    }
    println!("TSS initialized.")
}
