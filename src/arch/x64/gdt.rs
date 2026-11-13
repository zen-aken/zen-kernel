use crate::arch::x64::tss::{TSS, TSS_TABLE, TssEntry};
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
const TSS_ACCESS: u8 = 0b10001001; // 0x89

#[repr(C, packed)]
struct GDT {
    entries: [GDT_Entry; GDT_ENTRY_COUNT],
    tss: TssEntry,
}

#[repr(C, packed)]
struct GDTPtr {
    size: u16,
    offset: u64,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct GDT_Entry {
    limit: u16,
    base_low: u16,
    base_mid: u8,
    access_byte: u8,
    limit_and_flags: u8,
    base_high: u8,
}

impl GDT_Entry {
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
    const fn new(limit: u16, base: u32, access_byte: u8, limit_and_flags: u8) -> GDT_Entry {
        GDT_Entry {
            limit,
            base_low: (base & 0xFFFF) as u16,
            base_mid: ((base >> 16) & 0xFF) as u8,
            access_byte,
            limit_and_flags,
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }

    /// Creates a null GDT entry (all fields zero).
    ///
    /// # Returns
    /// A `GDT_Entry` with every field set to 0. Used for the mandatory
    /// null descriptor (index 0) and to initialize unused entries.
    const fn null() -> GDT_Entry {
        GDT_Entry {
            limit: 0,
            base_low: 0,
            base_mid: 0,
            access_byte: 0,
            limit_and_flags: 0,
            base_high: 0,
        }
    }
}

static mut GDT_TABLE: GDT = GDT {
    entries: [GDT_Entry::null(); GDT_ENTRY_COUNT],
    tss: TssEntry::null(),
};

fn setup_gdt_entry() {
    unsafe {
        GDT_TABLE.entries[0] = GDT_Entry::new(0, 0, 0, 0); // Null
        GDT_TABLE.entries[1] = GDT_Entry::new(0xFFFF, 0, KERNEL_CODE, KERNEL_CODE_LIMIT_AND_FLAGS);
        GDT_TABLE.entries[2] = GDT_Entry::new(0xFFFF, 0, KERNEL_DATA, KERNEL_DATA_LIMIT_AND_FLAGS);
        GDT_TABLE.entries[3] = GDT_Entry::new(0xFFFF, 0, USER_CODE, USER_CODE_LIMIT_AND_FLAGS);
        GDT_TABLE.entries[4] = GDT_Entry::new(0xFFFF, 0, USER_DATA, USER_DATA_LIMIT_AND_FLAGS);
        GDT_TABLE.tss = TssEntry::new(
            (core::mem::size_of::<TSS>() - 1) as u64,
            &raw const TSS_TABLE as u64,
            TSS_ACCESS,
            0,
        );
    }
}

fn load_gdtr() {
    unsafe {
        let gdtr = GDTPtr {
            size: (core::mem::size_of::<GDT>() - 1) as u16,
            offset: &raw const GDT_TABLE as u64,
        };
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
    println!("[GDT] Initializing...");
    setup_gdt_entry();
    dprintln!("[GDT] Setting up GDT entry's");
    load_gdtr();
    dprintln!("[GDT] GDTR loaded");
    refresh_segments();
    dprintln!("[GDT] Segments refreshed");
    println!("[GDT] OK");
}
