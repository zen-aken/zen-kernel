use core::arch::asm;

use crate::{dprintln, println};

const GDT_ENTRY_COUNT: usize = 5;

const KERNEL_CODE: u8 = 0b1001_1011; // 0x9B
const KERNEL_DATA: u8 = 0b1001_0011; // 0x93
const USER_CODE: u8 = 0b1111_1011; // 0xFB
const USER_DATA: u8 = 0b1111_0011; //0xF3
const KERNEL_CODE_LIMIT_AND_FLAGS: u8 = 0b1010_1111; // 0xAF
const KERNEL_DATA_LIMIT_AND_FLAGS: u8 = 0b1100_1111; //0xCF
const USER_CODE_LIMIT_AND_FLAGS: u8 = 0b1010_1111; // 0xAF
const USER_DATA_LIMIT_AND_FLAGS: u8 = 0b1100_1111; // 0xCF

#[repr(C, packed)]
struct GDTPtr {
    size: u16,
    offset: u64,
}

#[repr(C, packed)]
struct GDTEntry {
    limit: u16,
    base_low: u16,
    base_mid: u8,
    access_byte: u8,
    limit_and_flags: u8,
    base_high: u8,
}

impl GDTEntry {
    /// Creates a GDT entry (descriptor).
    ///
    /// # Parameters
    /// * `limit` - Segment limit (lower 16 bits)
    /// * `base` - Segment base address (32 bits)
    /// * `access_byte` - Access byte (Present, DPL, S, Type)
    /// * `limit_and_flags` - Upper nibble: flags (G, D/B, L, AVL), lower nibble: limit[19:16]
    ///
    /// # Returns
    /// The constructed `GDTEntry`.
    const fn new(limit: u16, base: u32, access_byte: u8, limit_and_flags: u8) -> GDTEntry {
        GDTEntry {
            limit,
            base_low: (base & 0xFFFF) as u16,
            base_mid: ((base >> 16) & 0xFF) as u8,
            access_byte,
            limit_and_flags,
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}

static GDT: [GDTEntry; GDT_ENTRY_COUNT] = [
    GDTEntry::new(0xFFFF, 0, 0, 0), // Null
    GDTEntry::new(0xFFFF, 0, KERNEL_CODE, KERNEL_CODE_LIMIT_AND_FLAGS), // Kernel code
    GDTEntry::new(0xFFFF, 0, KERNEL_DATA, KERNEL_DATA_LIMIT_AND_FLAGS), // Kernel data
    GDTEntry::new(0xFFFF, 0, USER_CODE, USER_CODE_LIMIT_AND_FLAGS), // User code
    GDTEntry::new(0xFFFF, 0, USER_DATA, USER_DATA_LIMIT_AND_FLAGS), // User data
];

fn load_gdtr() {
    let gdtr = GDTPtr {
        size: (core::mem::size_of_val(&GDT) - 1) as u16,
        offset: GDT.as_ptr() as u64,
    };

    unsafe {
        // Load gdtr
        asm!(
            "lgdt [{}]",
            in(reg) &gdtr,
            options(readonly, nostack, preserves_flags)
        );
    }
}

fn refresh_segments() {
    unsafe {
        // Reload CS
        asm!(
            "push 0x08",
            "lea {tmp}, [rip + 2f]",
            "push {tmp}",
            "retfq",
            "2:",
            tmp = out(reg) _,
            options(preserves_flags)
        );

        // Reload data segments
        asm!(
            "mov ax, 0x10",
            "mov ds, ax",
            "mov es, ax",
            "mov ss, ax",
            "xor eax, eax",
            "mov fs, ax",
            "mov gs, ax",
            out("ax") _,
            options(nostack, preserves_flags)
        );
    }
}

pub fn init() {
    dprintln!("Initializing GDT.");
    load_gdtr();
    dprintln!("GDTR loaded.");
    refresh_segments();
    dprintln!("Segments refreshed.");
    println!("GDT Initialized.");
}
