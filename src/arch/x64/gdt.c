#include "arch/x64/gdt.h"
#include <debug.h>

struct GDT_Descriptor gdt[MAX_GDTR_ENTRY_COUNT];
struct GDTR gdtr;

void setGDTEntry(size_t index, uint8_t access, uint8_t limit_flags) {
    gdt[index].limit_low = 0;
    gdt[index].base_low = 0;
    gdt[index].base_mid = 0;
    gdt[index].access = access;
    gdt[index].limit_high_flags = limit_flags;
    gdt[index].base_high = 0;
}

static inline void gdt_load(struct GDTR* gdtr) {
    __asm__ volatile("lgdt %0" : : "m"(*gdtr) : "memory");
}

static inline void gdt_reload_segments(uint16_t code, uint16_t data) {
    __asm__ volatile("pushq %0\n"
                     "leaq 1f(%%rip), %%rax\n"
                     "pushq %%rax\n"
                     "lretq\n"
                     "1:\n"
                     "movw %1, %%ds\n"
                     "movw %1, %%es\n"
                     "movw %1, %%ss\n"
                     "xorw %%ax, %%ax\n"
                     "movw %%ax, %%fs\n"
                     "movw %%ax, %%gs\n"
                     :
                     : "r"((uint64_t)code), "r"(data)
                     : "rax", "memory");
}

void init_gdt() {
#if DEBUG == 1
    kprint("setting GDT entries...\n");
#endif
    setGDTEntry(0, 0, 0);
    setGDTEntry(1, 0x9A, 0x20);
    setGDTEntry(2, 0x92, 0x00);
    setGDTEntry(3, 0xFA, 0x20);
    setGDTEntry(4, 0xF2, 0x00);

    gdtr.size = sizeof(gdt) - 1;
    gdtr.offset = (uint64_t)&gdt;

#if DEBUG == 1
    kprint("loading GDTR...\n");
#endif
    gdt_load(&gdtr);

#if DEBUG == 1
    kprint("Reloading segments...\n");
#endif
    gdt_reload_segments(0x08, 0x10);

    kprint("GDT initialized\n");
}
