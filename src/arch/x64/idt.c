#include "arch/x64/idt.h"
#include <constants.h>
#include <debug.h>
#include <stdint.h>

struct IDTR idtr;
struct IDT_Descriptor idt[MAX_IDT_ENTRY];

/**
 * @brief Fills an IDT gate descriptor.
 *
 * @param index             Interrupt vector number (0-255).
 * @param handler           Address of the interrupt service routine.
 * @param attributes        P (bit 7), DPL (bits 5-6), gate type (bits 0-3): 0x8E = interrupt gate,
 * 0x8F = trap gate.
 * @param segment_selector  Code segment selector (e.g. 0x08 for kernel code).
 * @param ist               IST index (0-7), 0 = no stack switch.
 */
void set_idt_entry(uint8_t index, void (*handler)(void), uint8_t attributes,
                   uint16_t segment_selector, uint8_t ist) {
    idt[index].offset_low = (uint64_t)handler & 0xFFFF;
    idt[index].offset_mid = ((uint64_t)handler >> 16) & 0xFFFF;
    idt[index].offset_high = (uint64_t)handler >> 32;
    idt[index].attributes = attributes;
    idt[index].segment_selector = segment_selector;
    idt[index].ist = ist;
    idt[index].reserved = 0;
}

void kprint_reg(const char* name, uint64_t value) {
    kprint(name);
    kprint_hex(value);
}

void isr_handler(struct Interrupt_Frame* f) {
    kprint("------------- Interrupt -------------\n");
    switch (f->vector) {
    case 0x0:
        kprint("Reason: Divide Error\n");
        break;
    case 0x1:
        kprint("Reason: Debug exception\n");
        break;
    case 0x2:
        kprint("Reason: NMI interrupt\n");
        break;
    case 0x3:
        kprint("Reason: Breakpoint\n");
        break;
    case 0x4:
        kprint("Reason: Overflow\n");
        break;
    case 0x5:
        kprint("Reason: BOUND Range Exceeded\n");
        break;
    case 0x6:
        kprint("Reason: Invalid Opcode\n");
        break;
    case 0x7:
        kprint("Reason: Device Not Available\n");
        break;
    case 0x8:
        kprint("Reason: Double Fault\n");
        break;
    case 0x9:
        kprint("Reason: Coprocessor Segment Overrun\n");
        break;
    case 0xA:
        kprint("Reason: Invalid TSS\n");
        break;
    case 0xB:
        kprint("Reason: Segment Not Present\n");
        break;
    case 0xC:
        kprint("Reason: Stack-Segment Fault\n");
        break;
    case 0xD:
        kprint("Reason: General Protection Fault\n");
        break;
    case 0xE:
        kprint("Reason: Page Fault\n");
        break;
    case 0x10:
        kprint("Reason: x87 FPU Floating-Point Error\n");
        break;
    case 0x11:
        kprint("Reason: Alignment Check\n");
        break;
    case 0x12:
        kprint("Reason: Machine Check\n");
        break;
    case 0x13:
        kprint("Reason: SIMD Floating-Point Exception\n");
        break;
    case 0x14:
        kprint("Reason: Virtualization Exception\n");
        break;
    case 0x15:
        kprint("Reason: Control Protection Exception\n");
        break;
    default:
        kprint("Reason: Reserved/Unknown\n");
        break;
    }

    kprint_reg("vector:", f->vector);
    kprint("    ");
    kprint_reg("error_code:", f->error_code);
    kprint_new_line();

    kprint_reg("rip:", f->rip);
    kprint("       ");
    kprint_reg("cs:", f->cs);
    kprint_new_line();

    kprint_reg("rflags:", f->rflags);
    kprint("    ");
    kprint_reg("rsp:", f->rsp);
    kprint_new_line();

    kprint_reg("ss:", f->ss);
    kprint("        ");
    kprint_reg("rax:", f->rax);
    kprint_new_line();

    kprint_reg("rbx:", f->rbx);
    kprint("       ");
    kprint_reg("rcx:", f->rcx);
    kprint_new_line();

    kprint_reg("rdx:", f->rdx);
    kprint("       ");
    kprint_reg("rsi:", f->rsi);
    kprint_new_line();

    kprint_reg("rdi:", f->rdi);
    kprint("       ");
    kprint_reg("rbp:", f->rbp);
    kprint_new_line();
    kprint_new_line();

    kprint_reg("r8:", f->r8);
    kprint("        ");
    kprint_reg("r9:", f->r9);
    kprint_new_line();

    kprint_reg("r10:", f->r10);
    kprint("       ");
    kprint_reg("r11:", f->r11);
    kprint_new_line();

    kprint_reg("r12:", f->r12);
    kprint("       ");
    kprint_reg("r13:", f->r13);
    kprint_new_line();

    kprint_reg("r14:", f->r14);
    kprint("       ");
    kprint_reg("r15:", f->r15);
    kprint_new_line();

    uint64_t cr2;
    if (f->vector == 0xE) {
        asm volatile("mov %%cr2, %0" : "=r"(cr2));
        kprint_reg("cr2: ", cr2);
        kprint_new_line();
    }

    kprint("-------------------------------------\n");

    while (1) {
        __asm__ volatile("hlt");
    }
}

extern void (*isr_table[32])(void);

void init_idt() {
#if DEBUG == 1
    kprint("Setting IDT entries...\n");
#endif
    for (size_t i = 0; i < 32; i++) {
        set_idt_entry(i, isr_table[i], KERNEL_ATTRIBUTES, KERNEL_SEGMENT, 0);
    }

#if DEBUG == 1
    kprint("Setting IDTR pointer...\n");
#endif
    idtr.offset = (uint64_t)&idt;
    idtr.size = sizeof(idt);

#if DEBUG == 1
    kprint("Loading IDTR pointer...\n");
#endif
    __asm__ volatile("lidt %0" : : "m"(idtr) : "memory");

    kprint("IDT initialized\n");
}
