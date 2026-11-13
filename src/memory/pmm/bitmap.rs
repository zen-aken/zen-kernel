use limine::memmap;

use crate::{HHDM, MEMMAP, dprintln, println};

const FRAME_SIZE: usize = 4096; // 4Kib

static mut MEMORY_BITMAP: *mut u64 = core::ptr::null_mut();
static mut BITMAP_LEN: usize = 0;
static mut FRAME_COUNT: usize = 0;

/**
 * @brief Checks whether a physical frame is marked as allocated.
 * @param addr Physical address of the frame (frame-aligned).
 * @return true if the frame is allocated, false if free.
 */
#[allow(dead_code)]
fn is_allocated(addr: u64) -> bool {
    let frame = addr as usize / FRAME_SIZE;
    unsafe {
        let bit_group = *MEMORY_BITMAP.add(frame / 64);
        (bit_group >> (frame % 64)) & 1 == 1
    }
}

/**
 * @brief Marks a physical frame as allocated in the bitmap.
 * @param addr Physical address of the frame (frame-aligned).
 */
#[allow(dead_code)]
fn set_allocated(addr: u64) {
    let frame = addr as usize / FRAME_SIZE;
    unsafe {
        let bitmap = MEMORY_BITMAP.add(frame / 64);
        *bitmap |= 1u64 << (frame % 64);
    }
}

/**
 * @brief Marks a physical frame as free in the bitmap.
 * @param addr Physical address of the frame (frame-aligned).
 */
#[allow(dead_code)]
fn set_deallocated(addr: u64) {
    let frame = addr as usize / FRAME_SIZE;
    unsafe {
        let bitmap = MEMORY_BITMAP.add(frame / 64);
        *bitmap &= !(1u64 << (frame % 64));
    }
}

/**
 * @brief Allocates one free physical frame (first-fit scan).
 * @return Physical address of the allocated frame, or None if out of memory.
 */
#[allow(dead_code)]
pub fn alloc_frame() -> Option<u64> {
    unsafe {
        for i in 0..FRAME_COUNT {
            let addr = (i * FRAME_SIZE) as u64;
            if !is_allocated(addr) {
                set_allocated(addr);
                return Some(addr);
            }
        }
        None
    }
}

/**
 * @brief Frees a previously allocated physical frame.
 * @param addr Physical address returned by alloc_frame().
 * @warning Address must be below FRAME_COUNT * FRAME_SIZE, otherwise the
 *          write goes outside the bitmap.
 */
#[allow(dead_code)]
pub fn dealloc_frame(addr: u64) {
    set_deallocated(addr);
}

pub fn init() {
    dprintln!("------------------------------");
    let memory_map = MEMMAP.response().unwrap();

    // Print usable memory spaces
    for entry in memory_map.entries() {
        if entry.type_ != memmap::MEMMAP_USABLE {
            continue;
        }
        dprintln!("[PMM] Entry base: 0x{:x}", entry.base);
        dprintln!("[PMM] Entry length: {}", entry.length);
        dprintln!("[PMM] Entry type: {}", entry.type_);
        dprintln!("--------------------");
    }

    // Get highest usable memory address
    let mut highest_end: u64 = 0;
    for entry in memory_map.entries() {
        if entry.type_ != memmap::MEMMAP_USABLE {
            continue;
        }
        highest_end = highest_end.max(entry.base + entry.length);
    }

    // Find out frame count and bitmap length
    let frame_count = (highest_end / FRAME_SIZE as u64) as usize;
    let bitmap_len = (frame_count + 63) / 64;
    dprintln!("[PMM] Frame count: {}", frame_count);
    dprintln!("[PMM] Bitmap length: {}", bitmap_len);

    unsafe {
        FRAME_COUNT = frame_count;
        BITMAP_LEN = bitmap_len;
    }

    // Set bitmap pointer address and fill whole bitmap as allocated
    let mut bitmap_phys = 0u64;
    let hhdm = HHDM.response().unwrap();
    for entry in memory_map.entries() {
        if entry.type_ != memmap::MEMMAP_USABLE {
            continue;
        }

        if entry.base == 0 {
            continue;
        }

        if (entry.length as usize) < bitmap_len * 8 {
            continue;
        }

        unsafe {
            MEMORY_BITMAP = (entry.base + hhdm.offset) as *mut u64;
            core::ptr::write_bytes(MEMORY_BITMAP, 0xFF, bitmap_len * 8);
        }
        bitmap_phys = entry.base;
        break;
    }

    unsafe {
        if MEMORY_BITMAP.is_null() {
            panic!("[PMM] No usable region to fit bitmap")
        }
        let _bitmap_ptr = MEMORY_BITMAP;
        dprintln!("[PMM] Bitmap address: {:p}", _bitmap_ptr);
    }

    // Deallocate usable memory spaces
    dprintln!("[PMM] Deallocating usable memory space");
    for entry in memory_map.entries() {
        if entry.type_ != memmap::MEMMAP_USABLE {
            continue;
        }
        let start = (entry.base + 4096 - 1) / FRAME_SIZE as u64;
        let end = (entry.base + entry.length) / FRAME_SIZE as u64;
        for i in start..end {
            set_deallocated(i * FRAME_SIZE as u64);
        }
    }

    // Set bitmap address as allocated
    dprintln!("[PMM] Setting bitmap address as allocated");
    let bitmap_start = bitmap_phys / FRAME_SIZE as u64;
    let bitmap_end =
        (bitmap_phys + (bitmap_len * 8) as u64 + FRAME_SIZE as u64 - 1) / FRAME_SIZE as u64;
    for f in bitmap_start..bitmap_end {
        set_allocated(f * FRAME_SIZE as u64);
    }
    dprintln!("------------------------------");
}
