#include "limine_headers.h"
#include <constants.h>

#include <debug.h>
#include <definition.h>

void hcf() {
#if ARCH == __x64__
    asm volatile("hlt");
#elif ARCH == __aarch64__
    asm volatile("wfe");
#endif
}

void kmain(void) {
    kprint("merhaba\n");
    uint64_t my_hex = 0xDEADBEEFCAFEBABE;
    kprint_hex(my_hex);
    kprint_new_line();
    long long int test_number = 8273643;
    kprint_int(test_number);
    while (1)
        hcf();
}
