use crate::{arch::x64::isr::page_fault, dprintln, println};
use core::arch::asm;

const IDT_ENTRY_COUNT: usize = 256;
const KERNEL_CODE_SEGMENT: u16 = 0x08;
const KERNEL_INTERRUPT: u8 = 0x8E;
// const KERNEL_TRAP: u8 = 0x8F;
// const USER_INTERRUPT: u8 = 0xEE;

#[repr(C, packed)]
struct IDTPtr {
    size: u16,
    offset: u64,
}

#[repr(C, packed)]
struct IDTEntry {
    offset_low: u16,
    segment_selector: u16,
    ist: u8,
    attributes: u8,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

impl IDTEntry {
    const fn new(address: u64, segment_selector: u16, ist: u8, attributes: u8) -> IDTEntry {
        IDTEntry {
            offset_low: (address & 0xFFFF) as u16,
            segment_selector: segment_selector,
            ist: ist,
            attributes: attributes,
            offset_mid: ((address >> 16) & 0xFFFF) as u16,
            offset_high: (address >> 32) as u32,
            reserved: 0,
        }
    }

    const MISSING: IDTEntry = IDTEntry {
        offset_low: 0,
        offset_mid: 0,
        offset_high: 0,
        attributes: 0,
        ist: 0,
        segment_selector: 0,
        reserved: 0,
    };
}

static mut IDT: [IDTEntry; IDT_ENTRY_COUNT] = [IDTEntry::MISSING; IDT_ENTRY_COUNT];

fn load_idt() {
    unsafe {
        let idtr = IDTPtr {
            size: (core::mem::size_of::<IDTEntry>() * IDT_ENTRY_COUNT - 1) as u16,
            offset: &raw const IDT as u64,
        };
        asm!("lidt [{}]", in(reg) &idtr, options(readonly, nostack, preserves_flags));
    }
}

pub fn init() {
    dprintln!("Initializing IDT.");
    unsafe {
        dprintln!("Setting IDT entry.");
        IDT[14] = IDTEntry::new(
            page_fault as *const () as usize as u64,
            KERNEL_CODE_SEGMENT,
            0,
            KERNEL_INTERRUPT,
        );
    }
    dprintln!("Loading IDTR.");
    load_idt();
    println!("IDT initialized.");
}
