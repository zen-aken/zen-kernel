#ifndef IDT_H
#define IDT_H

#define MAX_IDT_ENTRY 256
#define KERNEL_SEGMENT 0x08
#define KERNEL_ATTRIBUTES 0x8E

#include <debug.h>
#include <types.h>

/**
 * @brief IDTR register structure (loaded with `lidt`).
 */
struct IDTR {
    uint16_t size;
    uint64_t offset;
} __attribute__((packed));

/**
 * @brief 16-byte IDT gate descriptor (64-bit mode).
 */
struct IDT_Descriptor {
    uint16_t offset_low;
    uint16_t segment_selector;
    uint8_t ist;
    uint8_t attributes;
    uint16_t offset_mid;
    uint32_t offset_high;
    uint32_t reserved;
} __attribute__((packed));

struct Interrupt_Frame {
    uint64_t r15, r14, r13, r12, r11, r10, r9, r8;
    uint64_t rbp, rdi, rsi, rdx, rcx, rbx, rax;
    uint64_t vector, error_code;
    uint64_t rip, cs, rflags, rsp, ss;
};

void init_idt();

#endif
