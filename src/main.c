#include "limine_headers.h"
#include <constants.h>

#ifdef __x86_64__
#include "arch/x64/gdt.h"
#include "arch/x64/idt.h"
#endif

void hcf() {
#if ARCH == __x64__
    asm volatile("hlt");
#elif ARCH == __aarch64__
    asm volatile("wfe");
#endif
}

void kmain(void) {
    init_gdt();
    init_idt();

    while (1)
        hcf();
}
