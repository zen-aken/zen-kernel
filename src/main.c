#include <limine.h>

#define COM1 0x3F8

static inline void outb(unsigned short port, unsigned char value)
{
    __asm__ volatile (
        "outb %0, %1"
        :
        : "a"(value), "Nd"(port)
    );
}

void serial_write(char c)
{
    outb(COM1, c);
}

void kmain(void)
{
    serial_write('H');
    serial_write('i');

    while (1)
        __asm__ volatile ("hlt");
}
