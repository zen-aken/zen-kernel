#ifndef GDT_H
#define GDT_H

#include <types.h>
#include <debug.h>
#include <constants.h>

#define MAX_GDTR_ENTRY_COUNT 8192

/**
 * @brief GDTR register structure (loaded with `lgdt`).
 */
struct GDTR {
    uint16_t size;
    uint64_t offset;
} __attribute__((packed));

/**
 * @brief 8-byte GDT segment descriptor.
 *
 * In 64-bit mode, base and limit are ignored for code/data segments.
 */
struct GDT_Descriptor {
    uint16_t limit_low;
    uint16_t base_low;
    uint8_t  base_mid;
    uint8_t  access;
    uint8_t  limit_high_flags;
    uint8_t  base_high;
} __attribute__((packed));


void init_gdt();

#endif
