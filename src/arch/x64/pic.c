#include "arch/x64/pic.h"
#include "arch/x64/io.h"

#include <constants.h>
#include <debug.h>

void PIC_disable(void) {
    outb(MASTER_DATA, 0xff);
    outb(SLAVE_DATA, 0xff);
}

void init_pic() {
#if DEBUG == 1
    kprint("Setting up PIC...\n");
#endif
    PIC_disable();
    kprint("PIC disabled for LAPIC\n");
}
