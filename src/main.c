#include <acpi/xsdt.h>
#include <constants.h>
#include <stdint.h>

#if ARCH == __x64__
    #include "arch/x64/gdt.h"
    #include "arch/x64/idt.h"
    #include "arch/x64/pic.h"
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
    init_pic();

    while (1)
        hcf();
}
