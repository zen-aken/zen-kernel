#include "limine_headers.h"
#include <constants.h>

#ifdef __x86_64__
#include "arch/x64/gdt.h"
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

    while (1)
        hcf();
}
